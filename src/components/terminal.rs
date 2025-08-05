use yew::prelude::*;
use web_sys::HtmlInputElement;
use gloo::timers::callback::Timeout;

#[function_component(Terminal)]
pub fn terminal() -> Html {
    let input_ref = use_node_ref();
    let output = use_state(|| vec![
        "Welcome to Benjamin's Portfolio Terminal!".to_string(),
        "Type 'help' for available commands.".to_string(),
        "".to_string(),
    ]);
    let input_value = use_state(|| String::new());
    
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
        
        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                let command = (*input_value).clone();
                let mut new_output = (*output).clone();
                
                new_output.push(format!("benjamin@portfolio:~$ {}", command));
                
                let response = match command.trim() {
                    "help" => vec![
                        "Available commands:".to_string(),
                        "  help        - Show this help message".to_string(),
                        "  about       - Learn about Benjamin".to_string(),
                        "  skills      - View technical skills".to_string(),
                        "  projects    - See recent projects".to_string(),
                        "  contact     - Get contact information".to_string(),
                        "  clear       - Clear terminal".to_string(),
                        "  whoami      - Display user info".to_string(),
                        "  ls          - List portfolio sections".to_string(),
                        "  cat resume  - Display resume summary".to_string(),
                        "  sudo hire   - You know what this does 😉".to_string(),
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
                    "cat resume" => vec![
                        "BENJAMIN NICCUM".to_string(),
                        "Software Engineer | Recent Graduate".to_string(),
                        "".to_string(),
                        "EXPERIENCE:".to_string(),
                        "• Cross-industry project experience".to_string(),
                        "• Mobile app development (iOS/Android)".to_string(),
                        "• Modern web technologies".to_string(),
                        "• Team collaboration and management".to_string(),
                    ],
                    "sudo hire" => vec![
                        "[sudo] password for benjamin: ********".to_string(),
                        "".to_string(),
                        "Access granted! 🎉".to_string(),
                        "Initiating hiring process...".to_string(),
                        "Please contact benjination2@gmail.com to complete.".to_string(),
                    ],
                    "clear" => {
                        output.set(vec![]);
                        input_value.set(String::new());
                        return;
                    },
                    "" => vec![],
                    _ => vec![
                        format!("bash: {}: command not found", command),
                        "Type 'help' for available commands.".to_string(),
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

    html! {
        <div class="terminal-container">
            <div class="terminal-header-bar">
                <div class="terminal-controls">
                    <span class="control close"></span>
                    <span class="control minimize"></span>
                    <span class="control maximize"></span>
                </div>
                <div class="terminal-title">{"benjamin@portfolio - Terminal"}</div>
            </div>
            
            <div class="terminal-content">
                <div class="terminal-output">
                    {for output.iter().map(|line| html! {
                        <div class="terminal-line">{line}</div>
                    })}
                </div>
                
                <div class="terminal-input-line">
                    <span class="terminal-prompt">{"benjamin@portfolio:~$"}</span>
                    <input 
                        ref={input_ref}
                        type="text"
                        class="terminal-input"
                        value={(*input_value).clone()}
                        oninput={on_input}
                        onkeydown={on_keydown}
                        placeholder="Type a command..."
                    />
                    <span class="terminal-cursor">{"█"}</span>
                </div>
            </div>
        </div>
    }
}
