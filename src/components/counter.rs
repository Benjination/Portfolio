use yew::prelude::*;
use gloo_net::http::Request;
use serde_json;

const FIRESTORE_PROJECT_ID: &str = "portfolio-7148b";
const FIRESTORE_COUNTERS_URL: &str = "https://firestore.googleapis.com/v1/projects/portfolio-7148b/databases/(default)/documents/counters";

#[derive(Clone, PartialEq, Properties)]
pub struct CounterProps {
    pub counter_type: CounterType,
    pub label: String,
}

#[derive(Clone, PartialEq)]
pub enum CounterType {
    SiteVisits,
    GamePlays,
}

impl CounterType {
    fn document_id(&self) -> &'static str {
        match self {
            CounterType::SiteVisits => "site_visits",
            CounterType::GamePlays => "game_plays",
        }
    }
}

#[function_component(Counter)]
pub fn counter(props: &CounterProps) -> Html {
    let count = use_state(|| 0u32);
    let loading = use_state(|| true);

    // Fetch current count on mount
    {
        let count = count.clone();
        let loading = loading.clone();
        let counter_type = props.counter_type.clone();
        
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let current_count = fetch_counter(&counter_type).await;
                count.set(current_count);
                loading.set(false);
            });
            || ()
        });
    }

    // Increment counter on every mount/page load for site visits
    {
        let count = count.clone();
        let counter_type = props.counter_type.clone();
        
        use_effect_with((), move |_| {
            // Auto-increment site visits on every page load
            if matches!(counter_type, CounterType::SiteVisits) {
                wasm_bindgen_futures::spawn_local(async move {
                    let new_count = increment_counter(&counter_type).await;
                    count.set(new_count);
                });
            }
            || ()
        });
    }

    let count_value = *count;
    let is_loading = *loading;

    html! {
        <div class="counter-display">
            <div class="counter-label">{ &props.label }</div>
            <div class="counter-value">
                if is_loading {
                    <span class="loading">{ "..." }</span>
                } else {
                    <span class="number">{ format!("{:06}", count_value) }</span>
                }
            </div>
        </div>
    }
}

async fn fetch_counter(counter_type: &CounterType) -> u32 {
    let url = format!("{}/{}", FIRESTORE_COUNTERS_URL, counter_type.document_id());
    
    // Add some debug logging
    web_sys::console::log_1(&format!("Fetching counter from: {}", url).into());
    
    match Request::get(&url).send().await {
        Ok(response) => {
            web_sys::console::log_1(&format!("Response status: {}", response.status()).into());
            
            if response.status() == 200 {
                if let Ok(text) = response.text().await {
                    web_sys::console::log_1(&format!("Response text: {}", text).into());
                    
                    if let Ok(firestore_response) = serde_json::from_str::<serde_json::Value>(&text) {
                        if let Some(count_str) = firestore_response["fields"]["count"]["integerValue"].as_str() {
                            let count = count_str.parse().unwrap_or(0);
                            web_sys::console::log_1(&format!("Parsed count: {}", count).into());
                            return count;
                        } else {
                            web_sys::console::log_1(&"Count field not found in response".into());
                        }
                    } else {
                        web_sys::console::log_1(&"Failed to parse JSON response".into());
                    }
                }
            } else if response.status() == 404 {
                web_sys::console::log_1(&"Document not found, will create new one".into());
            }
        }
        Err(e) => {
            web_sys::console::log_1(&format!("Fetch error: {:?}", e).into());
        }
    }
    
    // If document doesn't exist or fetch fails, return 0
    0
}

