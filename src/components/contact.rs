use yew::prelude::*;

#[function_component(Contact)]
pub fn contact() -> Html {
    html! {
        <section id="contact" class="terminal-section">
            <div class="section-header">
                <span class="prompt">{"benjamin@portfolio:~$"}</span>
                <span class="command">{"./connect.sh --help"}</span>
            </div>
            
            <div class="section-content">
                <pre class="contact-info">
{r#"connect.sh - Establish connection with Benjamin Niccum

USAGE:
    ./connect.sh [OPTIONS]

OPTIONS:
    --email         Primary communication channel
    --github        View source code and contributions  
    --linkedin      Professional networking
    --portfolio     Legacy website (being replaced by this one!)
    --spotify       Current coding soundtrack 🎵

EXAMPLES:
    ./connect.sh --email
        Opens: benjination2@gmail.com
        
    ./connect.sh --github  
        Opens: https://github.com/Benjination
        
    ./connect.sh --linkedin
        Opens: https://linkedin.com/in/b-niccum
        
    ./connect.sh --portfolio
        Opens: https://benjaminniccum.godaddysites.com

AVAILABILITY:
    Status: ● Online and ready to collaborate
    Response Time: < 24 hours
    Time Zone: US (flexible for global teams)
    
CURRENTLY SEEKING:
    - Software Engineering roles
    - Interesting project collaborations  
    - Open source contributions
    - Mentorship opportunities

ERROR CODES:
    0    Success - Connection established
    1    Busy - Currently deep in code
    2    Away - Probably debugging something
    404  Not Found - Try harder! 😄
"#}
                </pre>
                
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
