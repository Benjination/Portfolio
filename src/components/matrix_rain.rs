use yew::prelude::*;
use gloo::timers::callback::Interval;
use web_sys::{HtmlCanvasElement, window};
use wasm_bindgen::JsCast;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, PartialEq, Debug)]
enum HalloweenMode {
    PumpkinRain,
    FloatingGhosts,
}

#[function_component(MatrixRain)]
pub fn matrix_rain() -> Html {
    let canvas_ref = use_node_ref();
    let mode = use_state(|| HalloweenMode::PumpkinRain);
    let restart_token = use_state(|| 0u32);

    // Poll localStorage and update mode. Treat has priority over trick.
    {
        let mode = mode.clone();
        let restart_token = restart_token.clone();
        use_effect_with((), move |_| {
            // Single watcher: prioritize treat over trick, and bump restart token on change
            let interval = Interval::new(150, move || {
                if let Some(win) = window() {
                    if let Ok(Some(storage)) = win.local_storage() {
                        let treat = storage.get_item("halloween-treat").ok().flatten();
                        let trick = storage.get_item("halloween-trick").ok().flatten();

                        let desired = if treat.is_some() {
                            HalloweenMode::PumpkinRain
                        } else if trick.is_some() {
                            HalloweenMode::FloatingGhosts
                        } else {
                            HalloweenMode::PumpkinRain
                        };

                        if *mode != desired {
                            mode.set(desired);
                            restart_token.set(*restart_token + 1);
                        }
                    }
                }
            });
            move || drop(interval)
        });
    }

    // (Re)create animation whenever mode or restart_token changes
    {
        let restart_token = *restart_token;
        let current_mode = (*mode).clone();
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
                            HalloweenMode::PumpkinRain => {
                                let emojis = ["🎃", "🍂", "🍁", "🎃", "🍂", "🎃"];
                                for (i, y) in drops.iter_mut().enumerate() {
                                    let x = i as f64 * spacing;
                                    if *y > -50.0 && *y < ch + 50.0 {
                                        let emoji = emojis[i % emojis.len()];
                                        context_clone.set_shadow_color("orange");
                                        context_clone.set_shadow_blur(20.0);
                                        context_clone.set_shadow_offset_x(0.0);
                                        context_clone.set_shadow_offset_y(-14.0);
                                        let _ = context_clone.fill_text(emoji, x, *y);
                                    }
                                    *y += 4.5;
                                    if *y > ch + 50.0 { *y = -50.0; }
                                }
                            }
                            HalloweenMode::FloatingGhosts => {
                                let entities = ["👻", "💀", "🔮", "👻", "💀", "🔮"];
                                for (i, y) in drops.iter_mut().enumerate() {
                                    let x = i as f64 * spacing;
                                    if *y > -50.0 && *y < ch + 50.0 {
                                        let entity = entities[i % entities.len()];
                                        let wobble = ((*y * 0.01) as f64).sin() * 3.0;
                                        context_clone.set_shadow_color("purple");
                                        context_clone.set_shadow_blur(20.0);
                                        context_clone.set_shadow_offset_x(0.0);
                                        context_clone.set_shadow_offset_y(-14.0);
                                        let _ = context_clone.fill_text(entity, x + wobble, *y);
                                    }
                                    *y += 4.5;
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
