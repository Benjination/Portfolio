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
    
    // Simplified approach: just get current count and increment it
    let current_count = fetch_counter(counter_type).await;
    let new_count = current_count + 1;
    
    web_sys::console::log_1(&format!("Current count: {}, incrementing to: {}", current_count, new_count).into());
    
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

    // Try PATCH first (update existing document)
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
                    } else {
                        // Log the response text for debugging
                        if let Ok(response_text) = response.text().await {
                            web_sys::console::log_1(&format!("PATCH failed with response: {}", response_text).into());
                        }
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
                    
                    if response.status() == 200 || response.status() == 201 {
                        web_sys::console::log_1(&format!("POST successful: {}", new_count).into());
                        return new_count;
                    } else {
                        // Log the response text for debugging
                        if let Ok(response_text) = response.text().await {
                            web_sys::console::log_1(&format!("POST failed with response: {}", response_text).into());
                        }
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
    
    // If all attempts fail, return the incremented value anyway for local display
    web_sys::console::log_1(&format!("Firebase update failed, returning local increment: {}", new_count).into());
    new_count
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
