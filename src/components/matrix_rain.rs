use yew::prelude::*;
use gloo::timers::callback::Interval;
use web_sys::{HtmlCanvasElement, window};
use wasm_bindgen::JsCast;
use std::cell::RefCell;
use std::rc::Rc;
use crate::components::theme_config::load_theme_config;

#[derive(Clone, PartialEq, Debug)]
enum ThemeMode {
    Retro,
    Halloween,
    Winter,
    Cyberpunk,
}

#[function_component(MatrixRain)]
pub fn matrix_rain() -> Html {
    let canvas_ref = use_node_ref();
    let theme_config = use_state(|| load_theme_config());
    let mode = use_state(|| {
        let config = load_theme_config();
        match config.name.as_str() {
            "halloween" => ThemeMode::Halloween,
            "winter" => ThemeMode::Winter,
            "cyberpunk" => ThemeMode::Cyberpunk,
            _ => ThemeMode::Retro,
        }
    });
    let restart_token = use_state(|| 0u32);

    // Poll theme changes and localStorage for special effects
    {
        let mode = mode.clone();
        let restart_token = restart_token.clone();
        let theme_config = theme_config.clone();
        use_effect_with((), move |_| {
            let interval = Interval::new(150, move || {
                let new_config = load_theme_config();
                let current_theme_mode = match new_config.name.as_str() {
                    "halloween" => ThemeMode::Halloween,
                    "winter" => ThemeMode::Winter,
                    "cyberpunk" => ThemeMode::Cyberpunk,
                    _ => ThemeMode::Retro,
                };
                
                // Check for special Halloween effects in localStorage
                let mut desired_mode = current_theme_mode.clone();
                if new_config.name == "halloween" {
                    if let Some(win) = window() {
                        if let Ok(Some(storage)) = win.local_storage() {
                            let treat = storage.get_item("halloween-treat").ok().flatten();
                            let trick = storage.get_item("halloween-trick").ok().flatten();
                            
                            // Keep Halloween theme behavior for special effects
                            if treat.is_some() || trick.is_some() {
                                desired_mode = ThemeMode::Halloween;
                            }
                        }
                    }
                }

                if *mode != desired_mode || theme_config.name != new_config.name {
                    mode.set(desired_mode);
                    theme_config.set(new_config);
                    restart_token.set(*restart_token + 1);
                }
            });
            move || drop(interval)
        });
    }

    // (Re)create animation whenever mode or restart_token changes
    {
        let restart_token = *restart_token;
        let current_mode = (*mode).clone();
        let current_config = (*theme_config).clone();
        let canvas_ref = canvas_ref.clone();
        use_effect_with((restart_token, current_mode), move |(_, current_mode)| {
            let cleanup: Rc<RefCell<Option<Interval>>> = Rc::new(RefCell::new(None));

            if let Some(canvas) = canvas_ref.cast::<HtmlCanvasElement>() {
                if let Some(ctx) = canvas
                    .get_context("2d").ok().flatten()
                    .and_then(|c| c.dyn_into::<web_sys::CanvasRenderingContext2d>().ok())
                {
                    // Fit to window
                    if let Some(win) = window() {
                        if let (Ok(w), Ok(h)) = (win.inner_width(), win.inner_height()) {
                            canvas.set_width(w.as_f64().unwrap_or(1280.0) as u32);
                            canvas.set_height(h.as_f64().unwrap_or(720.0) as u32);
                        }
                    }

                    let font_size = 28.0;
                    let spacing = font_size * 0.7;
                    let cols = (canvas.width() as f64 / spacing).max(1.0) as usize;
                    let mut drops: Vec<f64> = (0..cols)
                        .map(|i| (i as f64 * -50.0) + (js_sys::Math::random() * -(canvas.height() as f64)))
                        .collect();

                    let mode_clone = current_mode.clone();
                    let config_clone = current_config.clone();
                    let canvas_clone = canvas.clone();
                    let context_clone = ctx.clone();

                    let int = Interval::new(60, move || {
                        let cw = canvas_clone.width() as f64;
                        let ch = canvas_clone.height() as f64;

                        context_clone.set_global_alpha(1.0);
                        context_clone.clear_rect(0.0, 0.0, cw, ch);

                        context_clone.set_font(&format!("{}px 'Fira Code', monospace", font_size));
                        context_clone.set_shadow_color("transparent");
                        context_clone.set_shadow_blur(0.0);
                        context_clone.set_shadow_offset_x(0.0);
                        context_clone.set_shadow_offset_y(0.0);

                        match mode_clone {
                            ThemeMode::Halloween => {
                                // Check for special Halloween effects
                                let mut use_special = false;
                                if let Some(win) = window() {
                                    if let Ok(Some(storage)) = win.local_storage() {
                                        let treat = storage.get_item("halloween-treat").ok().flatten();
                                        let trick = storage.get_item("halloween-trick").ok().flatten();
                                        use_special = treat.is_some() || trick.is_some();
                                    }
                                }
                                
                                if use_special {
                                    let emojis = ["🎃", "🍂", "🍁", "🎃", "🍂", "🎃"];
                                    for (i, y) in drops.iter_mut().enumerate() {
                                        let x = i as f64 * spacing;
                                        if *y > -50.0 && *y < ch + 50.0 {
                                            let emoji = emojis[i % emojis.len()];
                                            context_clone.set_shadow_color("orange");
                                            context_clone.set_shadow_blur(20.0);
                                            let _ = context_clone.fill_text(emoji, x, *y);
                                        }
                                        *y += 4.5;
                                        if *y > ch + 50.0 { *y = -50.0; }
                                    }
                                } else {
                                    // Use theme matrix characters
                                    let chars: Vec<char> = config_clone.matrix_chars.chars().collect();
                                    for (i, y) in drops.iter_mut().enumerate() {
                                        let x = i as f64 * spacing;
                                        if *y > -50.0 && *y < ch + 50.0 {
                                            let char_idx = ((js_sys::Math::random() * chars.len() as f64) as usize).min(chars.len() - 1);
                                            let ch = chars.get(char_idx).unwrap_or(&'?').to_string();
                                            context_clone.set_fill_style(&wasm_bindgen::JsValue::from(config_clone.primary_color.clone()));
                                            context_clone.set_shadow_color(&config_clone.primary_color);
                                            context_clone.set_shadow_blur(10.0);
                                            let _ = context_clone.fill_text(&ch, x, *y);
                                        }
                                        *y += 4.5;
                                        if *y > ch + 50.0 { *y = -50.0; }
                                    }
                                }
                            }
                            ThemeMode::Winter => {
                                let chars: Vec<char> = config_clone.matrix_chars.chars().collect();
                                for (i, y) in drops.iter_mut().enumerate() {
                                    let x = i as f64 * spacing;
                                    if *y > -50.0 && *y < ch + 50.0 {
                                        let char_idx = ((js_sys::Math::random() * chars.len() as f64) as usize).min(chars.len() - 1);
                                        let ch = chars.get(char_idx).unwrap_or(&'?').to_string();
                                        context_clone.set_fill_style(&wasm_bindgen::JsValue::from(config_clone.primary_color.clone()));
                                        context_clone.set_shadow_color(&config_clone.primary_color);
                                        context_clone.set_shadow_blur(15.0);
                                        let _ = context_clone.fill_text(&ch, x, *y);
                                    }
                                    *y += 2.5; // Slower fall like snow
                                    if *y > ch + 50.0 { *y = -50.0; }
                                }
                            }
                            ThemeMode::Cyberpunk => {
                                let chars: Vec<char> = config_clone.matrix_chars.chars().collect();
                                for (i, y) in drops.iter_mut().enumerate() {
                                    let x = i as f64 * spacing;
                                    if *y > -50.0 && *y < ch + 50.0 {
                                        let char_idx = ((js_sys::Math::random() * chars.len() as f64) as usize).min(chars.len() - 1);
                                        let ch = chars.get(char_idx).unwrap_or(&'?').to_string();
                                        
                                        // Alternate between primary (magenta) and secondary (cyan) colors
                                        let color = if i % 2 == 0 { &config_clone.primary_color } else { &config_clone.secondary_color };
                                        context_clone.set_fill_style(&wasm_bindgen::JsValue::from(color.clone()));
                                        context_clone.set_shadow_color(color);
                                        context_clone.set_shadow_blur(20.0);
                                        let _ = context_clone.fill_text(&ch, x, *y);
                                    }
                                    *y += 6.0; // Faster for cyberpunk intensity
                                    if *y > ch + 50.0 { *y = -50.0; }
                                }
                            }
                            ThemeMode::Retro => {
                                let chars: Vec<char> = config_clone.matrix_chars.chars().collect();
                                for (i, y) in drops.iter_mut().enumerate() {
                                    let x = i as f64 * spacing;
                                    if *y > -50.0 && *y < ch + 50.0 {
                                        let char_idx = ((js_sys::Math::random() * chars.len() as f64) as usize).min(chars.len() - 1);
                                        let ch = chars.get(char_idx).unwrap_or(&'?').to_string();
                                        context_clone.set_fill_style(&wasm_bindgen::JsValue::from(config_clone.primary_color.clone()));
                                        context_clone.set_shadow_color(&config_clone.primary_color);
                                        context_clone.set_shadow_blur(8.0);
                                        let _ = context_clone.fill_text(&ch, x, *y);
                                    }
                                    *y += 4.0; // Classic speed
                                    if *y > ch + 50.0 { *y = -50.0; }
                                }
                            }
                        }
                    });

                    *cleanup.borrow_mut() = Some(int);
                }
            }
            let cleanup_handle = cleanup.clone();
            move || {
                if let Some(i) = cleanup_handle.borrow_mut().take() {
                    drop(i);
                }
            }
        });
    }

    html! { <canvas ref={canvas_ref} class="matrix-bg" /> }
}
