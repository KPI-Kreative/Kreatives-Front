use leptos::*;
use crate::components::{Header, Footer};

#[component]
pub fn HomePage() -> impl IntoView {
    view! {
        <div class="home-page">
            <Header />
            
            <main class="main-content">
                <section class="hero">
                    <h1>"Welcome to Kreatives"</h1>
                    <p>"A static website built with Leptos and Axum"</p>
                </section>
                
                <section class="features">
                    <h2>"Features"</h2>
                    <div class="features-grid">
                        <div class="feature-card">
                            <h3>"📊 Static Data"</h3>
                            <p>"Load data from JSON, TOML, and YAML files"</p>
                        </div>
                        <div class="feature-card">
                            <h3>"⚡ Fast"</h3>
                            <p>"Built with Leptos for optimal performance"</p>
                        </div>
                        <div class="feature-card">
                            <h3>"🎨 Responsive"</h3>
                            <p>"Beautiful and responsive design"</p>
                        </div>
                    </div>
                </section>
            </main>
            
            <Footer />
        </div>
    }
}
