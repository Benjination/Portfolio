use yew::prelude::*;

#[function_component(About)]
pub fn about() -> Html {
    html! {
        <section id="about" class="terminal-section">
            <div class="section-header">
                <span class="prompt">{"benjamin@BenjaminNiccum:~$"}</span>
                <span class="command">{"cat about.md"}</span>
            </div>
            
            <div class="section-content">
                <pre class="code-block">
{r#"## About Benjamin Niccum

```rust
struct SoftwareEngineer {
    name: String,
    location: String,
    education: String,
    passion: Vec<String>,
    current_focus: String,
}

impl SoftwareEngineer {
    fn new() -> Self {
        Self {
            name: "Benjamin Niccum".to_string(),
            location: "Based in the US".to_string(),
            education: "Recent Software Engineering Graduate".to_string(),
            passion: vec![
                "Building intuitive applications".to_string(),
                "Solving complex problems".to_string(),
                "Learning cutting-edge technologies".to_string(),
                "Creating digital magic ✨".to_string(),
            ],
            current_focus: "Seeking opportunities to innovate and grow".to_string(),
        }
    }
    
    fn get_philosophy(&self) -> &str {
        "The best way to predict the future is to create it."
    }
}
```

💻 **Background:** Recent software engineering graduate with hands-on experience 
   across diverse industries including retail, manufacturing, finance, and team 
   management.

🚀 **Approach:** I start by understanding your unique challenges, then propose 
   practical solutions using modern development practices and cutting-edge tools.

🎯 **Goal:** Ready to collaborate on innovative projects that drive business 
   forward and create meaningful impact.
"#}
                </pre>
            </div>
        </section>
    }
}
