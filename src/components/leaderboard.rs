use yew::prelude::*;
use gloo::storage::{LocalStorage, Storage};
use gloo::net::http::Request;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct HighScore {
    pub name: String,
    pub score: u32,
    pub date: String,
}

#[derive(Serialize, Deserialize)]
struct GlobalLeaderboard {
    scores: Vec<HighScore>,
}

#[derive(Clone, Debug, PartialEq)]
enum LeaderboardMode {
    Local,
    Global,
}

// Global leaderboard functions
async fn fetch_global_scores() -> Result<Vec<HighScore>, Box<dyn std::error::Error>> {
    // Using a GitHub Gist as a simple JSON database
    // You can replace this URL with your own gist
    let gist_url = "https://gist.githubusercontent.com/Benjination/YOUR_GIST_ID/raw/snake_leaderboard.json";
    
    match Request::get(gist_url).send().await {
        Ok(response) => {
            if response.ok() {
                match response.json::<GlobalLeaderboard>().await {
                    Ok(leaderboard) => Ok(leaderboard.scores),
                    Err(_) => {
                        // If JSON parsing fails, return empty scores
                        Ok(Vec::new())
                    }
                }
            } else {
                // If request fails, return empty scores
                Ok(Vec::new())
            }
        }
        Err(_) => Ok(Vec::new())
    }
}

async fn submit_global_score(name: String, score: u32) -> Result<Vec<HighScore>, Box<dyn std::error::Error>> {
    // For now, we'll simulate a successful submission
    // In a real implementation, you'd send this to a backend API
    
    // Fetch current scores
    let mut scores = fetch_global_scores().await?;
    
    let now = js_sys::Date::new_0();
    let date = format!("{}/{}/{}", 
        now.get_month() as u32 + 1,
        now.get_date() as u32,
        now.get_full_year() as u32
    );
    
    scores.push(HighScore { name, score, date });
    scores.sort_by(|a, b| b.score.cmp(&a.score));
    scores.truncate(50); // Keep top 50 global scores
    
    // TODO: In a real implementation, send the updated scores to your backend
    // For now, just return the updated scores locally
    
    Ok(scores)
}

