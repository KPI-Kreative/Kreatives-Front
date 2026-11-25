use leptos::*;

#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="footer">
            <div class="footer-content">
                <p>"© 2026 Kreatives. All rights reserved."</p>
                <div class="social-links">
                    <a href="https://twitter.com" target="_blank">"Twitter"</a>
                    <a href="https://github.com" target="_blank">"GitHub"</a>
                    <a href="https://linkedin.com" target="_blank">"LinkedIn"</a>
                </div>
            </div>
        </footer>
    }
}
