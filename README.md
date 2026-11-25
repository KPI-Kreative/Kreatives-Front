# Kreatives - Static Website with Leptos & Axum

A modern static website built with Leptos (frontend) and Axum (backend), supporting mixed data formats (JSON, TOML, YAML).

## Project Structure

```
kreatives-front/
├── frontend/                 # Main Leptos + Axum project
│   ├── src/
│   │   ├── lib.rs           # Leptos component library
│   │   ├── main.rs          # Axum server
│   │   ├── components/
│   │   │   ├── mod.rs
│   │   │   ├── header.rs
│   │   │   ├── footer.rs
│   │   │   └── data_display.rs
│   │   ├── pages/
│   │   │   ├── mod.rs
│   │   │   └── home.rs
│   │   └── data_loader.rs   # Multi-format data loading
│   ├── dist/                # Static assets (CSS, images)
│   │   └── style.css
│   └── Cargo.toml
├── data/                    # Static data files
│   ├── json/
│   │   └── site-config.json
│   ├── toml/
│   │   └── portfolio.toml
│   └── yaml/
│       └── content.yaml
├── Cargo.toml               # Workspace root
└── README.md
```

## Setup & Installation

### Prerequisites
- Rust 1.70+ (install from [rustup.rs](https://rustup.rs/))
- Cargo (comes with Rust)

### Installation Steps

1. **Clone and navigate to the project:**
   ```bash
   cd Kreatives-Front
   ```

2. **Build the project:**
   ```bash
   cargo build --release
   ```

3. **Run the development server:**
   ```bash
   cargo run
   ```

4. **Access the application:**
   - Main site: http://localhost:3000/
   - API health check: http://localhost:3000/health
   - Load data: http://localhost:3000/api/data/json/site-config.json

## Working with Data

### Supported Formats

#### JSON
Store structured data in JSON format:
```json
{
  "title": "My Page",
  "items": ["item1", "item2"]
}
```

#### TOML
Use TOML for configuration files:
```toml
[site]
title = "My Site"

[[pages]]
name = "Home"
```

#### YAML
Use YAML for content and blog metadata:
```yaml
posts:
  - title: Post 1
    date: 2026-11-20
```

### Loading Data in Components

```rust
use crate::data_loader;
use leptos::*;

#[component]
pub fn MyComponent() -> impl IntoView {
    let data = data_loader::load_data("./data/json/my-data.json")
        .unwrap_or_default();
    
    view! {
        <div>
            {format!("{:#}", data)}
        </div>
    }
}
```

### Data Loader API

```rust
// Auto-detect format and load
let data = data_loader::load_data("./data/json/file.json")?;

// Load specific format
let data = data_loader::load_json("./data/json/file.json")?;
let data = data_loader::load_toml("./data/toml/file.toml")?;
let data = data_loader::load_yaml("./data/yaml/file.yaml")?;

// Cache management
let cached = data_loader::get_cached("key");
data_loader::clear_cache();
```

## Axum API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/` | Homepage |
| GET | `/products` | Products overview page |
| GET | `/demo` | Demo hub page |
| GET | `/extensions` | Extensions demo page |
| GET | `/mobile-apps` | Mobile apps demo page |
| GET | `/data-analytics` | Data analytics demo page |
| GET | `/sample-contract` | Sample contract PDF page |
| GET | `/sample-contract-pdf` | PDF-only sample contract viewer |
| GET | `/health` | Health check with timestamp |
| GET | `/api/data/:path` | Load data file from data folder |

### Example API Calls

```bash
# Health check
curl http://localhost:3000/health

# Load JSON data
curl http://localhost:3000/api/data/json/site-config.json

# Load TOML data
curl http://localhost:3000/api/data/toml/portfolio.toml

# Load YAML data
curl http://localhost:3000/api/data/yaml/content.yaml
```

## Building for Production

### Static Export
To generate a static HTML export:

```bash
cargo build --release
```

The compiled binary can be deployed to any server that supports running Rust applications.

### Deployment Options

#### GitHub Pages
For static hosting on GitHub Pages, you'll need to pre-generate HTML files:

```bash
cargo build --release
# Move dist/ folder contents to docs/ or gh-pages branch
```

#### Vercel / Netlify
1. Configure build command: `cargo build --release`
2. Set output directory to match your build output
3. Deploy!

#### Self-Hosted Server
1. Build the release binary
2. Copy the binary and `dist/` folder to your server
3. Run the binary with appropriate permissions
4. Configure reverse proxy (nginx/Apache) if needed

## Development

### Adding New Pages

1. Create a new file in `frontend/src/pages/`:
   ```rust
   // frontend/src/pages/about.rs
   use leptos::*;
   
   #[component]
   pub fn AboutPage() -> impl IntoView {
       view! { <div>"About Us"</div> }
   }
   ```

2. Export from `frontend/src/pages/mod.rs`:
   ```rust
   pub mod about;
   pub use about::AboutPage;
   ```

3. Add route to main App component in `frontend/src/lib.rs`

### Adding New Components

1. Create component file in `frontend/src/components/`:
   ```rust
   use leptos::*;
   
   #[component]
   pub fn MyComponent() -> impl IntoView {
       view! { <div>"My Component"</div> }
   }
   ```

2. Export from `frontend/src/components/mod.rs`
3. Use in pages or other components

### Adding New Data Files

1. Add data file to appropriate folder:
   - JSON files → `data/json/`
   - TOML files → `data/toml/`
   - YAML files → `data/yaml/`

2. Access via API:
   ```bash
   curl http://localhost:3000/api/data/json/my-file.json
   ```

## Styling

Styles are located in `frontend/dist/style.css`. The design is fully responsive and uses CSS Grid for layouts.

### Key CSS Classes
- `.header` - Header navigation
- `.main-content` - Main page content wrapper
- `.hero` - Hero section
- `.features-grid` - Responsive feature grid
- `.data-display` - Data display containers
- `.footer` - Footer section

## Testing

Run tests with:
```bash
cargo test
```

## Troubleshooting

### Port Already in Use
If port 3000 is in use, modify `main.rs`:
```rust
let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
    .await?;
```

### Data File Not Found
Ensure data files are in the correct path relative to where you run the server:
```
./data/json/file.json
./data/toml/file.toml
./data/yaml/file.yaml
```

### Compilation Issues
Clear and rebuild:
```bash
cargo clean
cargo build
```

## Performance Tips

1. **Data Caching**: The data loader caches loaded files in memory using `once_cell::Lazy`
2. **Compression**: Axum automatically compresses responses with gzip
3. **Static Assets**: Serve CSS and images through a CDN for production
4. **Lazy Loading**: Implement component lazy loading for large pages

## Architecture Decisions

- **Leptos**: Chosen for its Rust-native approach and excellent performance
- **Axum**: Lightweight web framework ideal for static content serving
- **Multi-format Support**: TOML for config, JSON for structured data, YAML for content
- **In-memory Caching**: Improves performance for frequently accessed data

## Contributing

Feel free to extend this template with:
- More page types
- Additional data formats
- Authentication (if needed)
- Database integration (if static isn't enough)
- Build optimizations

## License

MIT License - feel free to use this template for your projects!

## Resources

- [Leptos Documentation](https://leptos.dev/)
- [Axum Documentation](https://docs.rs/axum/)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Cargo Documentation](https://doc.rust-lang.org/cargo/)

## Support

For issues or questions:
1. Check the troubleshooting section
2. Review the example data files
3. Consult the documentation links above
