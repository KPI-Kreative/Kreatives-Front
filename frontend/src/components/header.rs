use leptos::*;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header class="header">
            <nav class="navbar">
                <div class="logo">
                    <h1>"Kreatives"</h1>
                </div>
                <ul class="nav-links">
                    <li><a href="/">"Home"</a></li>
                    <li><a href="/blog">"Blog"</a></li>
                    <li><a href="/portfolio">"Portfolio"</a></li>
                </ul>
            </nav>
        </header>
    }
}
