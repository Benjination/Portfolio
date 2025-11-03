# 🦀 Benjamin Niccum's Portfolio
Remember this: npm regenerate-blogs

A modern, retro-styled portfolio website built with **Rust + Yew** and compiled to **WebAssembly** for maximum performance and that "wow factor" that other developers will appreciate.

## 🚀 Tech Stack

- **Frontend**: Rust + Yew Framework
- **Target**: WebAssembly (WASM)
- **Build Tool**: Trunk
- **Styling**: SCSS with terminal aesthetics
- **Performance**: Zero JavaScript framework overhead

## 🎨 Design Features

- **Matrix Rain Background**: Animated green code rain effect
- **Terminal Interface**: Authentic command-line aesthetic
- **ASCII Art**: Custom generated headers and branding
- **CRT Effects**: Subtle scanlines and glow effects
- **Interactive Terminal**: Working command-line interface
- **Git-styled Project Display**: Projects shown as git log
- **File Listing Skills**: Unix ls-style skills presentation

## 🛠️ Development

### Prerequisites
- Rust (latest stable)
- Trunk (`cargo install trunk`)
- WASM target (`rustup target add wasm32-unknown-unknown`)

### Quick Start

```bash
# Clone the repository
git clone https://github.com/Benjination/Portfolio.git
cd Portfolio

# Install dependencies (automatically handled by Cargo)
# Start development server with hot reload
trunk serve

# Build for production
trunk build --release
```

### Project Structure

```
src/
├── main.rs              # App entry point
├── components/
│   ├── mod.rs           # Component exports
│   ├── header.rs        # ASCII art header with nav
│   ├── about.rs         # About section with Rust code
│   ├── skills.rs        # Skills as file listing
│   ├── projects.rs      # Projects as git log
│   ├── contact.rs       # Contact info as man page
│   ├── terminal.rs      # Interactive terminal
│   └── matrix_rain.rs   # Background matrix effect
styles/
└── main.scss            # Terminal-themed styling
```

## 🎯 Features

### Interactive Terminal
- Type `help` for available commands
- Fully functional command-line interface
- Custom responses for portfolio navigation
- Easter eggs for fellow developers

### Performance
- **WebAssembly**: Near-native performance
- **Small Bundle Size**: Rust's zero-cost abstractions
- **Memory Safe**: No runtime errors from memory issues
- **Fast Loading**: Optimized WASM compilation

### Developer Experience
- **Hot Reload**: Instant updates during development
- **Type Safety**: Rust's compile-time guarantees
- **Modern Tooling**: Cargo, Trunk, and Rust ecosystem

## 🌐 Deployment

The site is optimized for deployment on:
- **GitHub Pages** (static hosting)
- **Netlify** (automatic deployments)
- **Vercel** (serverless hosting)
- **Any static host** (just serve the `dist/` folder)

## 📝 About

This portfolio showcases Benjamin Niccum's software engineering skills with a focus on:
- Modern systems programming with Rust
- WebAssembly applications
- Creative UI/UX design
- Performance optimization
- Developer experience

## 🤝 Contributing

While this is a personal portfolio, suggestions and improvements are welcome! Feel free to:
- Open issues for bugs or suggestions
- Submit PRs for improvements
- Use this as inspiration for your own Rust+WASM projects

## 📄 License

MIT License - feel free to use this code for your own portfolio!

---

*Built with ❤️ and ☕ by Benjamin Niccum*

**"The best way to predict the future is to create it."** - Abraham Lincoln
# Force rebuild Wed Sep 17 20:04:56 CDT 2025
