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

    // Increment counter on mount (for site visits) or when component is used (for game plays)
    {
        let count = count.clone();
        let counter_type = props.counter_type.clone();
        
        use_effect_with((), move |_| {
            // Only auto-increment site visits on mount
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
    
    match Request::get(&url).send().await {
        Ok(response) => {
            if response.status() == 200 {
                if let Ok(text) = response.text().await {
                    if let Ok(firestore_response) = serde_json::from_str::<serde_json::Value>(&text) {
                        if let Some(count_str) = firestore_response["fields"]["count"]["integerValue"].as_str() {
                            return count_str.parse().unwrap_or(0);
                        }
                    }
                }
            }
        }
        Err(_) => {}
    }
    
    // If document doesn't exist or fetch fails, return 0
    0
}

async fn increment_counter(counter_type: &CounterType) -> u32 {
    // First, get current count
    let current_count = fetch_counter(counter_type).await;
    let new_count = current_count + 1;
    
    // Create Firestore document format
    let document = serde_json::json!({
        "fields": {
            "count": {
                "integerValue": new_count.to_string()
            },
            "last_updated": {
                "timestampValue": js_sys::Date::new_0().to_iso_string().as_string().unwrap_or_else(|| "2025-08-06T00:00:00.000Z".to_string())
            }
        }
    });

    let url = format!("{}?documentId={}", FIRESTORE_COUNTERS_URL, counter_type.document_id());
    
    match Request::post(&url)
        .header("Content-Type", "application/json")
        .body(document.to_string())
    {
        Ok(request) => {
            match request.send().await {
                Ok(response) => {
                    if response.status() == 200 {
                        return new_count;
                    }
                }
                Err(_) => {}
            }
        }
        Err(_) => {}
    }
    
    // If update fails, return current count (no increment)
    current_count
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
