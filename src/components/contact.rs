use yew::prelude::*;

#[function_component(Contact)]
pub fn contact() -> Html {
    html! {
        <section id="contact" class="terminal-section">
            <div class="section-header">
                <span class="prompt">{"benjamin@BenjaminNiccum:~$"}</span>
                <span class="command">{"./connect.sh --help"}</span>
            </div>
            
            <div class="section-content">
                <div class="contact-content">
                    <h2>{"💬 Let's Connect!"}</h2>
                    <p>{"I'd love to hear from you! Whether you have a project idea, want to collaborate, or just want to chat about technology, I'm always excited to meet new people and explore new opportunities."}</p>
                    
                    <h3>{"📧 Best way to reach me:"}</h3>
                    <p>{"benjination2@gmail.com - I typically respond within 24 hours (often much faster!)"}</p>
                    
                    <h3>{"🌐 Find me online:"}</h3>
                    <ul>
                        <li>{"GitHub - Check out my code and contributions"}</li>
                        <li>{"LinkedIn - Connect for professional networking"}</li>
                        <li>{"Portfolio - My previous website (this one's replacing it!)"}</li>
                        <li>{"Spotify - See what I'm listening to while coding 🎵"}</li>
                    </ul>
                    
                    <h3>{"📍 About my availability:"}</h3>
                    <ul>
                        <li>{"Currently available and ready to collaborate"}</li>
                        <li>{"Based in the US but flexible with global teams"}</li>
                        <li>{"Response time: Usually less than 24 hours"}</li>
                    </ul>
                    
                    <h3>{"🎯 What I'm looking for:"}</h3>
                    <ul>
                        <li>{"Software engineering opportunities"}</li>
                        <li>{"Interesting project collaborations"}</li>
                        <li>{"Open source contributions"}</li>
                        <li>{"Mentorship and learning opportunities"}</li>
                    </ul>
                    
                    <div class="call-to-action">
                        <p>{"💡 Got an idea? A problem to solve? A cool project in mind? Let's talk! I believe the best projects come from great conversations."}</p>
                        
                        <h3>{"🤝 Whether you're:"}</h3>
                        <ul>
                            <li>{"A company looking for a passionate developer"}</li>
                            <li>{"A fellow engineer wanting to collaborate"}</li>
                            <li>{"Someone with a crazy idea that might just work"}</li>
                            <li>{"A student or newcomer seeking guidance"}</li>
                        </ul>
                        
                        <p>{"Don't hesitate to reach out. I'm always excited to connect with like-minded people and explore what we can build together! ✨"}</p>
                    </div>
                </div>
                
                <div class="contact-links">
                    <a href="mailto:benjination2@gmail.com" class="contact-link">
                        <span class="link-icon">{"📧"}</span>
                        <span class="link-text">{"benjination2@gmail.com"}</span>
                    </a>
                    
                    <a href="https://github.com/Benjination" class="contact-link" target="_blank">
                        <span class="link-icon">{"💻"}</span>
                        <span class="link-text">{"github.com/Benjination"}</span>
                    </a>
                    
                    <a href="https://linkedin.com/in/b-niccum" class="contact-link" target="_blank">
                        <span class="link-icon">{"💼"}</span>
                        <span class="link-text">{"linkedin.com/in/b-niccum"}</span>
                    </a>
                    
                    <a href="https://benjaminniccum.godaddysites.com" class="contact-link" target="_blank">
                        <span class="link-icon">{"🌐"}</span>
                        <span class="link-text">{"Legacy Portfolio"}</span>
                    </a>
                </div>
            </div>
        </section>
    }
}
