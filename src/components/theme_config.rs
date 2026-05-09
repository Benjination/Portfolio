use web_sys::window;

// Theme configuration structure
#[derive(Clone, Debug)]
pub struct ThemeConfig {
    pub name: String,
    pub primary_color: String,
    pub secondary_color: String,
    pub bg_color: String,
    pub matrix_chars: String,
    pub terminal_prompt: String,
    pub welcome_message: String,
    pub header_decoration: String,
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {
            name: "retro".to_string(),
            primary_color: "#00ff00".to_string(),
            secondary_color: "#00cc00".to_string(),
            bg_color: "#0a0a0a".to_string(),
            matrix_chars: "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrs0123456789".to_string(),
            terminal_prompt: "necro@retro-terminal:~$".to_string(),
            welcome_message: "Welcome to the retro terminal...".to_string(),
            header_decoration: "".to_string(),
        }
    }
}

// Load current theme configuration from CSS custom properties
pub fn load_theme_config() -> ThemeConfig {
    // For now, hard-code retro theme to get it working
    // TODO: Read from CSS custom properties once web-sys method is available
    ThemeConfig {
        name: "retro".to_string(),
        primary_color: "#00ff00".to_string(),
        secondary_color: "#00cc00".to_string(),
        bg_color: "#0a0a0a".to_string(),
        matrix_chars: "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrs0123456789".to_string(),
        terminal_prompt: "necro@retro-terminal:~$".to_string(),
        welcome_message: "Welcome to the retro terminal...".to_string(),
        header_decoration: "".to_string(),
    }
}