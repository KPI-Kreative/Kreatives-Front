use leptos::*;
use serde_json::Value;

#[component]
pub fn DataDisplay(
    #[prop(default = "Data".to_string())] title: String,
    data: Value,
) -> impl IntoView {
    let formatted = format!("{:#}", data);
    
    view! {
        <div class="data-display">
            <h2>{title}</h2>
            <pre>
                <code>{formatted}</code>
            </pre>
        </div>
    }
}

#[component]
pub fn JsonList(items: Vec<Value>) -> impl IntoView {
    view! {
        <ul class="json-list">
            {items.into_iter().map(|item| {
                view! {
                    <li>
                        <div class="list-item">
                            {format!("{}", item)}
                        </div>
                    </li>
                }
            }).collect::<Vec<_>>()}
        </ul>
    }
}
