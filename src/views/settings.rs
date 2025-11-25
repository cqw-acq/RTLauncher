use dioxus::prelude::*;

/// The Settings page component that will be rendered when the current route is `[Route::Settings]`
#[component]
pub fn Settings() -> Element {
    rsx! {
        div {
            class: "flex flex-col items-center justify-center min-h-screen p-4 ml-16",
            h1 {
                class: "text-3xl font-bold mb-4",
                "设置页面"
            }
            p {
                class: "text-lg text-muted-foreground",
                "这是设置页面的内容"
            }
        }
    }
}
