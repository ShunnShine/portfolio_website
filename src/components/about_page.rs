use leptos::*;

#[component]
pub fn AboutPage() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-5 p-10">
            <h1>"Hi my name is Jack Shunn!"</h1>
            <h2>"I am a software developer and a computer science student at the University of Utah!"</h2>
            <h3>"This website is built using the Rust programming language using the Leptos framework. Also tailwind for styling."</h3>
        </div>
    }
}