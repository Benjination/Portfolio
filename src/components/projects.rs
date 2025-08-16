use yew::prelude::*;

#[function_component(Projects)]
pub fn projects() -> Html {
    html! {
        <section id="projects" class="terminal-section">
            <div class="section-header">
                <span class="prompt">{"benjamin@BenjaminNiccum:~$"}</span>
                <span class="command">{"git log --oneline --graph"}</span>
            </div>
            
            <div class="section-content">
                <div class="projects-intro">
{r#"🚀 Projects I've Built

Here are some of the things I've created recently. Each project taught me something new 
and represents a different challenge I've tackled. I love building things that people 
actually want to use!

📊 Project Stats:
   • 150+ commits across various projects
   • 10,000+ lines of code written  
   • 8 different programming languages used
   • ∞ cups of coffee consumed ☕ (definitely the most important metric!)
"#}
                </div>
                
                <div class="project-cards">
                    <div class="project-card">
                        <div class="card-header">
                            <span class="file-icon">{"📱"}</span>
                            <span class="project-name">{"The Mystical Tarot"}</span>
                            <span class="language-tag">{"iOS App"}</span>
                        </div>
                        <div class="card-content">
                            <p>{"A beautiful iOS tarot card reading app with custom animations and AI-powered insights. Users can get personalized readings and explore the mystical world of tarot cards with smooth, engaging interactions."}</p>
                            <div class="tech-stack">
                                <span class="tech">{"SwiftUI"}</span>
                                <span class="tech">{"Core ML"}</span>
                                <span class="tech">{"Firebase"}</span>
                            </div>
                        </div>
                    </div>
                    
                    <div class="project-card">
                        <div class="card-header">
                            <span class="file-icon">{"🎮"}</span>
                            <span class="project-name">{"Elder Scrolls Mods"}</span>
                            <span class="language-tag">{"Game Mods"}</span>
                        </div>
                        <div class="card-content">
                            <p>{"Custom game modifications and add-ons that enhance the Elder Scrolls gaming experience. Features advanced scripting systems and UI improvements that thousands of players have downloaded and enjoyed."}</p>
                            <div class="tech-stack">
                                <span class="tech">{"C++"}</span>
                                <span class="tech">{"Game Engine API"}</span>
                                <span class="tech">{"Modding Tools"}</span>
                            </div>
                        </div>
                    </div>
                    
                    <div class="project-card">
                        <div class="card-header">
                            <span class="file-icon">{"🦀"}</span>
                            <span class="project-name">{"This Portfolio"}</span>
                            <span class="language-tag">{"Web App"}</span>
                        </div>
                        <div class="card-content">
                            <p>{"The website you're looking at right now! Built with cutting-edge web technologies for lightning-fast performance. Features a retro terminal theme, interactive Snake game, and real-time visitor counters."}</p>
                            <div class="tech-stack">
                                <span class="tech">{"Rust"}</span>
                                <span class="tech">{"WebAssembly"}</span>
                                <span class="tech">{"Firebase"}</span>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}
