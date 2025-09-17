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

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {
            <div class="app">
                <MatrixRain />
                <div class="content-wrapper">
                    <Header />
                    <main>
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
                    <Terminal />
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

use wasm_bindgen::prelude::*;

use wasm_bindgen::prelude::*;
use web_sys::window;
use gloo::events::EventListener;

fn main() {
    // Attach a global keydown event listener to prevent arrow/space scrolling
    let window = window().unwrap();
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
