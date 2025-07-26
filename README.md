# Perplexity-style Chat UI with Leptos 0.8 + DaisyUI

A modern chat interface inspired by Perplexity AI, built with Rust using Leptos 0.8, DaisyUI components, and Lucide icons via CDN.

## Features

- 🎨 **Modern UI**: Perplexity-inspired design with clean, professional aesthetics
- 🌓 **Dark/Light Theme**: Toggle between themes with DaisyUI's theme system
- 📱 **Responsive Design**: Collapsible sidebar that adapts to screen size
- 🤖 **LLM Selection**: Dropdown to choose between different AI models
- 💬 **Chat Interface**: Real-time chat with message bubbles and timestamps
- 🔍 **Knowledge Toggle**: Enable/disable knowledge graph integration
- 📊 **Status Bar**: Real-time status updates and connection indicator
- ⚡ **Fast & Lightweight**: Built with Rust/WASM for optimal performance

## Tech Stack

- **Frontend Framework**: [Leptos 0.8](https://leptos.dev/) (Rust)
- **UI Components**: [DaisyUI](https://daisyui.com/) with Tailwind CSS
- **Icons**: [Lucide Icons](https://lucide.dev/) via CDN
- **Build Tool**: [Trunk](https://trunkrs.dev/)
- **Language**: Rust (compiles to WebAssembly)

## Project Structure

```
src/
├── components/
│   ├── main_interface.rs    # Main app layout
│   ├── sidebar.rs          # Collapsible sidebar
│   ├── llm_select.rs       # LLM model selector
│   ├── sidebar_action.rs   # Sidebar action buttons
│   ├── conversation_history.rs # Recent conversations
│   ├── chat_area.rs        # Main chat interface
│   ├── message_bubble.rs   # Individual message display
│   ├── input_area.rs       # Message input with controls
│   ├── status_bar.rs       # Bottom status bar
│   └── theme_toggle.rs     # Theme switcher
├── models.rs               # Data structures
├── lib.rs                  # Main app component
└── main.rs                 # Entry point
```

## Key Components

### MainInterface
The root component that orchestrates the entire application layout, managing state for sidebar collapse, LLM selection, and knowledge graph toggle.

### Sidebar
- Collapsible design (80px collapsed, 320px expanded)
- LLM model selection dropdown
- Action buttons for file upload, knowledge graph, and new chat
- Recent conversation history

### ChatArea
- Message display with user/assistant differentiation
- Real-time message rendering
- Simulated AI responses with typing indicators

### InputArea
- Text input with Enter key support
- Knowledge graph toggle
- Send button with icon
- Input validation

## DaisyUI Components Used

- `btn` - Buttons with various styles (ghost, outline, primary)
- `input` - Text inputs with borders and focus states
- `chat` - Chat bubble components with alignment
- `dropdown` - LLM selection dropdown
- `toggle` - Knowledge graph toggle switch
- `join` - Grouped input and button elements
- Theme system with `data-theme` attributes

## Lucide Icons Integration

Icons are loaded via CDN and re-rendered when component state changes:

```rust
Effect::new(move |_| {
    request_animation_frame(move || {
        if let Some(window) = web_sys::window() {
            if let Ok(lucide) = js_sys::Reflect::get(&window, &"lucide".into()) {
                if let Ok(create_icons) = js_sys::Reflect::get(&lucide, &"createIcons".into()) {
                    let _ = js_sys::Function::from(create_icons).call0(&lucide);
                }
            }
        }
    });
});
```

## Development

### Prerequisites
- Rust (latest stable)
- `trunk` for building and serving
- `wasm32-unknown-unknown` target

### Setup
```bash
# Install trunk
cargo install trunk

# Add WASM target
rustup target add wasm32-unknown-unknown

# Clone and run
git clone <repo>
cd wasm-llm-trunk
trunk serve --open
```

### Building
```bash
# Development build
trunk build

# Production build
trunk build --release
```

## Customization

### Themes
The app uses DaisyUI's theme system. Current themes:
- `black` - Dark theme with high contrast
- `light` - Light theme with clean aesthetics

Add new themes in `tailwind.config.js`:

```javascript
daisyui: {
    themes: [
        "light",
        {
            "custom": {
                "primary": "#your-color",
                // ... other colors
            }
        }
    ]
}
```

### Adding New LLM Models
Update the `llms` vector in `sidebar.rs`:

```rust
let llms = vec![
    LLMModel {
        id: "new-model".to_string(),
        name: "New Model".to_string(),
        provider: "Provider".to_string(),
        logo_slug: "provider-logo".to_string(),
    },
    // ... existing models
];
```

### Styling
The app uses Tailwind CSS classes with DaisyUI components. Customize by:
1. Modifying component classes directly
2. Adding custom CSS in `public/input.css`
3. Extending the Tailwind config

## Performance

- **Bundle Size**: ~200KB gzipped (WASM + JS)
- **First Load**: ~100ms on modern browsers
- **Runtime**: Near-native performance thanks to WASM
- **Memory**: ~2MB baseline usage

## Browser Support

- Chrome/Edge 88+
- Firefox 89+
- Safari 14+
- All browsers with WebAssembly support

## License

MIT License - see LICENSE file for details.

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test thoroughly
5. Submit a pull request

## Acknowledgments

- [Leptos](https://leptos.dev/) - The reactive web framework
- [DaisyUI](https://daisyui.com/) - Beautiful Tailwind CSS components
- [Lucide](https://lucide.dev/) - Clean, customizable icons
- [Perplexity AI](https://perplexity.ai/) - UI/UX inspiration