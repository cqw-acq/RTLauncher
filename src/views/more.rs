use dioxus::prelude::*;

/// The More page component that will be rendered when the current route is `[Route::More]`
#[component]
pub fn More() -> Element {
    rsx! {
        div {
            class: "flex flex-col items-center justify-center min-h-screen p-4 ml-16",
            h1 {
                class: "text-3xl font-bold mb-4",
                "更多页面"
            }
            p {
                class: "text-lg text-muted-foreground",
                "这是更多页面的内容"
            }
        }
    }
}
