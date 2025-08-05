use yew::prelude::*;

#[function_component(Projects)]
pub fn projects() -> Html {
    html! {
        <section id="projects" class="terminal-section">
            <div class="section-header">
                <span class="prompt">{"benjamin@portfolio:~$"}</span>
                <span class="command">{"git log --oneline --graph"}</span>
            </div>
            
            <div class="section-content">
                <pre class="git-log">
{r#"* 7f5a8bc (HEAD -> main) feat: Portfolio Website in Rust + Yew
|\  
| * 4e2d9fa feat: Advanced WebAssembly implementation
| * 2b1c5ef refactor: Modern terminal-inspired UI
|/  
* 8a3f7bd (tag: v2.0) feat: The Mystical Tarot iOS App
|\  
| * 5c9e4af feat: Core ML integration for tarot insights
| * 3d7b2fe feat: SwiftUI custom card animations
| * 1a6f8ce feat: Firebase backend integration
|/  
* 6b4a9cd feat: Elder Scrolls Add-ons
|\  
| * 9e7f3bc feat: Advanced scripting system
| * 8d2a5fe feat: Custom UI modifications
|/  
* 4c8e2fb feat: GitHub Profile Enhancement
|\  
| * 7a1b9de feat: Dynamic Spotify integration
| * 2f5c8ea feat: Animated contribution snake
| * 6d4e7af feat: Custom README automation
|/  
* 1b9d5ea init: Begin software engineering journey

Repository Statistics:
├── Total Commits: 150+
├── Lines of Code: 10,000+
├── Languages Used: 8
└── Coffee Consumed: ∞ cups ☕
"#}
                </pre>
                
                <div class="project-cards">
                    <div class="project-card">
                        <div class="card-header">
                            <span class="file-icon">{"📱"}</span>
                            <span class="project-name">{"The Mystical Tarot"}</span>
                            <span class="language-tag">{"Swift"}</span>
                        </div>
                        <div class="card-content">
                            <p>{"iOS tarot card reading app with custom animations and AI-powered insights."}</p>
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
                            <span class="language-tag">{"C++"}</span>
                        </div>
                        <div class="card-content">
                            <p>{"Game modifications and add-ons with advanced scripting systems."}</p>
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
                            <span class="language-tag">{"Rust"}</span>
                        </div>
                        <div class="card-content">
                            <p>{"Modern portfolio built with Rust and WebAssembly for maximum performance."}</p>
                            <div class="tech-stack">
                                <span class="tech">{"Rust"}</span>
                                <span class="tech">{"Yew"}</span>
                                <span class="tech">{"WebAssembly"}</span>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}
