use yew::prelude::*;
use web_sys::{HtmlInputElement, window};
use gloo::timers::callback::Timeout;
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen::JsCast;
use crate::components::blog_auth::{verify_blog_password, verify_blog_credentials, authenticate_admin, AuthState};
use crate::components::blog_admin::BlogAdmin;

#[function_component(Terminal)]
pub fn terminal() -> Html {
    let input_ref = use_node_ref();
    let output = use_state(|| vec![
        "Welcome to Benjamin's Terminal! ".to_string(),
        "Type 'help' for available commands.".to_string(),
        "".to_string(),
    ]);
    let input_value = use_state(|| String::new());
    let waiting_for_password = use_state(|| false);
    let waiting_for_email = use_state(|| false);
    let blog_email = use_state(|| String::new());
    let password_attempts = use_state(|| 0u32);
    let auth_state = use_state(|| None::<AuthState>);

    // Auto-run a stored terminal command (e.g., "treat") after a page reload
    use_effect_with((), {
        let output = output.clone();
        move |_| {
            if let Some(window) = window() {
                if let Ok(Some(storage)) = window.local_storage() {
                    if let Ok(Some(cmd)) = storage.get_item("terminal-auto-run") {
                        if cmd == "treat" {
                            // Clear the auto-run flag to avoid loops
                            let _ = storage.remove_item("terminal-auto-run");

                            // Re-apply treat side effects
                            let _ = storage.remove_item("halloween-trick");
                            let _ = storage.set_item("halloween-treat", "true");

                            // Echo command and output
                            let mut new_output = (*output).clone();
                            new_output.push("👻benjamin@SpookyTown:~/haunted💀 treat".to_string());
                            new_output.extend(vec![
                                "🍬 TREAT! 🍭".to_string(),
                                "The ghosts fade away... 👻💨".to_string(),
                                "Pumpkins return to the autumn rain! �".to_string(),
                                "Sweet dreams... if you can sleep! 😈".to_string(),
                                "".to_string(),
                            ]);
                            output.set(new_output);

                            // Scroll to bottom after render
                            let timeout = Timeout::new(50, move || {
                                if let Some(win) = web_sys::window() {
                                    if let Some(doc) = win.document() {
                                        if let Some(body) = doc.body() {
                                            win.scroll_to_with_x_and_y(0.0, body.scroll_height() as f64);
                                        }
                                    }
                                }
                            });
                            timeout.forget();
                        }
                    }
                }
            }
            || {}
        }
    });
    
    let on_input = {
        let input_value = input_value.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            input_value.set(input.value());
        })
    };
    
    let on_keydown = {
        let input_value = input_value.clone();
        let output = output.clone();
        let input_ref = input_ref.clone();
        let waiting_for_password = waiting_for_password.clone();
        let waiting_for_email = waiting_for_email.clone();
        let blog_email = blog_email.clone();
        let password_attempts = password_attempts.clone();
        let auth_state_for_keydown = auth_state.clone();
        
        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                let command = (*input_value).clone();
                let mut new_output = (*output).clone();
                
                // Handle email input mode
                if *waiting_for_email {
                    let email = command.clone();
                    blog_email.set(email.clone());
                    waiting_for_email.set(false);
                    waiting_for_password.set(true);
                    
                    // Show email and prompt for password
                    new_output.push(format!("benjamin@BenjaminNiccum:~$ {}", email));
                    new_output.push("🔐 [sudo] password for admin: ".to_string());
                    output.set(new_output);
                    
                    input_value.set(String::new());
                    if let Some(input) = input_ref.cast::<HtmlInputElement>() {
                        input.set_value("");
                    }
                    return;
                }
                
                // Handle password input mode
                if *waiting_for_password {
                    let password = command.clone();
                    let email = (*blog_email).clone();
                    let output = output.clone();
                    let waiting_for_password = waiting_for_password.clone();
                    let waiting_for_email = waiting_for_email.clone();
                    let blog_email = blog_email.clone();
                    let password_attempts = password_attempts.clone();
                    let auth_state = auth_state_for_keydown.clone();
                    
                    // Don't show the password in the terminal output
                    new_output.push("benjamin@BenjaminNiccum:~$ [hidden]".to_string());
                    
                    spawn_local(async move {
                        match authenticate_admin(&email, &password).await {
                            Ok(auth) => {
                                let mut success_output = (*output).clone();
                                success_output.push("Access granted! 🎉".to_string());
                                success_output.push("".to_string());
                                success_output.push("🚀 Loading Blog Administration Panel...".to_string());
                                success_output.push("".to_string());
                                output.set(success_output);
                                waiting_for_password.set(false);
                                waiting_for_email.set(false);
                                blog_email.set(String::new());
                                password_attempts.set(0);
                                auth_state.set(Some(auth));
                            }
                            Err(_) => {
                                let attempts = *password_attempts + 1;
                                password_attempts.set(attempts);
                                
                                let mut failure_output = (*output).clone();
                                failure_output.push("Sorry, try again.".to_string());
                                
                                if attempts >= 3 {
                                    failure_output.push("sudo: 3 incorrect password attempts".to_string());
                                    failure_output.push("".to_string());
                                    waiting_for_password.set(false);
                                    waiting_for_email.set(false);
                                    blog_email.set(String::new());
                                    password_attempts.set(0);
                                } else {
                                    failure_output.push("🔐 [sudo] password for admin: ".to_string());
                                }
                                output.set(failure_output);
                            }
                        }
                    });
                    
                    input_value.set(String::new());
                    if let Some(input) = input_ref.cast::<HtmlInputElement>() {
                        input.set_value("");
                    }
                    return;
                }
                
                // Normal command handling
                new_output.push(format!("👻benjamin@CyberPunk:~/cyber {}", command));
                
                let response = match command.trim() {
                    "help" => vec![
                        "⚡ Available neural commands: 🤖".to_string(),
                        "  help        - Show this digital help interface".to_string(),
                        "  about       - Learn about the cyber Benjamin".to_string(),
                        "  skills      - View enhanced technical abilities".to_string(),
                        "  projects    - See recent code projects".to_string(),
                        "  contact     - Get contact info (direct neural link)".to_string(),
                        "  clear       - Clear the terminal buffer".to_string(),
                        "  whoami      - Display user profile data".to_string(),
                        "  ls          - List portfolio modules".to_string(),
                        "  cat resume  - Display resume data stream".to_string(),
                        "  sudo hire   - Execute employment protocol 💾".to_string(),
                    ],
                    "about" => vec![
                        "Benjamin Niccum - Software Engineer".to_string(),
                        "Recent graduate passionate about innovative development.".to_string(),
                        "Currently seeking opportunities to create amazing software!".to_string(),
                    ],
                    "skills" => vec![
                        "Core Languages: Rust, C, Java, Python, C++, Swift".to_string(),
                        "Current Focus: WebAssembly, System Design, AI/ML".to_string(),
                        "Tools: Git, Android Studio, Xcode, Firebase".to_string(),
                    ],
                    "projects" => vec![
                        "🎮 Elder Scrolls Add-ons (C++)".to_string(),
                        "📱 The Mystical Tarot (Swift/iOS)".to_string(),
                        "🦀 This Portfolio (Rust + WebAssembly)".to_string(),
                        "💻 GitHub Profile Enhancements".to_string(),
                    ],
                    "contact" => vec![
                        "📧 Email: benjination2@gmail.com".to_string(),
                        "💻 GitHub: github.com/Benjination".to_string(),
                        "💼 LinkedIn: linkedin.com/in/b-niccum".to_string(),
                    ],
                    "whoami" => vec![
                        "benjamin".to_string(),
                        "Groups: software-engineers, problem-solvers, coffee-addicts".to_string(),
                    ],
                    "ls" => vec![
                        "about.md  contact.sh  projects/  skills/  README.md".to_string(),
                    ],
                    "cat resume" => {
                        // Trigger download
                        if let Some(window) = web_sys::window() {
                            if let Some(document) = window.document() {
                                if let Ok(link) = document.create_element("a") {
                                    let link = link.dyn_into::<web_sys::HtmlAnchorElement>().unwrap();
                                    link.set_href("/BenjaminNiccum.pdf");
                                    link.set_download("Benjamin_Niccum_Resume.pdf");
                                    link.click();
                                }
                            }
                        }
                        
                        vec![
                            "💾 Downloading resume data stream...".to_string(),
                            "".to_string(),
                            "BENJAMIN NICCUM".to_string(),
                            "Software Engineer | Recent Graduate".to_string(),
                            "".to_string(),
                            "EXPERIENCE:".to_string(),
                            "• Cross-industry project experience".to_string(),
                            "• Mobile app development (iOS/Android)".to_string(),
                            "• Modern web technologies".to_string(),
                            "• Team collaboration and management".to_string(),
                            "".to_string(),
                            "📄 Full resume downloaded to your downloads folder.".to_string(),
                        ]
                    },
                    "sudo hire" => vec![
                        "[sudo] password for benjamin: ********".to_string(),
                        "".to_string(),
                        "Access granted! 🎉".to_string(),
                        "Initiating hiring process...".to_string(),
                        "Please contact benjination2@gmail.com to complete.".to_string(),
                    ],
                    "sudo blog" => {
                        waiting_for_email.set(true);
                        password_attempts.set(0);
                        blog_email.set(String::new());
                        
                        // Add the command line and email prompt
                        new_output.push(format!("benjamin@BenjaminNiccum:~$ {}", command));
                        new_output.push("� Enter admin email: ".to_string());
                        output.set(new_output);
                        input_value.set(String::new());
                        
                        // Clear input field
                        if let Some(input) = input_ref.cast::<HtmlInputElement>() {
                            input.set_value("");
                        }
                        return;
                    },
                    "clear" => {
                        output.set(vec![]);
                        input_value.set(String::new());
                        return;
                    },
                    "boo" => vec![
                        "👻 BOOOOOO! 👻".to_string(),
                        "Did I scare you? 🎃".to_string(),
                        "Happy Halloween! 🦇🕷️💀".to_string(),
                    ],
                    "trick" => {
                        web_sys::console::log_1(&"TRICK command executed".into());
                        // Use a simple state sharing approach instead of custom events
                        // Store trick state in local storage for the matrix rain to read
                        if let Some(window) = window() {
                            if let Ok(Some(storage)) = window.local_storage() {
                                let _ = storage.set_item("halloween-trick", "true");
                                // Clear any existing treat flag
                                let _ = storage.remove_item("halloween-treat");
                                web_sys::console::log_1(&"Set halloween-trick, removed halloween-treat".into());
                                // Ghosts will persist until treat command is used
                            }
                        }
                        
                        vec![
                            "🎭 TRICK! 🎭".to_string(),
                            "🎃 The pumpkins transform into floating ghosts! 👻".to_string(),
                            "Watch the matrix rain change above... 🦇".to_string(),
                        ]
                    },
                    "treat" => {
                        web_sys::console::log_1(&"TREAT command executed".into());
                        // Set treat flag to stop ghosts and return to pumpkins
                        if let Some(window) = window() {
                            if let Ok(Some(storage)) = window.local_storage() {
                                // Clear ALL Halloween flags first
                                let _ = storage.remove_item("halloween-trick");
                                let _ = storage.remove_item("halloween-treat");
                                // Then set the treat flag
                                let _ = storage.set_item("halloween-treat", "true");
                                web_sys::console::log_1(&"Cleared all flags and set halloween-treat".into());
                                
                                // Verify it was set
                                if let Ok(Some(value)) = storage.get_item("halloween-treat") {
                                    web_sys::console::log_2(&"Verified halloween-treat set to:".into(), &value.into());
                                } else {
                                    web_sys::console::log_1(&"ERROR: halloween-treat was not set!".into());
                                }

                                // Ensure terminal shows the treat command after a full page reload
                                let _ = storage.set_item("terminal-auto-run", "treat");
                            }
                        }

                        // Perform a short delayed reload to guarantee the MatrixRain re-initializes to pumpkins
                        let reload_timeout = Timeout::new(50, move || {
                            if let Some(win) = window() {
                                let _ = win.location().reload();
                            }
                        });
                        reload_timeout.forget();
                        vec![
                            "🍬 TREAT! 🍭".to_string(),
                            "The ghosts fade away... 👻💨".to_string(),
                            "Pumpkins return to the autumn rain! �".to_string(),
                            "Sweet dreams... if you can sleep! 😈".to_string(),
                        ]
                    },
                    "reset" => {
                        web_sys::console::log_1(&"RESET command executed".into());
                        
                        // Clear all Halloween flags to reset to default
                        if let Some(window) = window() {
                            if let Ok(Some(storage)) = window.local_storage() {
                                let _ = storage.remove_item("halloween-trick");
                                let _ = storage.remove_item("halloween-treat");
                                web_sys::console::log_1(&"Cleared all Halloween flags - reset to default".into());
                            }
                        }
                        vec![
                            "🔄 Reset Complete! 🔄".to_string(),
                            "Halloween effects have been cleared.".to_string(),
                            "Back to the default spooky rain... 🎃".to_string(),
                        ]
                    },
                    "" => vec![],
                    _ => vec![
                        format!("👻 bash: {}: command not found in the haunted realm", command),
                        "Type 'help' to summon the command spirits.".to_string(),
                    ],
                };
                
                new_output.extend(response);
                new_output.push("".to_string());
                
                output.set(new_output);
                input_value.set(String::new());
                
                // Clear the input field
                if let Some(input) = input_ref.cast::<HtmlInputElement>() {
                    input.set_value("");
                }
                
                // Scroll to bottom
                let timeout = Timeout::new(50, move || {
                    if let Some(window) = web_sys::window() {
                        window.scroll_to_with_x_and_y(0.0, window.document().unwrap().body().unwrap().scroll_height() as f64);
                    }
                });
                timeout.forget();
            }
        })
    };

    let on_logout = {
        let auth_state_clone = auth_state.clone();
        Callback::from(move |_| {
            auth_state_clone.set(None);
        })
    };

    // If authenticated, show blog admin instead of terminal
    if let Some(auth) = (*auth_state).as_ref() {
        return html! {
            <BlogAdmin auth_state={auth.clone()} on_logout={on_logout} />
        };
    }

    html! {
        <div class="terminal-container">
            <div class="terminal-header-bar">
                <div class="terminal-controls">
                    <span class="control close"></span>
                    <span class="control minimize"></span>
                    <span class="control maximize"></span>
                </div>
                <div class="terminal-title">{"benjamin@BenjaminNiccum - Terminal"}</div>
            </div>
            
            <div class="terminal-content">
                <div class="terminal-output">
                    {for output.iter().map(|line| html! {
                        <div class="terminal-line">{line}</div>
                    })}
                </div>
                
                <div class="terminal-input-line">
                    <span class="terminal-prompt">{"benjamin@BenjaminNiccum:~$"}</span>
                    <input 
                        ref={input_ref}
                        type={if *waiting_for_password { "password" } else { "text" }}
                        class="terminal-input"
                        value={(*input_value).clone()}
                        oninput={on_input}
                        onkeydown={on_keydown}
                        placeholder={
                            if *waiting_for_password {
                                "Enter password..."
                            } else if *waiting_for_email {
                                "Enter email..."
                            } else {
                                "Type a command..."
                            }
                        }
                    />
                    <span class="terminal-cursor">{"█"}</span>
                </div>
            </div>
        </div>
    }
}
