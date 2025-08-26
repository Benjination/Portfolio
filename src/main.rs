use yew::prelude::*;

mod components;

use components::{Header, About, Skills, Projects, Contact, Terminal, MatrixRain, Snake, Counter, CounterType, Streaming};

#[function_component(App)]
fn app() -> Html {
    html! {
        <div class="app">
            <MatrixRain />
            <div class="content-wrapper">
                <Header />
                <main>
                    <About />
                    <Skills />
                    <Projects />
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
    }
}

use wasm_bindgen::prelude::*;
use web_sys::window;
use gloo::events::EventListener;

fn main() {
    // Attach a global keydown event listener to prevent arrow/space scrolling
    let window = window().unwrap();
    let _listener = EventListener::new(&window, "keydown", move |event| {
        let event = event.dyn_ref::<web_sys::KeyboardEvent>().unwrap();
        match event.key().as_str() {
            "ArrowUp" | "ArrowDown" | "ArrowLeft" | "ArrowRight" | " " => {
                event.prevent_default();
            }
            _ => {}
        }
    });
    // Leak the listener so it lives for the app lifetime
    std::mem::forget(_listener);
    yew::Renderer::<App>::new().render();
}
