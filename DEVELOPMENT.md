# Development Guide - Kreatives

## Project Architecture

### Technology Stack
- **Frontend**: Leptos (Rust-based web framework)
- **Backend**: Axum (high-performance web server)
- **Data Loading**: Multi-format support (JSON, TOML, YAML)
- **Caching**: In-memory LRU with thread-safe access
- **Compression**: Built-in gzip compression via tower-http

### Directory Structure Explained

```
frontend/
├── src/
│   ├── lib.rs                 # Root Leptos component library
│   ├── main.rs               # Axum server configuration
│   ├── data_loader.rs        # Multi-format data loading module
│   │
│   ├── components/           # Reusable UI components
│   │   ├── mod.rs            # Module exports
│   │   ├── header.rs         # Navigation header
│   │   ├── footer.rs         # Page footer
│   │   └── data_display.rs   # Data visualization components
│   │
│   └── pages/                # Full page components
│       ├── mod.rs            # Module exports
│       └── home.rs           # Home page
│
├── dist/                     # Static assets
│   ├── index.html            # Static HTML fallback
│   ├── style.css             # Global styles
│   └── [images/fonts]        # Place media files here
│
└── Cargo.toml               # Rust dependencies

data/
├── json/                     # JSON data files
│   └── site-config.json      # Site configuration
├── toml/                     # TOML configuration files
│   └── portfolio.toml        # Portfolio/project data
└── yaml/                     # YAML content files
    └── content.yaml          # Blog posts and content
```

## Core Concepts

### 1. Data Loading System

The `data_loader` module provides:
- **Format Detection**: Automatically detects file type from extension
- **Caching**: In-memory cache prevents repeated file reads
- **Error Handling**: Comprehensive error reporting

**Usage Example:**
```rust
use crate::data_loader;

// Auto-detect format
let data = data_loader::load_data("./data/json/my-file.json")?;

// Or load specific format
let json = data_loader::load_json("./data/json/file.json")?;
let toml = data_loader::load_toml("./data/toml/file.toml")?;
let yaml = data_loader::load_yaml("./data/yaml/file.yaml")?;

// Check cache without disk I/O
if let Some(cached) = data_loader::get_cached("key") {
    // Use cached data
}
```

### 2. Component System

Built on Leptos `#[component]` macro:
- **Declarative**: View syntax similar to JSX
- **Reactive**: Automatic re-rendering when data changes
- **Type-Safe**: Compile-time checking for props and state

**Component Template:**
```rust
use leptos::*;

#[component]
pub fn MyComponent(
    #[prop(default = "default value")] title: String,
) -> impl IntoView {
    view! {
        <div class="my-component">
            <h1>{title}</h1>
        </div>
    }
}
```

### 3. Server Routes (Axum)

Routes are defined in `main.rs`:
- `/` - Main page
- `/health` - Health check endpoint
- `/api/data/:path` - Load data files dynamically

**Route Structure:**
```rust
Router::new()
    .route("/", get(index))
    .route("/api/data/:path", get(get_data))
    .route("/health", get(health_check))
    .layer(CompressionLayer::new())
```

## Working with Data

### JSON Data Files

**File**: `data/json/site-config.json`
```json
{
  "site_title": "My Site",
  "site_description": "Description here",
  "features": [
    {
      "name": "Feature 1",
      "description": "Feature description"
    }
  ]
}
```

**Load in Component:**
```rust
#[component]
pub fn SiteInfo() -> impl IntoView {
    let data = data_loader::load_data("./data/json/site-config.json")
        .unwrap_or_default();
    
    view! {
        <div>
            {data.get("site_title").and_then(|t| t.as_str())
                .unwrap_or("Site")}
        </div>
    }
}
```

### TOML Configuration Files

**File**: `data/toml/settings.toml`
```toml
[site]
title = "My Site"
theme = "dark"

[performance]
cache_duration_minutes = 60
max_items_per_page = 20

[[sections]]
name = "Home"
path = "/"

[[sections]]
name = "About"
path = "/about"
```

**Access Fields:**
```rust
let settings = data_loader::load_toml("./data/toml/settings.toml")?;
let title = settings["site"]["title"].as_str()?;
let theme = settings["site"]["theme"].as_str()?;
```

### YAML Content Files

**File**: `data/yaml/blog.yaml`
```yaml
posts:
  - title: Post 1
    slug: post-1
    date: 2026-11-20
    tags: [rust, web]
    content: Post content here
    
  - title: Post 2
    slug: post-2
    date: 2026-11-18
    tags: [performance]
    content: More content
```

**Display in Component:**
```rust
#[component]
pub fn BlogList() -> impl IntoView {
    let data = data_loader::load_data("./data/yaml/blog.yaml")
        .unwrap_or_default();
    
    if let Some(posts) = data["posts"].as_array() {
        view! {
            <ul>
                {posts.iter().map(|post| {
                    let title = post["title"].as_str().unwrap_or("Untitled");
                    view! { <li>{title}</li> }
                }).collect::<Vec<_>>()}
            </ul>
        }
    } else {
        view! { <div>No posts found</div> }
    }
}
```

## Building Components

### Stateless Components (Recommended)

```rust
#[component]
pub fn Card(
    #[prop(default = "Title".to_string())] title: String,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="card">
            <div class="card-title">{title}</div>
            <div class="card-content">
                {children()}
            </div>
        </div>
    }
}

// Usage:
<Card title="Hello">
    <p>"Card content"</p>
</Card>
```

