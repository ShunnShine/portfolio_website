use leptos::*;

#[component]
pub fn ProjectPage() -> impl IntoView {
    view! {
        <div class="flex flex-col items-center w-full py-10">
            <a href="https://collectshunn.com" class="text-30 underline text-xl mb-5">"CollectShunn.com"</a>
            <iframe src="https://collectshunn.com" class="h-[600px] w-3/4 border-8 border-10 rounded-md" />
            <div class="h-10"></div>
            <a href="https://shunn-shine.itch.io/heatwave" class="text-30 underline text-xl mb-5">"HeatWave Game"</a>
            <iframe src="heatwave.html" class="h-[600px] w-3/4 border-8 border-10 rounded-md" />
        </div>
    }
}