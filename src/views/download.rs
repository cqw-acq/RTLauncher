use dioxus::prelude::*;

/// The Download page component that will be rendered when the current route is `[Route::Download]`
#[component]
pub fn Download() -> Element {
    rsx! {
        div {
            class: "flex flex-col items-center justify-center min-h-screen p-4 ml-16",
            h1 {
                class: "text-3xl font-bold mb-4",
                "下载页面"
            }
            p {
                class: "text-lg text-muted-foreground",
                "这是下载页面的内容"
            }
        }
    }
}
