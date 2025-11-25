use dioxus::prelude::*;

/// The Lan page component that will be rendered when the current route is `[Route::Lan]`
#[component]
pub fn Lan() -> Element {
    rsx! {
        div {
            class: "flex flex-col items-center justify-center min-h-screen p-4 ml-16",
            h1 {
                class: "text-3xl font-bold mb-4",
                "联机页面"
            }
            p {
                class: "text-lg text-muted-foreground",
                "这是联机页面的内容"
            }
        }
    }
}
