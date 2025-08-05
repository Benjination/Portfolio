# Portfolio Instructions for Copilot

<!-- Use this file to provide workspace-specific custom instructions to Copilot. For more details, visit https://code.visualstudio.com/docs/copilot/copilot-customization#_use-a-githubcopilotinstructionsmd-file -->

This is a Rust + Yew WebAssembly portfolio project with a retro terminal aesthetic.

## Project Structure
- **Language**: Rust with Yew framework
- **Target**: WebAssembly (wasm32-unknown-unknown)
- **Build Tool**: Trunk for development and building
- **Styling**: SCSS with terminal/retro computer theme

## Design Guidelines
- Use terminal/command-line inspired UI elements
- Color scheme: Green (#00ff00) on black (#0a0a0a) terminal theme
- Typography: Monospace fonts (Fira Code, JetBrains Mono)
- Include retro computer science references and ASCII art
- Add subtle CRT/scanline effects for authenticity

## Technical Notes
- All components should be functional components using `#[function_component]`
- Use `yew::prelude::*` for Yew imports
- Leverage `gloo` crate for browser APIs and timers
- WebAssembly-optimized code with minimal external dependencies
- Follow Rust best practices and idiomatic code

## Features
- Matrix rain background effect
- Interactive terminal component
- ASCII art headers
- Git log styled project display
- File listing styled skills section
- CRT monitor effects with scanlines

When implementing new features, maintain the retro terminal aesthetic and ensure WebAssembly compatibility.
