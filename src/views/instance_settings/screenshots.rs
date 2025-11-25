use dioxus::prelude::*;
use crate::components::ui::{Card, CardHeader, CardTitle, CardContent, CardDescription, Button, ButtonVariant};

/// Instance screenshots page
#[component]
pub fn InstanceScreenshots() -> Element {
    rsx! {
        div {
            class: "space-y-6",
            // Header
            div {
                class: "mb-6 flex flex-col md:flex-row md:items-center md:justify-between gap-4",
                h1 {
                    class: "text-3xl font-bold leading-tight",
                    "截图管理"
                }
                div {
                    class: "flex gap-3",
                    Button {
                        variant: ButtonVariant::Outline,
                        class: "px-6 py-2",
                        "刷新列表"
                    }
                    Button {
                        variant: ButtonVariant::Default,
                        class: "px-6 py-2",
                        "打开文件夹"
                    }
                }
            }

            // Screenshots gallery card
            Card {
                CardHeader {
                    CardTitle { "游戏截图" }
                    CardDescription { "查看和管理游戏中的截图" }
                }
                CardContent {
                    div {
                        class: "text-center py-8 text-muted-foreground",
                        p { "暂无截图" }
                        p {
                            class: "text-sm mt-2",
                            "在游戏中按 F2 键截图"
                        }
                    }
                }
            }
        }
    }
}
