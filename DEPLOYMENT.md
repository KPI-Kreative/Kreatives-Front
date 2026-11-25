# Deployment Guide - Kreatives

## Deployment Options

### 1. Static Hosting (Recommended for most users)

Perfect for: Blog, portfolio, documentation sites

**Supported Platforms:**
- GitHub Pages
- Vercel
- Netlify
- Cloudflare Pages
- AWS Amplify

**Benefits:**
- Fast CDN delivery
- Automatic HTTPS
- Simple deployment
- Free tier available
- No server maintenance

**Limitations:**
- Dynamic APIs require separate backend
- Serverless functions needed for server-side logic

### 2. VPS/Cloud Server

Perfect for: Custom backends, real-time features

**Supported Platforms:**
- AWS EC2
- DigitalOcean
- Linode
- Google Cloud
- Azure VMs

**Benefits:**
- Full control
- Run any code
- Custom domains
- Scalable

**Requires:**
- Server maintenance
- SSL certificate setup
- Database management

### 3. Containerized Deployment

Perfect for: Kubernetes, Docker Compose setups

**Docker Support:**
- Build container
- Push to registry
- Deploy with orchestration

---

## 1️⃣ GitHub Pages Deployment

### Prerequisites
- GitHub account
- Repository with this code
- No backend services needed (static only)

### Setup Steps

**Step 1: Build Release Binary**
```bash
cargo build --release
```

**Step 2: Extract Static Assets**
```bash
# Copy these to your gh-pages branch:
# - frontend/dist/ (HTML, CSS)
# - Rename index.html if needed
```

**Step 3: Configure GitHub Pages**
1. Go to Repository Settings → Pages
2. Source: Deploy from branch
3. Branch: `gh-pages`
4. Folder: `/root`
5. Save

**Step 4: Deploy**
```bash
# Option A: Using GitHub Actions (Recommended)
# Create .github/workflows/deploy.yml:

name: Deploy to GitHub Pages
on:
  push:
    branches: [main]

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - name: Build
        run: cargo build --release
      - name: Deploy
        uses: peaceiris/actions-gh-pages@v3
        with:
          github_token: ${{ secrets.GITHUB_TOKEN }}
          publish_dir: ./frontend/dist
```

**Step 5: Monitor Deployment**
- Check "Actions" tab in GitHub
- Site published at: `https://username.github.io/repository-name`

---

## 2️⃣ Vercel Deployment

### Prerequisites
- Vercel account (free)
- GitHub repository
- Node.js runtime for build

### Setup Steps

**Step 1: Connect Repository**
1. Go to vercel.com
2. Click "New Project"
3. Import your GitHub repository
4. Select `frontend` directory as root

**Step 2: Configure Build Settings**
- Framework: "Other"
- Build Command: `cargo build --release`
- Output Directory: `frontend/dist`

**Step 3: Environment Variables**
- No variables needed for static site

**Step 4: Deploy**
```bash
# Vercel CLI (optional)
npm i -g vercel
vercel
```

**Step 5: Configure Domain**
- Settings → Domains
- Add custom domain or use vercel.app URL

---

## 3️⃣ Netlify Deployment

### Prerequisites
- Netlify account (free)
- GitHub repository

### Setup Steps

**Step 1: Connect Git**
1. Go to netlify.com
2. Click "New site from Git"
3. Choose GitHub provider
4. Select your repository

**Step 2: Build Settings**
- Base directory: `frontend`
- Build command: `cargo build --release`
- Publish directory: `frontend/dist`

**Step 3: Deploy**
Netlify automatically deploys on git push

**Step 4: Custom Domain**
- Settings → Domain management
- Add custom domain

---

## 4️⃣ DigitalOcean Droplet

### Prerequisites
- DigitalOcean account
- SSH access to droplet
- Ubuntu 20.04+ Droplet ($4-5/month)

### Setup Steps

**Step 1: Create Droplet**
```bash
# Create Ubuntu 22.04 Droplet
# Size: Basic ($4-6/month)
# Region: Choose closest to users
```

**Step 2: Connect and Setup**
```bash
# SSH into droplet
ssh root@your-droplet-ip

# Update system
apt update && apt upgrade -y

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Install git
apt install -y git
```

**Step 3: Clone and Build**
```bash
# Clone repository
git clone https://github.com/your-username/kreatives-front.git
cd kreatives-front

# Build release binary
cargo build --release
```

**Step 4: Setup Systemd Service**
```bash
# Create service file
sudo tee /etc/systemd/system/kreatives.service > /dev/null <<EOF
[Unit]
Description=Kreatives Web Server
After=network.target

[Service]
Type=simple
User=root
WorkingDirectory=/root/kreatives-front
ExecStart=/root/kreatives-front/target/release/kreatives-frontend
Restart=always
RestartSec=10
StandardOutput=append:/var/log/kreatives.log
StandardError=append:/var/log/kreatives-error.log

[Install]
WantedBy=multi-user.target
EOF

# Enable and start service
sudo systemctl enable kreatives
sudo systemctl start kreatives
sudo systemctl status kreatives
```

**Step 5: Setup Nginx Reverse Proxy**
```bash
# Install Nginx
sudo apt install -y nginx

# Create configuration
sudo tee /etc/nginx/sites-available/kreatives > /dev/null <<EOF
server {
    listen 80;
    server_name your-domain.com;

    location / {
        proxy_pass http://localhost:3000;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_cache_bypass $http_upgrade;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
EOF

# Enable site
sudo ln -s /etc/nginx/sites-available/kreatives /etc/nginx/sites-enabled/
sudo nginx -t
sudo systemctl restart nginx
```

