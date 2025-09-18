use yew::prelude::*;
use yew_router::prelude::*;

mod components;

use components::{Header, About, Skills, Projects, Contact, Terminal, MatrixRain, Snake, Counter, CounterType, Streaming, BlogList, BlogPostComponent};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/blog/:id")]
    BlogPost { id: String },
}

#[function_component(RedirectHandler)]
fn redirect_handler() -> Html {
    let navigator = use_navigator().unwrap();
    
    use_effect_with((), move |_| {
        // Check if the current URL path contains a blog post pattern
        if let Some(window) = web_sys::window() {
            if let Ok(pathname) = window.location().pathname() {
                // Check if we're on a blog post URL (like /blog/post_123456)
                if pathname.starts_with("/blog/post_") {
                    let post_id = pathname.trim_start_matches("/blog/").to_string();
                    // Navigate to the blog post route in the Yew app
                    navigator.push(&Route::BlogPost { id: post_id });
                }
            }
        }
        || {}
    });
    
    html! {}
}

fn switch(routes: Route) -> Html {
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
