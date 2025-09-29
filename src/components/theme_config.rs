use wasm_bindgen::prelude::*;
use web_sys::{window, Element};

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

// Get CSS custom property value  
fn get_css_property(property: &str) -> Option<String> {
    let window = window()?;
    let document = window.document()?;
    let document_element = document.document_element()?;
    
    // Use getComputedStyle method from Window
    let computed_style = window
        .get_computed_style_with_pseudo_elt(&document_element, None)
        .ok()??;
    
    let value = computed_style.get_property_value(property).ok()?;
    if value.is_empty() {
        None
    } else {
        // Remove quotes if present
        Some(value.trim_matches('"').trim_matches('\'').to_string())
    }
}

// Load current theme configuration from CSS custom properties
pub fn load_theme_config() -> ThemeConfig {
    ThemeConfig {
        name: get_css_property("--current-theme").unwrap_or_else(|| "retro".to_string()),
        primary_color: get_css_property("--primary-color").unwrap_or_else(|| "#00ff00".to_string()),
        secondary_color: get_css_property("--secondary-color").unwrap_or_else(|| "#00cc00".to_string()),
        bg_color: get_css_property("--bg-color").unwrap_or_else(|| "#0a0a0a".to_string()),
        matrix_chars: get_css_property("--matrix-chars").unwrap_or_else(|| "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrs0123456789".to_string()),
        terminal_prompt: get_css_property("--terminal-prompt").unwrap_or_else(|| "necro@retro-terminal:~$".to_string()),
        welcome_message: get_css_property("--welcome-message").unwrap_or_else(|| "Welcome to the retro terminal...".to_string()),
        header_decoration: get_css_property("--header-decoration").unwrap_or_else(|| "".to_string()),
    }
}