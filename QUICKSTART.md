# Quick Start Guide - Kreatives

## 🚀 Getting Started (5 minutes)

### 1. Install Rust (if not already installed)
```bash
# Visit rustup.rs and follow the instructions
# Or on Windows: download from https://rustup.rs/
```

### 2. Build & Run the Project
```bash
# Navigate to project directory
cd kreatives-front

# Run the development server
cargo run
```

The server will start on `http://localhost:3000`

### 3. Test the API
```bash
# In another terminal, test the API:

# Health check
curl http://localhost:3000/health

# Load JSON data
curl http://localhost:3000/api/data/json/site-config.json

# Load TOML data
curl http://localhost:3000/api/data/toml/portfolio.toml

# Load YAML data
curl http://localhost:3000/api/data/yaml/content.yaml
```

## 📁 Project Layout

```
kreatives-front/
├── data/              # Static data files
│   ├── json/         # JSON data files
│   ├── toml/         # TOML configuration
│   └── yaml/         # YAML content
├── frontend/         # Rust/Leptos code
│   ├── src/
│   │   ├── lib.rs           # Leptos components
│   │   ├── main.rs          # Axum server
│   │   ├── data_loader.rs   # Data loading logic
│   │   ├── components/      # Reusable components
│   │   └── pages/           # Page components
│   └── dist/         # Static assets & HTML
└── README.md         # Full documentation
```

## 🛠️ Common Tasks

### Add New JSON Data
```bash
# 1. Create file: data/json/my-data.json
# 2. Add JSON content
# 3. Access via: http://localhost:3000/api/data/json/my-data.json
```

### Add New TOML Configuration
```bash
# 1. Create file: data/toml/my-config.toml
# 2. Add TOML content
# 3. Access via: http://localhost:3000/api/data/toml/my-config.toml
```

### Add New YAML Content
```bash
# 1. Create file: data/yaml/my-content.yaml
# 2. Add YAML content
# 3. Access via: http://localhost:3000/api/data/yaml/my-content.yaml
```

### Create New Page Component
```rust
// 1. Create: frontend/src/pages/my-page.rs
#[component]
pub fn MyPage() -> impl IntoView {
    view! { <div>"My Page"</div> }
}

// 2. Export in: frontend/src/pages/mod.rs
pub mod my_page;
```

### Create New Component
```rust
// 1. Create: frontend/src/components/my-component.rs
#[component]
pub fn MyComponent() -> impl IntoView {
    view! { <div>"My Component"</div> }
}

// 2. Export in: frontend/src/components/mod.rs
pub mod my_component;
```

## 🌍 Deployment

### For Static Hosting (GitHub Pages, Vercel, Netlify):
```bash
# Build release version
cargo build --release

# The binary and dist/ folder are your deployment
# Configure your host to serve dist/ as static files
```

### For Self-Hosted Server:
```bash
# Build
cargo build --release

# Copy to server
scp -r target/release/kreatives-frontend your-server:/app/
scp -r frontend/dist your-server:/app/

# Run on server (configure port as needed)
./kreatives-frontend
```

## 🔧 Troubleshooting

### "Port 3000 already in use"
Edit `frontend/src/main.rs` and change port number, then rebuild

### "Data file not found"
- Ensure files are in `./data/` relative to where you run the binary
- Check file format matches the endpoint

### Build errors
```bash
# Clean rebuild
cargo clean
cargo build
```

### Slow first build
- First build takes longer (1-2 minutes) due to dependency compilation
- Subsequent builds are much faster

## 📚 Next Steps

1. **Customize**: Edit `data/json/site-config.json` with your site info
2. **Style**: Modify `frontend/dist/style.css` to match your brand
3. **Content**: Add more data files to `data/` folders
4. **Components**: Create new components in `frontend/src/components/`
5. **Pages**: Add new pages in `frontend/src/pages/`

## 📖 Learn More

- **Leptos**: https://leptos.dev/
- **Axum**: https://docs.rs/axum/
- **Rust**: https://doc.rust-lang.org/book/
- **Cargo**: https://doc.rust-lang.org/cargo/

## 💡 Tips

1. **Data Caching**: Loaded data is automatically cached in memory
2. **Hot Reload**: For development, use `cargo watch` for auto-rebuild
3. **API Testing**: Use `curl`, Postman, or your browser's developer tools
4. **CSS**: Static CSS in `frontend/dist/style.css` is served automatically
5. **Performance**: Build with `--release` for production

## 🎯 Example: Add a Blog

1. Create `data/json/blog-posts.json`:
```json
{
  "posts": [
    {
      "title": "First Post",
      "date": "2026-11-24",
      "content": "..."
    }
  ]
}
```

2. Access at: `http://localhost:3000/api/data/json/blog-posts.json`

3. Display in component:
```rust
#[component]
pub fn BlogList() -> impl IntoView {
    let data = data_loader::load_data("./data/json/blog-posts.json")
        .unwrap_or_default();
    
    view! { <div>{format!("{:#}", data)}</div> }
}
```

---

**Happy Building! 🚀**

For detailed documentation, see [README.md](./README.md)