async fn increment_counter(counter_type: &CounterType) -> u32 {
    let doc_id = counter_type.document_id();
    
    web_sys::console::log_1(&format!("Incrementing counter: {}", doc_id).into());
    
    // Try to increment using Firestore's atomic increment
    // First attempt: Use PATCH to update existing document
    let patch_url = format!("{}/{}", FIRESTORE_COUNTERS_URL, doc_id);
    
    // Create increment operation using Firestore transforms
    let transform_document = serde_json::json!({
        "writes": [{
            "transform": {
                "document": format!("projects/{}/databases/(default)/documents/counters/{}", FIRESTORE_PROJECT_ID, doc_id),
                "fieldTransforms": [{
                    "fieldPath": "count",
                    "increment": {
                        "integerValue": "1"
                    }
                }]
            }
        }]
    });

    let commit_url = format!("https://firestore.googleapis.com/v1/projects/{}/databases/(default)/documents:commit", FIRESTORE_PROJECT_ID);
    
    web_sys::console::log_1(&format!("Trying atomic increment at: {}", commit_url).into());
    
    // Try atomic increment first
    match Request::post(&commit_url)
        .header("Content-Type", "application/json")
        .body(transform_document.to_string())
    {
        Ok(request) => {
            match request.send().await {
                Ok(response) => {
                    web_sys::console::log_1(&format!("Atomic increment response: {}", response.status()).into());
                    
                    if response.status() == 200 {
                        // Fetch the updated count
                        let updated_count = fetch_counter(counter_type).await;
                        if updated_count > 0 {
                            web_sys::console::log_1(&format!("Atomic increment successful: {}", updated_count).into());
                            return updated_count;
                        }
                    }
                }
                Err(e) => {
                    web_sys::console::log_1(&format!("Atomic increment request error: {:?}", e).into());
                }
            }
        }
        Err(e) => {
            web_sys::console::log_1(&format!("Atomic increment build error: {:?}", e).into());
        }
    }
    
    // If atomic increment fails, try to create/update document manually
    // This handles the case where the document doesn't exist yet
    web_sys::console::log_1(&"Atomic increment failed, trying manual update".into());
    
    let current_count = fetch_counter(counter_type).await;
    let new_count = current_count + 1;
    
    web_sys::console::log_1(&format!("Manual increment: {} -> {}", current_count, new_count).into());
    
    let document = serde_json::json!({
        "fields": {
            "count": {
                "integerValue": new_count.to_string()
            },
            "last_updated": {
                "timestampValue": js_sys::Date::new_0().to_iso_string().as_string().unwrap_or_else(|| "2025-08-16T00:00:00.000Z".to_string())
            }
        }
    });

    // Use PATCH to update existing document or PUT to create new one
    let update_url = format!("{}/{}", FIRESTORE_COUNTERS_URL, doc_id);
    
    web_sys::console::log_1(&format!("Trying PATCH update at: {}", update_url).into());
    
    match Request::patch(&update_url)
        .header("Content-Type", "application/json")
        .body(document.to_string())
    {
        Ok(request) => {
            match request.send().await {
                Ok(response) => {
                    web_sys::console::log_1(&format!("PATCH response: {}", response.status()).into());
                    
                    if response.status() == 200 {
                        web_sys::console::log_1(&format!("PATCH successful: {}", new_count).into());
                        return new_count;
                    }
                }
                Err(e) => {
                    web_sys::console::log_1(&format!("PATCH request error: {:?}", e).into());
                }
            }
        }
        Err(e) => {
            web_sys::console::log_1(&format!("PATCH build error: {:?}", e).into());
        }
    }
    
    // If PATCH fails, try POST to create document
    let create_url = format!("{}?documentId={}", FIRESTORE_COUNTERS_URL, doc_id);
    
    web_sys::console::log_1(&format!("Trying POST create at: {}", create_url).into());
    
    match Request::post(&create_url)
        .header("Content-Type", "application/json")
        .body(document.to_string())
    {
        Ok(request) => {
            match request.send().await {
                Ok(response) => {
                    web_sys::console::log_1(&format!("POST response: {}", response.status()).into());
                    
                    if response.status() == 200 {
                        web_sys::console::log_1(&format!("POST successful: {}", new_count).into());
                        return new_count;
                    }
                }
                Err(e) => {
                    web_sys::console::log_1(&format!("POST request error: {:?}", e).into());
                }
            }
        }
        Err(e) => {
            web_sys::console::log_1(&format!("POST build error: {:?}", e).into());
        }
    }
    
    // If all attempts fail, return current count + 1 as best effort
    web_sys::console::log_1(&format!("All increment attempts failed, returning: {}", current_count + 1).into());
    current_count + 1
}

// Hook to increment game plays counter
#[hook]
pub fn use_game_counter() -> Callback<()> {
    Callback::from(|_| {
        wasm_bindgen_futures::spawn_local(async {
            increment_counter(&CounterType::GamePlays).await;
        });
    })
}
