use leptos::*;

pub mod components;
pub mod data_loader;
pub mod pages;

#[component]
pub fn App() -> impl IntoView {
    let path = leptos::window()
        .location()
        .pathname()
        .unwrap_or_else(|_| "/".to_string());

    let page = match path.as_str() {
        "/products" => view! { <pages::products::ProductsPage /> }.into_view(),
        "/demo" => view! { <pages::demo::DemoPage /> }.into_view(),
        _ => view! { <pages::home::HomePage /> }.into_view(),
    };

    view! {
        <div class="app">
            {page}
        </div>
    }
}
