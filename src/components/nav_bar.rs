use leptos::*;

use super::nav_element::*;

#[component]
pub fn NavBar() -> impl IntoView {
    view! {
        <div class="flex w-full h-full justify-start items-center gap-5 p-5">
            <NavElement text="About"/>
            <NavElement text="Resume"/>
            <NavElement text="Projects"/>
        </div>
    }
}