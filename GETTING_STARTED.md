# 📋 Getting Started Checklist

## ✅ Initial Setup (Do This First!)

### Prerequisites
- [ ] Install Rust from https://rustup.rs/
- [ ] Clone/download this repository
- [ ] Open terminal in project directory

### First Run
- [ ] Run `cargo build` to download dependencies
- [ ] Run `cargo run` to start dev server
- [ ] Open http://localhost:3000 in browser
- [ ] Verify "Welcome to Kreatives" page loads
- [ ] Test API: `curl http://localhost:3000/health`

---

## 🎯 Customize Your Site

### Step 1: Update Site Configuration
**File:** `data/json/site-config.json`

- [ ] Change `site_title` to your site name
- [ ] Update `site_description`
- [ ] Modify features list
- [ ] Update `author` name

**Example:**
```json
{
  "site_title": "My Portfolio",
  "site_description": "Welcome to my creative portfolio",
  "author": "Your Name"
}
```

### Step 2: Update Portfolio Data
**File:** `data/toml/portfolio.toml`

- [ ] Change author name and email
- [ ] Update social media links (Twitter, GitHub, LinkedIn)
- [ ] Add your projects in `[[projects]]` sections
- [ ] Remove or modify example projects

**Example:**
```toml
[author]
name = "Jane Doe"
email = "jane@example.com"
website = "https://janedoe.dev"

[[projects]]
title = "My Amazing Project"
description = "What I built"
url = "https://project.example.com"
tags = ["web", "design"]
```

### Step 3: Add Content/Blog Posts
**File:** `data/yaml/content.yaml`

- [ ] Update site name and tagline
- [ ] Add blog posts to `blog_posts` section
- [ ] Remove example posts
- [ ] Add your services

**Example:**
```yaml
site:
  name: Jane's Portfolio
  tagline: Web Designer & Developer
  
blog_posts:
  - title: My First Blog Post
    slug: first-post
    date: 2026-11-24
    tags: [web, design]
    excerpt: This is my first post
```

### Step 4: Customize Styling
**File:** `frontend/dist/style.css`

- [ ] Change primary colors (look for `#2c3e50`, `#3498db`)
- [ ] Update fonts if desired
- [ ] Adjust spacing/padding
- [ ] Modify responsive breakpoints
- [ ] Add your brand colors

**Key colors to change:**
```css
/* Header background */
background-color: #2c3e50;  /* Change this */

/* Accent color */
color: #3498db;  /* Change this */
```

---

## 🏗️ Add New Features

### Add New Data File
- [ ] Create new file in `data/json/`, `data/toml/`, or `data/yaml/`
- [ ] Add your data
- [ ] Test via API: `curl http://localhost:3000/api/data/[format]/[filename]`

### Create New Component
- [ ] Create `frontend/src/components/my-component.rs`
- [ ] Write component code
- [ ] Export from `frontend/src/components/mod.rs`
- [ ] Use in pages

### Add New Page
- [ ] Create `frontend/src/pages/my-page.rs`
- [ ] Write page component
- [ ] Export from `frontend/src/pages/mod.rs`
- [ ] Link in navigation header

### Update Navigation
**File:** `frontend/src/components/header.rs`

- [ ] Add new links in `.nav-links` section
- [ ] Test links work
- [ ] Check responsive on mobile

---

## 🧪 Testing

### Local Testing
- [ ] Site loads at http://localhost:3000
- [ ] API endpoints respond:
  - [ ] http://localhost:3000/health
  - [ ] http://localhost:3000/api/data/json/site-config.json
  - [ ] http://localhost:3000/api/data/toml/portfolio.toml
  - [ ] http://localhost:3000/api/data/yaml/content.yaml
- [ ] All navigation links work
- [ ] No console errors

### Responsive Testing
- [ ] Desktop view (1920x1080)
- [ ] Tablet view (768x1024)
- [ ] Mobile view (375x667)
- [ ] All features accessible

### Browser Compatibility
- [ ] Chrome/Edge
- [ ] Firefox
- [ ] Safari (if on Mac)

---

## 🚀 Ready to Deploy?

### Before Deployment
- [ ] All local tests pass
- [ ] Build succeeds: `cargo build --release`
- [ ] No compiler warnings
- [ ] Site looks good on mobile
- [ ] Data files are correct
- [ ] API responds properly

### Choose Deployment Platform
- [ ] **Easy:** GitHub Pages, Vercel, or Netlify
- [ ] **More Control:** DigitalOcean or AWS
- [ ] **Containerized:** Docker

### Deployment Steps
1. [ ] Choose platform from [DEPLOYMENT.md](./DEPLOYMENT.md)
2. [ ] Follow setup instructions
3. [ ] Connect repository
4. [ ] Configure build/output
5. [ ] Deploy
6. [ ] Verify live site works

---

## 📚 Documentation Roadmap

### For Quick Learning (30 min)
- [ ] Read [PROJECT_SUMMARY.md](./PROJECT_SUMMARY.md) (5 min)
- [ ] Follow [QUICKSTART.md](./QUICKSTART.md) (5 min)
- [ ] Explore project structure (5 min)
- [ ] Customize 2-3 files (10 min)
- [ ] Deploy somewhere (5 min)

