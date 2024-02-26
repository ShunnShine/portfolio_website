use leptos::*;
pub mod components;
use components::{nav_bar::*, front_content::*};


#[component]
pub fn App() -> impl IntoView {
    view! {
        <nav class="w-full h-20 bg-30">
            <NavBar/>
        </nav>
        <div class="w-full h-2 bg-10"/>
        <main class="w-full min-h-screen bg-60">
            <FrontContent/>
        </main>
    }
}