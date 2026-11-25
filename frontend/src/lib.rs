use leptos::*;

pub mod components;
pub mod data_loader;
pub mod pages;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div class="app">
            <pages::home::HomePage />
        </div>
    }
}
