use leptos::*;

#[component]
pub fn ResumePage() -> impl IntoView {
    view! {
        <div class="w-full min-h-screen flex justify-center p-28">
            <object data="resume.pdf" type="application/pdf" class="aspect-[8.5/11] h-[1000px]">
                <p>
                    "It appears you don't have a PDF plugin for this browser.
                    No biggie... you can "<a href="myfile.pdf">"click here to download the PDF file."</a>
                </p>
            </object>
        </div>
    }
}