### For Full Understanding (2-3 hours)
- [ ] Read [README.md](./README.md) - Full reference
- [ ] Study [DEVELOPMENT.md](./DEVELOPMENT.md) - Architecture & patterns
- [ ] Review [DEPLOYMENT.md](./DEPLOYMENT.md) - Deployment options
- [ ] Experiment with components
- [ ] Add custom features

### For Production Deployment (1-2 hours)
- [ ] Choose deployment platform
- [ ] Read relevant section in [DEPLOYMENT.md](./DEPLOYMENT.md)
- [ ] Set up custom domain
- [ ] Configure SSL/HTTPS
- [ ] Set up monitoring
- [ ] Test live site thoroughly

---

## 🛠️ Common Customizations

### Change Header Logo
**File:** `frontend/src/components/header.rs`
```rust
<h1>"Your Site Name"</h1>  // Change this
```

### Add Hero Image
1. [ ] Add image to `frontend/dist/images/`
2. [ ] Update CSS in `frontend/dist/style.css`
3. [ ] Link in hero section HTML

### Add Social Media Links
**File:** `frontend/src/components/footer.rs`
```rust
<a href="https://your-social-link">"Your Platform"</a>
```

### Change Color Scheme
**File:** `frontend/dist/style.css`
- [ ] Find `#2c3e50` - primary dark color
- [ ] Find `#3498db` - accent blue color
- [ ] Find `#f9f9f9` - light background
- [ ] Replace with your colors

### Add Email Contact Form
1. [ ] Create component `frontend/src/components/contact-form.rs`
2. [ ] Add form fields
3. [ ] Configure backend email (requires backend service)

---

## 🐛 Troubleshooting Quick Fixes

### Site Won't Start
```bash
# Clean build
cargo clean
cargo build
cargo run
```

### Port 3000 in use
```bash
# Windows: Find and kill process
netstat -ano | findstr :3000
taskkill /PID [PID] /F

# Linux/Mac: Kill on port
lsof -ti:3000 | xargs kill -9
```

### CSS not updating
- [ ] Hard refresh browser (Ctrl+Shift+R or Cmd+Shift+R)
- [ ] Clear browser cache
- [ ] Restart dev server

### Data file not loading
- [ ] Check file exists: `ls data/json/file.json`
- [ ] Check path in API call
- [ ] Verify file format is correct

### Slow build
- [ ] First build takes 1-2 minutes - normal!
- [ ] Subsequent builds ~10-30 seconds
- [ ] Use `cargo build --release` for production

---

## 📊 Project Stats After Customization

After completing setup, you'll have:

- [ ] Custom site configuration
- [ ] Updated portfolio/projects
- [ ] Custom content and blog posts
- [ ] Branded colors and styling
- [ ] Working navigation
- [ ] Responsive design
- [ ] API endpoints with your data
- [ ] Ready for deployment

---

## 🎓 Learning Path

1. **Understanding (Day 1)**
   - [ ] Read all documentation
   - [ ] Review source code
   - [ ] Run locally and explore

2. **Customization (Day 1-2)**
   - [ ] Update configuration files
   - [ ] Modify styling
   - [ ] Create custom content
   - [ ] Test thoroughly

3. **Extension (Day 2-3)**
   - [ ] Add new components
   - [ ] Create new pages
   - [ ] Add more data formats
   - [ ] Enhance styling

4. **Deployment (Day 3)**
   - [ ] Choose platform
   - [ ] Configure deployment
   - [ ] Go live!
   - [ ] Monitor and iterate

---

## ✨ Success Checklist

Your site is ready when:
- [ ] ✅ Builds without errors: `cargo build --release`
- [ ] ✅ Runs locally: `cargo run`
- [ ] ✅ Shows your content
- [ ] ✅ Uses your branding
- [ ] ✅ Responsive on mobile
- [ ] ✅ API endpoints work
- [ ] ✅ No console errors
- [ ] ✅ Ready to deploy

---

## 🚀 Deployment Checklist

Final checks before going live:
- [ ] Build release binary: `cargo build --release`
- [ ] Test on another machine (if possible)
- [ ] Choose hosting platform
- [ ] Configure custom domain
- [ ] Set up SSL/HTTPS
- [ ] Deploy and verify
- [ ] Monitor for errors
- [ ] Share with others!

---

## 🎉 You're Ready!

You have everything you need to:
1. ✅ Understand the project
2. ✅ Customize for your brand
3. ✅ Deploy to production
4. ✅ Maintain and update

**Start with:**
```bash
cargo run
```

**Visit:** http://localhost:3000

**Happy Building! 🚀**

---

## 📞 Quick Reference

| Task | File | Command |
|------|------|---------|
| Start server | - | `cargo run` |
| Build release | - | `cargo build --release` |
| View code | `frontend/src/` | - |
| Update config | `data/json/site-config.json` | - |
| Customize style | `frontend/dist/style.css` | - |
| Check API | - | `curl http://localhost:3000/health` |
| View docs | [README.md](./README.md) | - |

---

**Next Step:** Open terminal and run `cargo run` 🎯
