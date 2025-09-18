use yew::prelude::*;
use yew_router::prelude::*;
use wasm_bindgen_futures;
use gloo::timers::future::TimeoutFuture;

mod components;

use components::{Header, About, Skills, Projects, Contact, Terminal, MatrixRain, Snake, Counter, CounterType, Streaming, BlogList, BlogPostComponent};

#[derive(Clone, Routable, PartialEq, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/blog/:id")]
    BlogPost { id: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(RedirectHandler)]
fn redirect_handler() -> Html {
    let navigator = use_navigator().unwrap();
    
    use_effect_with((), move |_| {
        // Use a more reliable timing approach
        let navigator_clone = navigator.clone();
        wasm_bindgen_futures::spawn_local(async move {
            // Wait longer for everything to initialize
            gloo::timers::future::TimeoutFuture::new(500).await;
            
            if let Some(window) = web_sys::window() {
                // First check for hash-based redirect (from static pages)
                if let Ok(hash) = window.location().hash() {
                    if hash.starts_with("#redirect-blog=") {
                        let blog_id = hash.trim_start_matches("#redirect-blog=");
                        let decoded_id = web_sys::js_sys::decode_uri_component(blog_id).ok()
                            .and_then(|v| v.as_string())
                            .unwrap_or_else(|| blog_id.to_string());
                        
                        web_sys::console::log_1(&format!("🔗 Hash redirect detected for blog ID: '{}'", decoded_id).into());
                        
                        if !decoded_id.is_empty() {
                            // Clear the hash and navigate to blog post
                            let _ = window.history().unwrap().replace_state_with_url(&wasm_bindgen::JsValue::NULL, "", Some("/"));
                            navigator_clone.replace(&Route::BlogPost { id: decoded_id });
                            return;
                        }
                    }
                }
                
                // Fallback: check pathname for direct blog URLs
                if let Ok(pathname) = window.location().pathname() {
                    web_sys::console::log_1(&format!("🔍 RedirectHandler: Checking pathname '{}'", pathname).into());
                    
                    // More robust blog URL detection
                    if pathname.starts_with("/blog/") {
                        let path_after_blog = &pathname[6..]; // Remove "/blog/"
                        let clean_id = path_after_blog
                            .trim_end_matches('/')
                            .trim_end_matches(".html")
                            .trim_end_matches("/index.html")
                            .trim_end_matches("/index");
                            
                        web_sys::console::log_1(&format!("🔍 Extracted ID from pathname: '{}'", clean_id).into());
                        
                        if !clean_id.is_empty() {
                            web_sys::console::log_1(&format!("🚀 Navigating to blog post: {}", clean_id).into());
                            
                            // Use replace instead of push to avoid back button issues
                            navigator_clone.replace(&Route::BlogPost { id: clean_id.to_string() });
                        } else {
                            web_sys::console::log_1(&"❌ Empty blog ID, redirecting to home".into());
                            navigator_clone.replace(&Route::Home);
                        }
                    }
                }
            }
        });
        || {}
    });
    
    html! {
        <div style="display: none;">{"<!-- RedirectHandler active -->"}</div>
    }
}

fn switch(routes: Route) -> Html {
    // Add debug logging for route matching
    web_sys::console::log_1(&format!("🎯 Route matched: {:?}", routes).into());
    
    match routes {
        Route::Home => html! {
            <div class="app">
                <MatrixRain />
                <div class="content-wrapper">
                    <Header />
                    <main>
                        <RedirectHandler />
                        <About />
                        <Skills />
                        <Projects />
                        <BlogList />
                        <Streaming />
                        <Snake />
                        <Contact />
                    </main>
                    <div class="site-counter">
                        <Counter counter_type={CounterType::SiteVisits} label="TOTAL SITE VISITS" />
                    </div>
                    <Terminal />
                </div>
            </div>
        },
        Route::BlogPost { id } => html! {
            <div class="app">
                <MatrixRain />
                <div class="content-wrapper">
                    <Header />
                    <main>
                        <BlogPostComponent post_id={id} />
                    </main>
                    <div class="site-counter">
                        <Counter counter_type={CounterType::SiteVisits} label="TOTAL SITE VISITS" />
                    </div>
                </div>
            </div>
        },
        Route::NotFound => html! {
            <div class="app">
                <MatrixRain />
                <div class="content-wrapper">
                    <Header />
                    <main>
                        <RedirectHandler />
                        <div style="text-align: center; padding: 2rem;">
                            <h2>{"Page Not Found"}</h2>
                            <p>{"The page you're looking for doesn't exist. Checking for blog redirects..."}</p>
                        </div>
                    </main>
                    <div class="site-counter">
                        <Counter counter_type={CounterType::SiteVisits} label="TOTAL SITE VISITS" />
                    </div>
                </div>
            </div>
        }
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

use gloo::events::EventListener;
use wasm_bindgen::JsCast;

fn main() {
    // Attach a global keydown event listener to prevent arrow/space scrolling
    let window = web_sys::window().unwrap();
    let _listener = EventListener::new(&window, "keydown", move |event| {
        if let Some(event) = event.dyn_ref::<web_sys::KeyboardEvent>() {
            match event.key().as_str() {
                "ArrowUp" | "ArrowDown" | "ArrowLeft" | "ArrowRight" | " " => {
                    event.prevent_default();
                }
                _ => {}
            }
        }
    });
    // Leak the listener so it lives for the app lifetime
    std::mem::forget(_listener);
    yew::Renderer::<App>::new().render();
}
