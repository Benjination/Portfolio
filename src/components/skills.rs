use yew::prelude::*;

#[function_component(Skills)]
pub fn skills() -> Html {
    html! {
        <section id="skills" class="terminal-section">
            <div class="section-header">
                <span class="prompt">{"benjamin@portfolio:~$"}</span>
                <span class="command">{"ls -la skills/"}</span>
            </div>
            
            <div class="section-content">
                <pre class="file-listing">
{r#"total 42
drwxr-xr-x  7 benjamin benjamin 4096 Aug  5 2025 .
drwxr-xr-x 12 benjamin benjamin 4096 Aug  5 2025 ..

Programming Languages:
-rw-r--r--  1 benjamin benjamin  ███ Aug  5 2025 rust.rs          [Advanced]
-rw-r--r--  1 benjamin benjamin  ███ Aug  5 2025 c.c              [Proficient]
-rw-r--r--  1 benjamin benjamin  ███ Aug  5 2025 java.java        [Proficient]
-rw-r--r--  1 benjamin benjamin  ███ Aug  5 2025 python.py        [Proficient]
-rw-r--r--  1 benjamin benjamin  ███ Aug  5 2025 cpp.cpp          [Proficient]
-rw-r--r--  1 benjamin benjamin  ███ Aug  5 2025 swift.swift      [Intermediate]
-rw-r--r--  1 benjamin benjamin  ███ Aug  5 2025 assembly.s       [Intermediate]
-rw-r--r--  1 benjamin benjamin  ███ Aug  5 2025 sql.sql          [Proficient]

Web Technologies:
-rw-r--r--  1 benjamin benjamin  ███ Aug  5 2025 html5.html
-rw-r--r--  1 benjamin benjamin  ███ Aug  5 2025 css3.css
-rw-r--r--  1 benjamin benjamin  ███ Aug  5 2025 javascript.js
-rw-r--r--  1 benjamin benjamin  ███ Aug  5 2025 webassembly.wasm  [🔥 Latest]

Frameworks & Tools:
drwxr-xr-x  2 benjamin benjamin 4096 Aug  5 2025 android_studio/
drwxr-xr-x  2 benjamin benjamin 4096 Aug  5 2025 xcode/
drwxr-xr-x  2 benjamin benjamin 4096 Aug  5 2025 git/
drwxr-xr-x  2 benjamin benjamin 4096 Aug  5 2025 firebase/
drwxr-xr-x  2 benjamin benjamin 4096 Aug  5 2025 unix_systems/
drwxr-xr-x  2 benjamin benjamin 4096 Aug  5 2025 jira_confluence/
drwxr-xr-x  2 benjamin benjamin 4096 Aug  5 2025 latex/

Special Interests:
-rwxr-xr-x  1 benjamin benjamin  ███ Aug  5 2025 mobile_development*
-rwxr-xr-x  1 benjamin benjamin  ███ Aug  5 2025 data_structures*
-rwxr-xr-x  1 benjamin benjamin  ███ Aug  5 2025 algorithm_design*
-rwxr-xr-x  1 benjamin benjamin  ███ Aug  5 2025 system_architecture*
"#}
                </pre>
                
                <div class="skill-highlight">
                    <span class="comment">{"# Currently exploring: AI/ML, Advanced Software Design Patterns"}</span>
                </div>
            </div>
        </section>
    }
}