**Step 6: Setup SSL (Recommended)**
```bash
# Install Certbot
sudo apt install -y certbot python3-certbot-nginx

# Get certificate
sudo certbot --nginx -d your-domain.com

# Auto-renewal (already enabled by certbot)
sudo systemctl status certbot.timer
```

**Step 7: Monitor**
```bash
# Check service logs
sudo journalctl -u kreatives -f

# Check Nginx logs
sudo tail -f /var/log/nginx/access.log
sudo tail -f /var/log/nginx/error.log

# Check application logs
sudo tail -f /var/log/kreatives.log
```

---

## 5️⃣ Docker Deployment

### Prerequisites
- Docker installed
- Docker registry account (optional)

### Dockerfile

```dockerfile
# Build stage
FROM rust:latest as builder

WORKDIR /app
COPY . .

RUN cargo build --release

# Runtime stage
FROM ubuntu:22.04

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/kreatives-frontend /app/kreatives-frontend
COPY --from=builder /app/frontend/dist /app/frontend/dist
COPY --from=builder /app/data /app/data

WORKDIR /app

EXPOSE 3000
CMD ["./kreatives-frontend"]
```

**Build and Run:**
```bash
# Build image
docker build -t kreatives:latest .

# Run container
docker run -p 3000:3000 -v $(pwd)/data:/app/data kreatives:latest

# Push to registry
docker tag kreatives:latest your-registry/kreatives:latest
docker push your-registry/kreatives:latest
```

### Docker Compose

```yaml
# docker-compose.yml
version: '3.8'

services:
  kreatives:
    build: .
    ports:
      - "3000:3000"
    volumes:
      - ./data:/app/data
    restart: always
    environment:
      RUST_LOG: info

  nginx:
    image: nginx:alpine
    ports:
      - "80:80"
      - "443:443"
    volumes:
      - ./nginx.conf:/etc/nginx/nginx.conf:ro
      - ./ssl:/etc/nginx/ssl:ro
    depends_on:
      - kreatives
    restart: always
```

**Run:**
```bash
docker-compose up -d
```

---

## 6️⃣ AWS EC2 Deployment

### Setup EC2 Instance

**Step 1: Launch Instance**
1. AWS Console → EC2 → Launch Instance
2. Amazon Linux 2 or Ubuntu 22.04
3. Instance type: t3.micro (free tier)
4. Create security group:
   - Allow SSH (22) from your IP
   - Allow HTTP (80) from anywhere
   - Allow HTTPS (443) from anywhere
5. Launch

**Step 2: Connect**
```bash
chmod 400 your-key.pem
ssh -i your-key.pem ec2-user@your-instance-ip
```

**Step 3: Install Dependencies**
```bash
# Amazon Linux 2
sudo yum update -y
sudo yum install -y git
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Ubuntu
sudo apt update && sudo apt upgrade -y
sudo apt install -y git
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

**Step 4-7: Follow DigitalOcean steps above**

---

## 7️⃣ CloudFlare Pages (Recommended for Static)

### Perfect for: Static exports only

**Setup:**
1. Build static site
2. Push to GitHub
3. Connect to Cloudflare Pages
4. Build command: `cargo build --release`
5. Output: `frontend/dist`

**Benefits:**
- Automatic deployments
- Global CDN
- Free SSL
- Worker support for dynamic features

---

## Post-Deployment Checklist

After deploying:

- [ ] Site loads successfully
- [ ] API endpoints respond
- [ ] Data files accessible
- [ ] SSL certificate valid
- [ ] Responsive on mobile
- [ ] Performance acceptable
- [ ] Error logging works
- [ ] Update DNS if needed
- [ ] Setup monitoring
- [ ] Configure backups
- [ ] Test with real users

---

## Monitoring & Maintenance

### Logs
```bash
# Local
cargo run 2>&1 | tee app.log

# Docker
docker logs container-name

# Server (systemd)
journalctl -u kreatives -f
```

### Updates
```bash
# Pull latest code
git pull

# Rebuild
cargo build --release

# Restart service
systemctl restart kreatives
```

### Backup
```bash
# Backup data directory
tar -czf data-backup.tar.gz data/

# Backup entire app
tar -czf kreatives-backup.tar.gz .
```

---

## Performance Optimization

### CDN Setup
```nginx
# In Nginx config
expires 1y;
add_header Cache-Control "public, immutable";
```

### Compression
Already enabled in Axum - gzip compression active

### Database (if needed)
```rust
// Add to dependencies
sqlx = "0.7"
tokio-postgres = "0.7"
```

---

## Troubleshooting Deployment

### Port Already in Use
```bash
lsof -i :3000
kill -9 PID
```

### Permission Denied
```bash
chmod +x ./target/release/kreatives-frontend
```

### Data Not Found
```bash
# Ensure data folder exists and has correct path
ls -la data/
```

### High Memory Usage
```bash
# Monitor
top
ps aux | grep kreatives

# Restart service
systemctl restart kreatives
```

---

For specific platform issues:
- GitHub Pages: https://docs.github.com/en/pages
- Vercel: https://vercel.com/docs
- Netlify: https://docs.netlify.com
- DigitalOcean: https://www.digitalocean.com/docs
