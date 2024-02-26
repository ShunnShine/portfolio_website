use leptos::*;

#[component]
pub fn NavElement(text: &'static str) -> impl IntoView {
    view! {
        <a href={format!("#/{}", text.to_lowercase())}>{text}</a>
    }
}