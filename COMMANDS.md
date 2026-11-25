# 🛠️ Kreatives - Command Reference Card

## 🚀 Core Commands

### Development
```bash
# Start development server
cargo run

# Check compilation
cargo check

# Run with logging
RUST_LOG=debug cargo run

# Watch for changes (requires cargo-watch)
cargo watch -x run
```

### Building
```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Clean build directory
cargo clean

# Full rebuild
cargo clean && cargo build --release
```

### Testing
```bash
# Run all tests
cargo test

# Run specific test
cargo test data_loader::tests

# Run with output
cargo test -- --nocapture

# Run tests for a specific module
cargo test --lib data_loader
```

### Maintenance
```bash
# Format code
cargo fmt

# Check code with linter
cargo clippy

# Update dependencies
cargo update

# Generate documentation
cargo doc --open

# Check for outdated packages
cargo outdated
```

---

## 🌐 API Testing

### Using curl
```bash
# Health check
curl http://localhost:3000/health

# Load JSON data
curl http://localhost:3000/api/data/json/site-config.json

# Load TOML data
curl http://localhost:3000/api/data/toml/portfolio.toml

# Load YAML data
curl http://localhost:3000/api/data/yaml/content.yaml

# Pretty print JSON
curl http://localhost:3000/api/data/json/site-config.json | jq .
```

### Using wget
```bash
wget http://localhost:3000/health
wget -O - http://localhost:3000/api/data/json/site-config.json
```

### Using Postman
1. Create new request
2. Method: GET
3. URL: http://localhost:3000/api/data/json/site-config.json
4. Send

---

## 📁 File Management

### Creating Files
```bash
# Create new JSON data file
echo '{"key": "value"}' > data/json/my-data.json

# Create new TOML config
touch data/toml/my-config.toml

# Create new YAML content
touch data/yaml/my-content.yaml

# Create new Rust component
touch frontend/src/components/my-component.rs
```

### Viewing Files
```bash
# View JSON
cat data/json/site-config.json

# View file size
ls -lh data/json/site-config.json

# Count files
find data -type f | wc -l

# List all Rust files
find frontend/src -name "*.rs"
```

### Editing Files
```bash
# View file (nano)
nano data/json/site-config.json

# View file (vim)
vim data/json/site-config.json

# View file (Windows)
type data\json\site-config.json
```

---

## 🔍 Debugging

### Logging
```bash
# Enable all logging
RUST_LOG=trace cargo run

# Debug specific module
RUST_LOG=kreatives_frontend=debug cargo run

# Filter by level
RUST_LOG=info,kreatives_frontend=debug cargo run
```

### Process Management
```bash
# List running processes
ps aux | grep kreatives

# Kill process by PID
kill -9 <PID>

# Kill on port (Linux/Mac)
lsof -ti:3000 | xargs kill -9

# Find process on port (Windows)
netstat -ano | findstr :3000
taskkill /PID <PID> /F
```

### Memory & Performance
```bash
# Monitor while running
top

# Check binary size
ls -lh target/release/kreatives-frontend

# Check memory usage
ps aux | grep kreatives | grep -v grep
```

---

## 🌍 Deployment Commands

### Local Binary
```bash
# Run release binary directly
./target/release/kreatives-frontend

# Run with output redirection
./target/release/kreatives-frontend > app.log 2>&1 &

# Run in background (Unix)
nohup ./target/release/kreatives-frontend &
```

### Docker
```bash
# Build Docker image
docker build -t kreatives:latest .

# Run Docker container
docker run -p 3000:3000 -v $(pwd)/data:/app/data kreatives:latest

# Docker Compose
docker-compose up -d

# Stop container
docker stop <container-id>

# View logs
docker logs <container-id>
```

### Git Operations
```bash
# Check status
git status

# Add changes
git add .

# Commit changes
git commit -m "Update content"

# Push to repository
git push origin main

# Pull latest
git pull origin main

# View log
git log --oneline
```

---

## 🔧 Configuration

### Change Port
Edit `frontend/src/main.rs`:
```rust
let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
    .await?;
```

### Change Bind Address
Edit `frontend/src/main.rs`:
```rust
// Allow external connections
let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
    .await?;
```

### Add Environment Variables
```bash
# Set in terminal
export RUST_LOG=debug
cargo run

# Or use .env file (requires dotenv crate)
echo "RUST_LOG=debug" > .env
```

---

## 📊 Cargo Workspace Commands

### Workspace Operations
```bash
# Build entire workspace
cargo build --workspace

# Run specific package
cargo run -p kreatives-frontend

# Test entire workspace
cargo test --workspace

# Check specific package
cargo check -p kreatives-frontend
```

### Feature Flags
```bash
# Build with specific features
cargo build --features feature1,feature2

# Check available features
cargo metadata --format-version 1 | jq '.packages[].features'
```

---

## 🚨 Troubleshooting Commands

### Common Issues

**Build fails**
```bash
# Clean and rebuild
cargo clean
cargo build

# Check for issues
cargo check
```

