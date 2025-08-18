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
{r#"🚀 Recent Projects

Here are some of the things I've built recently. Each project represents different challenges I've tackled, from holistic health websites to desktop applications. I enjoy creating solutions that solve real-world problems!

📊 Project Stats:
   • 50+ commits across recent projects
   • Multiple programming languages and frameworks
   • Full-stack development experience
   • ∞ cups of coffee consumed ☕ (definitely the most important metric!)
"#}
                </div>
                
                <div class="project-cards">
                    <div class="project-card">
                        <div class="card-header">
                            <span class="file-icon">{"🌿"}</span>
                            <span class="project-name">{"Healing Synergies"}</span>
                            <span class="language-tag">{"Website"}</span>
                        </div>
                        <div class="card-content">
                            <p>{"A comprehensive holistic health and wellness website featuring service management, recipe collections, and client testimonials. Built with modern web technologies and includes payment integration, contact forms, and responsive design."}</p>
                            <div class="tech-stack">
                                <span class="tech">{"HTML5"}</span>
                                <span class="tech">{"CSS3"}</span>
                                <span class="tech">{"JavaScript"}</span>
                                <span class="tech">{"GitHub Pages"}</span>
                            </div>
                            <div class="project-links">
                                <a href="https://HealingSynergies.com" target="_blank" rel="noopener noreferrer" class="project-link">
                                    {"🌐 Live Site"}
                                </a>
                                <a href="https://github.com/Benjination/Healing-Synergies" target="_blank" rel="noopener noreferrer" class="project-link">
                                    {"📂 View Code"}
                                </a>
                            </div>
                        </div>
                    </div>
                    
                    <div class="project-card">
                        <div class="card-header">
                            <span class="file-icon">{"�"}</span>
                            <span class="project-name">{"The Pantry"}</span>
                            <span class="language-tag">{"Desktop App"}</span>
                        </div>
                        <div class="card-content">
                            <p>{"A beautiful macOS meal planning application with recipe management, weekly menu boards, and smart shopping list generation. Features Walmart integration, local data storage, and an intuitive drag-and-drop interface."}</p>
                            <div class="tech-stack">
                                <span class="tech">{"TypeScript"}</span>
                                <span class="tech">{"Electron"}</span>
                                <span class="tech">{"Node.js"}</span>
                            </div>
                            <div class="project-links">
                                <a href="https://github.com/Benjination/The-Pantry" target="_blank" rel="noopener noreferrer" class="project-link">
                                    {"📂 View Repository"}
                                </a>
                            </div>
                        </div>
                    </div>
                    
                    <div class="project-card">
                        <div class="card-header">
                            <span class="file-icon">{"🌱"}</span>
                            <span class="project-name">{"Adelante Landscapes"}</span>
                            <span class="language-tag">{"Jekyll Site"}</span>
                        </div>
                        <div class="card-content">
                            <p>{"Professional landscape design website built with Jekyll for GitHub Pages. Features responsive design, filterable portfolio gallery, blog system, and contact integration. Showcases edible landscaping and sustainable garden design services."}</p>
                            <div class="tech-stack">
                                <span class="tech">{"Jekyll"}</span>
                                <span class="tech">{"HTML5"}</span>
                                <span class="tech">{"CSS3"}</span>
                                <span class="tech">{"GitHub Pages"}</span>
                            </div>
                            <div class="project-links">
                                <a href="https://AdelanteLandscapes.com" target="_blank" rel="noopener noreferrer" class="project-link">
                                    {"🌐 Live Site"}
                                </a>
                                <a href="https://github.com/Benjination/AdelanteLandscapes" target="_blank" rel="noopener noreferrer" class="project-link">
                                    {"📂 View Code"}
                                </a>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}