### Components with Reactive State

```rust
#[component]
pub fn Counter() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    
    view! {
        <div>
            <p>"Count: " {count}</p>
            <button on:click=move |_| {
                set_count.update(|c| *c += 1)
            }>
                "Increment"
            </button>
        </div>
    }
}
```

### Components with Effects

```rust
#[component]
pub fn DataDisplay() -> impl IntoView {
    let (data, set_data) = create_signal(Value::Null);
    
    // Load data on mount
    create_effect(move |_| {
        if let Ok(loaded) = data_loader::load_data("./data/json/file.json") {
            set_data(loaded);
        }
    });
    
    view! {
        <div class="data-display">
            {data.with(|d| format!("{:#}", d))}
        </div>
    }
}
```

## Styling Guide

### CSS Structure

The main stylesheet is `frontend/dist/style.css` with:
- **Global Styles**: Basic typography and layout
- **Component Styles**: Modular CSS classes
- **Responsive Design**: Mobile-first approach
- **Utilities**: Reusable class patterns

### Adding Styles to Components

```rust
#[component]
pub fn StyledCard(title: String) -> impl IntoView {
    view! {
        <div class="card">
            <div class="card-header">
                <h3 class="card-title">{title}</h3>
            </div>
            <div class="card-body">
                // Content here
            </div>
        </div>
    }
}
```

**Corresponding CSS:**
```css
.card {
    background-color: white;
    border-radius: 8px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.card-header {
    padding: 1rem;
    border-bottom: 1px solid #eee;
}

.card-title {
    margin: 0;
    font-size: 1.3rem;
}

.card-body {
    padding: 1rem;
}
```

## Development Workflow

### 1. Local Development

```bash
# Start the dev server
cargo run

# In another terminal, watch for changes (requires cargo-watch)
cargo watch -x run
```

Access at `http://localhost:3000`

### 2. Hot Reload (Advanced)

Install cargo-watch for auto-rebuild:
```bash
cargo install cargo-watch
cargo watch -x run
```

### 3. Testing

```bash
# Run all tests
cargo test

# Run specific test
cargo test data_loader::tests

# Run with output
cargo test -- --nocapture
```

### 4. Build for Production

```bash
# Release build
cargo build --release

# Binary location
./target/release/kreatives-frontend

# Run production build
./target/release/kreatives-frontend
```

## Performance Optimization

### 1. Data Caching

Data is automatically cached after first load:
```rust
// First call: loads from disk
let data = data_loader::load_data("./data/json/file.json")?;

// Second call: returns from cache (instant)
let data = data_loader::load_data("./data/json/file.json")?;

// Clear cache if needed
data_loader::clear_cache();
```

### 2. Component Memoization

Prevent unnecessary re-renders:
```rust
#[component]
pub fn ExpensiveComponent(
    #[prop(eq)] data: String,
) -> impl IntoView {
    // Only re-renders if data changes (structural equality)
    view! { <div>{data}</div> }
}
```

### 3. Lazy Loading

```rust
use leptos::*;

#[component]
pub fn LazyLoadedComponent() -> impl IntoView {
    let (visible, set_visible) = create_signal(false);
    
    view! {
        <div>
            {move || {
                if visible.get() {
                    view! { <div>"Lazy loaded content"</div> }
                } else {
                    view! { <div>"Loading..."</div> }
                }
            }}
        </div>
    }
}
```

## Error Handling

### In Data Loader

```rust
match data_loader::load_data(&path) {
    Ok(data) => {
        // Use data
    }
    Err(e) => {
        eprintln!("Failed to load data: {}", e);
        // Handle error gracefully
    }
}
```

### In Components

```rust
#[component]
pub fn SafeDataDisplay() -> impl IntoView {
    let data = data_loader::load_data("./data/json/file.json")
        .unwrap_or_else(|_| json!({"error": "Failed to load data"}));
    
    view! {
        <div>
            {format!("{:#}", data)}
        </div>
    }
}
```

## Deployment Checklist

Before deploying:
- [ ] Build succeeds: `cargo build --release`
- [ ] No compiler warnings
- [ ] All tests pass: `cargo test`
- [ ] Data files are correct and accessible
- [ ] CSS and assets are in `frontend/dist/`
- [ ] Update configuration in data files
- [ ] Test API endpoints locally
- [ ] Review environment-specific settings

## Useful Commands

```bash
# Check without compiling
cargo check

# Format code
cargo fmt

# Lint code
cargo clippy

# Generate documentation
cargo doc --open

# Clean build artifacts
cargo clean

# Update dependencies
cargo update

# Check outdated dependencies
cargo outdated
```

## Troubleshooting

### Compilation Errors

```bash
# Clean and rebuild
cargo clean
cargo build

# Check for issues
cargo check
```

### Runtime Errors

```bash
# Enable logging
RUST_LOG=debug cargo run

# Check data file paths
# Ensure relative paths work from binary location
```

### Performance Issues

```bash
# Profile with flamegraph (requires cargo-flamegraph)
cargo install flamegraph
cargo flamegraph

# Check binary size
cargo build --release
ls -lh target/release/kreatives-frontend
```

---

For more information:
- Leptos Docs: https://leptos.dev/
- Axum Docs: https://docs.rs/axum/
- Rust Book: https://doc.rust-lang.org/book/
