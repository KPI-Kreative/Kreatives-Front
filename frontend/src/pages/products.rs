use leptos::*;
use crate::components::{Header, Footer};

#[component]
pub fn ProductsPage() -> impl IntoView {
    view! {
        <div class="products-page">
            <Header />
            <main class="products-main">
                <section class="products-hero">
                    <p class="eyebrow">"THE KREATIVES COLLECTION"</p>
                    <h1>"Tools for ideas that deserve to ship."</h1>
                    <p class="hero-copy">
                        "Planned products include extensions, mobile apps, and data analytics experiences."
                    </p>
                    <a class="primary-button" href="/demo">"Open demo page"</a>
                </section>

                <section class="product-section" id="product-grid">
                    <div class="product-heading">
                        <div>
                            <p class="eyebrow">"OUR PRODUCTS"</p>
                            <h2>"Built for momentum"</h2>
                        </div>
                        <p>"Simple, polished, and ready to make work feel lighter."</p>
                    </div>
                    <div class="product-grid">
                        <article class="product-card product-card-featured">
                            <div class="product-art art-brand"><span>"K"</span></div>
                            <div class="product-details">
                                <p class="product-type">"PLANNED PRODUCT"</p>
                                <h3>"Extensions"</h3>
                                <p>"Browser and editor extensions that streamline daily workflows."</p>
                            </div>
                        </article>
                        <article class="product-card">
                            <div class="product-art art-portal"><span></span><span></span><span></span></div>
                            <div class="product-details">
                                <p class="product-type">"PLANNED PRODUCT"</p>
                                <h3>"Mobile Apps"</h3>
                                <p>"Cross-platform companion apps for teams and clients on the go."</p>
                            </div>
                        </article>
                        <article class="product-card">
                            <div class="product-art art-launch"><span></span></div>
                            <div class="product-details">
                                <p class="product-type">"PLANNED PRODUCT"</p>
                                <h3>"Data Analytics"</h3>
                                <p>"Actionable dashboards and reporting to guide better decisions."</p>
                            </div>
                        </article>
                    </div>
                </section>
            </main>
            <Footer />
        </div>
    }
}
