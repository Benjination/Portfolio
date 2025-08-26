use yew::prelude::*;

#[function_component(Streaming)]
pub fn streaming() -> Html {
    html! {
        <section id="streaming" class="terminal-section">
            <div class="section-header">
                <span class="prompt">{"benjamin@BenjaminNiccum:~$"}</span>
                <span class="command">{"./twitch_stream.sh --status"}</span>
            </div>
            
            <div class="section-content">
                <div class="streaming-content">
                    <h2>{"🎮 Live Streaming"}</h2>
                    <p>{"Watch me code, game, and create in real-time! I stream programming sessions, game development, and tech discussions."}</p>
                    
                    <div class="twitch-widget">
                        <div class="twitch-header">
                            <span class="twitch-icon">{"📺"}</span>
                            <span class="twitch-title">{"Twitch Stream"}</span>
                            <span class="stream-status">{"LIVE"}</span>
                        </div>
                        
                        <div class="twitch-embed-container">
                            // Embedded Twitch player
                            <iframe
                                src="https://player.twitch.tv/?channel=benjination&parent=localhost&parent=benjination.github.io&parent=vercel.app&parent=netlify.app&parent=surge.sh&parent=benjaminniccum.com"
                                height="400"
                                width="100%"
                                allowfullscreen={true}>
                            </iframe>
                        </div>
                        
                        <div class="twitch-actions">
                            <a href="https://www.twitch.tv/benjination" target="_blank" class="twitch-link">
                                <span class="twitch-icon">{"🎮"}</span>
                                <span>{"Follow on Twitch"}</span>
                            </a>
                            
                            <div class="stream-info">
                                <h3>{"🎯 What I Stream:"}</h3>
                                <ul>
                                    <li>{"Live coding sessions"}</li>
                                    <li>{"Gaming - ESO, Baldur's Gate, and more"}</li>
                                    <li>{"Web development tutorials"}</li>
                                    <li>{"Tech talks and Q&A"}</li>
                                </ul>
                            </div>
                        </div>
                    </div>
                    
                    <div class="streaming-schedule">
                        <h3>{"📅 Streaming Schedule:"}</h3>
                        <div class="schedule-grid">
                            <div class="schedule-item">
                                <span class="day">{"Tuesday"}</span>
                                <span class="time">{"12:00 PM - 3:00 PM CST"}</span>
                            </div>
                            <div class="schedule-item">
                                <span class="day">{"Thursday"}</span>
                                <span class="time">{"12:00 PM - 3:00 PM CST"}</span>
                            </div>
                            <div class="schedule-item">
                                <span class="day">{"Friday"}</span>
                                <span class="time">{"8:00 PM - 10:00 PM CST"}</span>
                            </div>
                        </div>
                        <p class="schedule-note">{"Schedule may vary - follow on Twitch for notifications!"}</p>
                    </div>
                </div>
            </div>
        </section>
    }
}