#[function_component(Leaderboard)]
pub fn leaderboard() -> Html {
    let local_scores = use_state(|| get_local_scores());
    let global_scores = use_state(|| Vec::<HighScore>::new());
    let current_mode = use_state(|| LeaderboardMode::Local);
    let loading_global = use_state(|| false);
    let show_form = use_state(|| false);
    let player_name = use_state(|| String::new());
    let temp_score = use_state(|| 0u32);
    let submit_to_global = use_state(|| false);

    // Load global scores when switching to global mode
    {
        let global_scores = global_scores.clone();
        let loading_global = loading_global.clone();
        let current_mode = current_mode.clone();
        
        use_effect_with(current_mode.clone(), move |mode| {
            if **mode == LeaderboardMode::Global && global_scores.is_empty() {
                loading_global.set(true);
                let global_scores = global_scores.clone();
                let loading_global = loading_global.clone();
                
                spawn_local(async move {
                    match fetch_global_scores().await {
                        Ok(scores) => global_scores.set(scores),
                        Err(_) => {
                            // If fetch fails, show empty state
                            global_scores.set(Vec::new());
                        }
                    }
                    loading_global.set(false);
                });
            }
            || {}
        });
    }

    let toggle_mode = {
        let current_mode = current_mode.clone();
        Callback::from(move |_: MouseEvent| {
            let new_mode = match *current_mode {
                LeaderboardMode::Local => LeaderboardMode::Global,
                LeaderboardMode::Global => LeaderboardMode::Local,
            };
            current_mode.set(new_mode);
        })
    };

    let handle_name_input = {
        let player_name = player_name.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            player_name.set(input.value());
        })
    };

    let handle_checkbox_change = {
        let submit_to_global = submit_to_global.clone();
        Callback::from(move |e: Event| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            submit_to_global.set(input.checked());
        })
    };

    let handle_score_submit = {
        let local_scores = local_scores.clone();
        let global_scores = global_scores.clone();
        let show_form = show_form.clone();
        let player_name = player_name.clone();
        let temp_score = temp_score.clone();
        let submit_to_global = submit_to_global.clone();
        
        Callback::from(move |_| {
            if !player_name.is_empty() && *temp_score > 0 {
                // Always save locally
                add_local_score((*player_name).clone(), *temp_score);
                local_scores.set(get_local_scores());
                
                // Submit to global if requested
                if *submit_to_global {
                    let name = (*player_name).clone();
                    let score = *temp_score;
                    let global_scores = global_scores.clone();
                    
                    spawn_local(async move {
                        if let Ok(updated_scores) = submit_global_score(name, score).await {
                            global_scores.set(updated_scores);
                        }
                    });
                }
                
                show_form.set(false);
                player_name.set(String::new());
                temp_score.set(0);
                submit_to_global.set(false);
            }
        })
    };

    let current_scores = match *current_mode {
        LeaderboardMode::Local => (*local_scores).clone(),
        LeaderboardMode::Global => (*global_scores).clone(),
    };

    html! {
        <section id="leaderboard" class="terminal-section">
            <div class="section-header">
                <span class="prompt">{"benjamin@BenjaminNiccum:~$"}</span>
                <span class="command">{"cat /var/log/snake_scores.log | head -10"}</span>
            </div>
            
            <div class="section-content">
                <div class="leaderboard-container">
                    <div class="leaderboard-header">
                        <h3 class="leaderboard-title">{"🏆 High Scores"}</h3>
                        <div class="mode-toggle">
                            <button 
                                class={classes!("mode-btn", if *current_mode == LeaderboardMode::Local { "active" } else { "" })}
                                onclick={
                                    let current_mode = current_mode.clone();
                                    Callback::from(move |_| current_mode.set(LeaderboardMode::Local))
                                }
                            >
                                {"📱 Local"}
                            </button>
                            <button 
                                class={classes!("mode-btn", if *current_mode == LeaderboardMode::Global { "active" } else { "" })}
                                onclick={
                                    let current_mode = current_mode.clone();
                                    Callback::from(move |_| current_mode.set(LeaderboardMode::Global))
                                }
                                disabled={*loading_global}
                            >
                                {"🌍 Global"}
                            </button>
                        </div>
                    </div>

                    {if *current_mode == LeaderboardMode::Global && *loading_global {
                        html! {
                            <div class="loading">
                                <p>{"⏳ Loading global scores..."}</p>
                            </div>
                        }
                    } else if current_scores.is_empty() {
                        html! {
                            <div class="no-scores">
                                <p>{"No high scores yet!"}</p>
                                <p>{"Be the first to play Snake and set a record! 🐍"}</p>
                            </div>
                        }
                    } else {
                        html! {
                            <div class="scores-list">
                                <div class="scores-header">
                                    <span class="rank-header">{"Rank"}</span>
                                    <span class="name-header">{"Name"}</span>
                                    <span class="score-header">{"Score"}</span>
                                    <span class="date-header">{"Date"}</span>
                                </div>
                                {for current_scores.iter().enumerate().map(|(index, score)| {
                                    let rank = index + 1;
                                    let medal = match rank {
                                        1 => "🥇",
                                        2 => "🥈", 
                                        3 => "🥉",
                                        _ => "  "
                                    };
                                    
                                    html! {
                                        <div class={classes!("score-entry", if rank <= 3 { "top-three" } else { "" })}>
                                            <span class="rank">{format!("{} {}", medal, rank)}</span>
                                            <span class="name">{&score.name}</span>
                                            <span class="score">{score.score}</span>
                                            <span class="date">{&score.date}</span>
                                        </div>
                                    }
                                })}
                            </div>
                        }
                    }}
                    
                    {if *show_form {
                        html! {
                            <div class="score-form">
                                <h4>{"New High Score!"}</h4>
                                <p>{format!("You scored {} points!", *temp_score)}</p>
                                <div class="form-group">
                                    <label for="player-name">{"Enter your name:"}</label>
                                    <input 
                                        type="text" 
                                        id="player-name"
                                        class="name-input"
                                        value={(*player_name).clone()}
                                        oninput={handle_name_input}
                                        placeholder="Enter your initials"
                                        maxlength="10"
                                    />
                                </div>
                                <div class="form-group">
                                    <label class="checkbox-label">
                                        <input 
                                            type="checkbox"
                                            class="global-checkbox"
                                            checked={*submit_to_global}
                                            onchange={handle_checkbox_change}
                                        />
                                        <span class="checkbox-text">{"Submit to Global Leaderboard"}</span>
                                    </label>
                                    <p class="checkbox-help">{"Share your score with all visitors to this site"}</p>
                                </div>
                                <div class="form-buttons">
                                    <button 
                                        onclick={handle_score_submit} 
                                        class="submit-btn"
                                        disabled={player_name.is_empty()}
                                    >
                                        {"Save Score"}
                                    </button>
                                    <button 
                                        onclick={Callback::from(move |_| show_form.set(false))} 
                                        class="cancel-btn"
                                    >
                                        {"Cancel"}
                                    </button>
                                </div>
                            </div>
                        }
                    } else {
                        html! {}
                    }}
                    
                    <div class="leaderboard-info">
                        <p>{"💾 Local: Saved in your browser only"}</p>
                        <p>{"🌍 Global: Compete with all visitors to this site"}</p>
                        <p>{"Play Snake above to compete for high scores!"}</p>
                    </div>
                </div>
            </div>
        </section>
    }
}

// Local storage functions
fn get_local_scores() -> Vec<HighScore> {
    LocalStorage::get("snake_high_scores").unwrap_or_else(|_| Vec::new())
}

pub fn add_local_score(name: String, score: u32) {
    let mut scores = get_local_scores();
    
    let now = js_sys::Date::new_0();
    let date = format!("{}/{}/{}", 
        now.get_month() as u32 + 1,
        now.get_date() as u32,
        now.get_full_year() as u32
    );
    
    scores.push(HighScore { name, score, date });
    scores.sort_by(|a, b| b.score.cmp(&a.score));
    scores.truncate(10); // Keep only top 10
    
    let _ = LocalStorage::set("snake_high_scores", &scores);
}

pub fn is_high_score(score: u32) -> bool {
    let local_scores = get_local_scores();
    local_scores.len() < 10 || local_scores.iter().any(|s| score > s.score)
}

// API function for showing score form from the snake game
pub fn show_score_form(_score: u32) {
    // This function is called from the snake game when a high score is achieved
    // The actual implementation would need to be handled by passing callbacks
    // or using a global state management solution
}

use yew::prelude::*;

