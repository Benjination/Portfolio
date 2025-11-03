use yew::prelude::*;

#[function_component(Header)]
pub fn header() -> Html {
    html! {
        <header class="terminal-header">
            <div class="ascii-art">
                <pre class="ascii-name">
{r#"
    ⚡  ██████╗ ███████╗███╗   ██╗     ███╗   ██╗██╗ ██████╗  ██████╗██╗   ██╗███╗   ███╗  �
   🤖   ██╔══██╗██╔════╝████╗  ██║     ████╗  ██║██║██╔════╝██╔════╝██║   ██║████╗ ████║   💾
  ⚡    ██████╔╝█████╗  ██╔██╗ ██║     ██╔██╗ ██║██║██║     ██║     ██║   ██║██╔████╔██║    �
 🤖     ██╔══██╗██╔══╝  ██║╚██╗██║     ██║╚██╗██║██║██║     ██║     ██║   ██║██║╚██╔╝██║     �
⚡      ██████╔╝███████╗██║ ╚████║     ██║ ╚████║██║╚██████╗╚██████╗╚██████╔╝██║ ╚═╝ ██║      🔥
        ╚═════╝ ╚══════╝╚═╝  ╚═══╝     ╚═╝  ╚═══╝╚═╝ ╚═════╝ ╚═════╝ ╚═════╝ ╚═╝     ╚═╝        
                                                                                    
                      ⚡ CYBERPUNK NEURAL EDITION ⚡ BenjaminNiccum.com 🤖
"#}
                </pre>
            </div>
            
            <div class="terminal-prompt">
                <span class="prompt-user">{"⚡benjamin@CyberNet💾"}</span>
                <span class="prompt-separator">{":"}</span>
                <span class="prompt-path">{"~/neural-link"}</span>
                <span class="prompt-symbol">{"🤖"}</span>
                <span class="cursor">{"🔥"}</span>
            </div>
            
            <div class="tagline">
                <span class="comment">{"# Software Engineer | Code Enthusiast | Problem Solver"}</span>
            </div>
            
            <nav class="terminal-nav">
                <ul>
                    <li><a href="#about" class="nav-command">{"./about.sh"}</a></li>
                    <li><a href="#skills" class="nav-command">{"./skills.sh"}</a></li>
                    <li><a href="#projects" class="nav-command">{"./projects.sh"}</a></li>
                    <li><a href="#snake-game" class="nav-command">{"./snake.exe"}</a></li>
                    <li><a href="#leaderboard" class="nav-command">{"./highscores.db"}</a></li>
                    <li><a href="#contact" class="nav-command">{"./contact.sh"}</a></li>
                    <li><a href="https://github.com/Benjination" class="nav-command">{"cat github.txt"}</a></li>
                </ul>
            </nav>
        </header>
    }
}
