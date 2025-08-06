use yew::prelude::*;

mod components;

use components::{Header, About, Skills, Projects, Contact, Terminal, MatrixRain, Snake};

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
                    <Snake />
                    <Contact />
                </main>
                <Terminal />
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
