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
    move || {
        match hash_memo.get().as_str() {
            "" | "#/about" => view! {<AboutPage/>},
            "#/resume" => view! {<ResumePage/>},
            "#/projects" => view! {<ProjectPage/>},
            unknown_hash => format!("\"{}\" Route Not found", unknown_hash).into_view(),
        }
    }
}