**Can't connect to server**
```bash
# Check if server is running
ps aux | grep kreatives

# Check if port is in use
netstat -an | grep 3000

# Kill process on port
lsof -ti:3000 | xargs kill -9
```

**Data not loading**
```bash
# Check file exists
ls -la data/json/

# Check permissions
chmod 644 data/json/site-config.json

# Verify JSON syntax
cat data/json/site-config.json | jq .
```

**Slow builds**
```bash
# Use incremental compilation (default)
cargo build

# Use parallel jobs
cargo build -j 4

# Profile build
cargo build --verbose
```

---

## 📈 Performance Commands

### Build Benchmarks
```bash
# Clean build time
time cargo build --release

# Check build script
cargo build -v

# Profile compilation
cargo build --release -v
```

### Runtime Monitoring
```bash
# Monitor CPU and memory
top -p <PID>

# More detailed stats
htop

# Real-time logs
tail -f /var/log/kreatives.log

# Count requests
grep "GET" /var/log/kreatives.log | wc -l
```

---

## 🔐 Security Commands

### Code Quality
```bash
# Security audit
cargo audit

# Linting
cargo clippy

# Format check
cargo fmt --check

# Full quality check
cargo check && cargo clippy && cargo fmt --check
```

---

## 📝 Documentation Commands

### Generate Docs
```bash
# Generate and open documentation
cargo doc --open

# Generate without opening
cargo doc

# Document private items
cargo doc --document-private-items --open

# Generate for specific package
cargo doc -p kreatives-frontend --open
```

---

## 🎓 Learning Commands

### Project Information
```bash
# Show project structure
cargo tree

# Show dependencies
cargo tree --depth 1

# Show reverse dependencies
cargo tree --invert <dependency>

# Check specific dependency
cargo inspect <dependency>
```

### Version Info
```bash
# Check Rust version
rustc --version

# Check Cargo version
cargo --version

# See toolchain info
rustup show

# Update Rust
rustup update stable
```

---

## 🌐 Network Commands

### Port Testing
```bash
# Check if port is open (Linux)
nc -zv localhost 3000

# Check if port is open (Mac/BSD)
telnet localhost 3000

# More detail
curl -v http://localhost:3000/

# Check response headers
curl -i http://localhost:3000/health

# Time request
time curl http://localhost:3000/health
```

---

## 🚀 Deployment Verification

### Post-Deployment Checklist
```bash
# Check site is running
curl http://your-domain.com/

# Check health endpoint
curl http://your-domain.com/health | jq .

# Load test data
curl http://your-domain.com/api/data/json/site-config.json

# Check performance
ab -n 100 -c 10 http://your-domain.com/

# Monitor logs
ssh user@server
tail -f /var/log/kreatives.log
```

---

## 💾 Backup Commands

### Backup Operations
```bash
# Backup data directory
tar -czf data-backup-$(date +%Y%m%d).tar.gz data/

# Backup entire project
tar -czf kreatives-backup-$(date +%Y%m%d).tar.gz --exclude=target .

# Backup to remote
scp -r data/ user@remote:/backups/

# List backups
ls -lh *backup*.tar.gz
```

---

## 🔄 Maintenance Commands

### Regular Maintenance
```bash
# Update dependencies
cargo update

# Check for security issues
cargo audit

# Clean cache
cargo clean

# Optimize project
cargo build --release

# Check code quality
cargo clippy

# Format code
cargo fmt

# Run all tests
cargo test --all
```

---

## 📚 Quick Reference Table

| Task | Command |
|------|---------|
| Start server | `cargo run` |
| Build release | `cargo build --release` |
| Format code | `cargo fmt` |
| Lint code | `cargo clippy` |
| Run tests | `cargo test` |
| Check compilation | `cargo check` |
| Generate docs | `cargo doc --open` |
| Update deps | `cargo update` |
| Kill port 3000 | `lsof -ti:3000 \| xargs kill -9` |
| Health check | `curl http://localhost:3000/health` |

---

## 🎯 Workflow Examples

### Full Development Cycle
```bash
# 1. Start dev server
cargo run

# 2. In another terminal, make changes

# 3. Test API
curl http://localhost:3000/api/data/json/site-config.json

# 4. Run tests
cargo test

# 5. Format and lint
cargo fmt
cargo clippy

# 6. Build release
cargo build --release

# 7. Deploy
# See DEPLOYMENT.md
```

### Quick Debugging
```bash
# 1. Enable logging
RUST_LOG=debug cargo run

# 2. Test endpoint
curl -v http://localhost:3000/health

# 3. Check logs (in another terminal)
tail -f /var/log/kreatives.log

# 4. Kill and restart
Ctrl+C
cargo run
```

### Deployment Workflow
```bash
# 1. Update files
# Edit data files, components, etc.

# 2. Test locally
cargo run
# Verify at http://localhost:3000

# 3. Build release
cargo build --release

# 4. Push to git
git add .
git commit -m "Update: description"
git push origin main

# 5. Deploy (platform-specific)
# See DEPLOYMENT.md
```

---

**Pro Tip:** Bookmark this page for quick reference during development! 🚀
