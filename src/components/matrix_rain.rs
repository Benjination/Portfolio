use yew::prelude::*;
use gloo::timers::callback::Interval;
use web_sys::HtmlCanvasElement;
use wasm_bindgen::JsCast;

#[function_component(MatrixRain)]
pub fn matrix_rain() -> Html {
    let canvas_ref = use_node_ref();
    
    use_effect_with(canvas_ref.clone(), move |canvas_ref| {
        let canvas = canvas_ref.cast::<HtmlCanvasElement>().unwrap();
        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        canvas.set_width(1920);
        canvas.set_height(1080);
        
        let chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789@#$%^&*()";
        let font_size = 14.0;
        let columns = (canvas.width() as f64 / font_size) as usize;
        let mut drops: Vec<f64> = vec![1.0; columns];
        
        let interval = Interval::new(50, move || {
            context.set_fill_style(&"rgba(0, 0, 0, 0.05)".into());
            context.fill_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
            
            context.set_fill_style(&"#00ff00".into());
            context.set_font(&format!("{}px 'Fira Code'", font_size));
            
            for i in 0..drops.len() {
                let char_index = (js_sys::Math::random() * chars.len() as f64) as usize;
                let text = &chars[char_index..char_index + 1];
                
                context.fill_text(text, (i as f64) * font_size, drops[i] * font_size).unwrap();
                
                if drops[i] * font_size > canvas.height() as f64 && js_sys::Math::random() > 0.975 {
                    drops[i] = 0.0;
                }
                drops[i] += 1.0;
            }
        });
        
        move || {
            drop(interval);
        }
    });

    html! {
        <canvas 
            ref={canvas_ref}
            class="matrix-bg"
        />
    }
}
