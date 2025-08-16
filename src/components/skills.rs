use yew::prelude::*;

#[function_component(Skills)]
pub fn skills() -> Html {
    html! {
        <section id="skills" class="terminal-section">
            <div class="section-header">
                <span class="prompt">{"benjamin@BenjaminNiccum:~$"}</span>
                <span class="command">{"ls -la skills/"}</span>
            </div>
            
            <div class="section-content">
                <div class="skills-content">
                    <h2>{"🛠️ My Toolkit & Expertise"}</h2>
                    <p>{"I work with a variety of programming languages and tools to bring ideas to life. Here's what I can help you build with:"}</p>
                    
                    <h3>{"💻 Programming Languages I Love:"}</h3>
                    <ul>
                        <li>{"🦀 Rust - My current favorite! Great for building fast, reliable software"}</li>
                        <li>{"☕ Java - Solid choice for large applications and enterprise solutions"}</li>
                        <li>{"🐍 Python - Perfect for data work, automation, and quick prototypes"}</li>
                        <li>{"🎯 C/C++ - When you need maximum performance and control"}</li>
                        <li>{"📱 Swift - For beautiful iOS apps that users actually enjoy"}</li>
                        <li>{"🔧 Assembly - Getting down to the metal when needed"}</li>
                        <li>{"📊 SQL - Making sense of data and databases"}</li>
                    </ul>
                    
                    <h3>{"🌐 Web Technologies:"}</h3>
                    <ul>
                        <li>{"HTML5 & CSS3 - The foundation of every great website"}</li>
                        <li>{"JavaScript - Making websites interactive and dynamic"}</li>
                        <li>{"WebAssembly - Bringing desktop-level performance to the web"}</li>
                    </ul>
                    
                    <h3>{"🔨 Tools & Frameworks I Use Daily:"}</h3>
                    <ul>
                        <li>{"Mobile Development (iOS & Android)"}</li>
                        <li>{"Git - Keeping track of code changes and collaborating with teams"}</li>
                        <li>{"Firebase - Building apps with real-time features"}</li>
                        <li>{"Unix/Linux Systems - My preferred development environment"}</li>
                        <li>{"Project Management (Jira, Confluence)"}</li>
                        <li>{"LaTeX - For beautiful documentation and reports"}</li>
                    </ul>
                    
                    <h3>{"🎯 What I'm Passionate About:"}</h3>
                    <ul>
                        <li>{"Building mobile apps that people love to use"}</li>
                        <li>{"Designing efficient algorithms and data structures"}</li>
                        <li>{"Creating scalable system architectures"}</li>
                        <li>{"Solving complex technical challenges"}</li>
                    </ul>
                    
                    <h3>{"🌱 Currently Exploring:"}</h3>
                    <p>{"Artificial Intelligence, Machine Learning, and Advanced Software Design Patterns (Always learning something new!)"}</p>
                </div>
                
                <div class="skill-highlight">
                    <span class="comment">{"💡 Love working with new technologies? Let's explore them together!"}</span>
                </div>
            </div>
        </section>
    }
}
