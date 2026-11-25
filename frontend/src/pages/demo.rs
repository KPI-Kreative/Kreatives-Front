use leptos::*;
use crate::components::{Footer, Header};

#[component]
pub fn DemoPage() -> impl IntoView {
    view! {
        <div class="demo-page">
            <Header />
            <main class="demo-main">
                <section class="demo-hero">
                    <p class="eyebrow">"PRODUCT DEMO HUB"</p>
                    <h1>"Explore the planned product demos."</h1>
                    <p>"Open each demo section to preview direction and sample deliverables."</p>
                </section>
                <section class="demo-grid">
                    <a class="demo-card" href="/extensions">
                        <h2>"Extensions"</h2>
                        <p>"Preview browser and editor extension concepts."</p>
                    </a>
                    <a class="demo-card" href="/mobile-apps">
                        <h2>"Mobile Apps"</h2>
                        <p>"Explore mobile-first workflows and user flows."</p>
                    </a>
                    <a class="demo-card" href="/data-analytics">
                        <h2>"Data Analytics"</h2>
                        <p>"Review dashboard and reporting demo concepts."</p>
                    </a>
                    <a class="demo-card" href="/sample-contract">
                        <h2>"Sample Contract PDF"</h2>
                        <p>"Open the sample contract PDF preview page."</p>
                    </a>
                </section>
            </main>
            <Footer />
        </div>
    }
}
