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
                <div class="about-content">
                    <div class="intro-text">
{r#"👋 Hi there! I'm Benjamin Niccum

I'm a passionate software engineer who loves turning ideas into reality through code. 
Think of me as a digital problem-solver who gets excited about building things that 
make people's lives easier and more enjoyable.

🎯 What drives me:
   • Creating apps and websites that are both beautiful and functional
   • Solving puzzles (whether they're in code or real life!)
   • Learning new technologies and staying on the cutting edge
   • Collaborating with teams to build something amazing

📍 Where I come from:
I'm a recent software engineering graduate based in the US, with hands-on experience 
across different industries - from retail and manufacturing to finance and team 
management. This diverse background helps me understand problems from multiple angles.

🚀 My approach:
I believe the best software starts with understanding what people really need. 
I listen first, then craft solutions using modern tools and best practices. 
My goal? To build digital experiences that feel magical but work reliably.

💭 Philosophy: "The best way to predict the future is to create it."

Ready to collaborate on something incredible? Let's build the future together! ✨"#}
                    </div>
                </div>
            </div>
        </section>
    }
}
