use leptos::*;
pub mod components;
use components::{nav_bar::*, about_page::*, resume_page::*, project_page::*};
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <nav class="w-full h-20 bg-30">
                <NavBar/>
            </nav>
            <div class="w-full h-2 bg-10"/>
            <main class="w-full min-h-screen bg-60">
                <Routes>
                    <Route path="/portfolio_website/" view=|| view! {<Outlet/>}>
                        <Route path="" view=HashRouter/>
                    </Route>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
pub fn HashRouter() -> impl IntoView {
    let hash_memo = use_location().hash;
    view! { 
        {move || {
            match hash_memo.get().as_str() {
                "" => view! {<AboutPage/>},
                "#/about" => view! {<AboutPage/>},
                "#/resume" => view! {<ResumePage/>},
                "#/projects" => view! {<ProjectPage/>},
                unknown_hash => view! {<p>{format!("\"{}\" Route Not found", unknown_hash)}</p>}.into_view(),
            }
        }}
    }
}