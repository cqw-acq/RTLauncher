use dioxus::prelude::*;
use crate::components::ui::{Card, CardHeader, CardTitle, CardContent, CardDescription, Button, ButtonVariant};

/// Instance worlds management page
#[component]
pub fn InstanceWorlds() -> Element {
    rsx! {
        div {
            class: "space-y-6",
            // Header
            div {
                class: "mb-6 flex flex-col md:flex-row md:items-center md:justify-between gap-4",
                h1 {
                    class: "text-3xl font-bold leading-tight",
                    "世界管理"
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
                        "导入世界"
                    }
                }
            }

            // Worlds list card
            Card {
                CardHeader {
                    CardTitle { "存档列表" }
                    CardDescription { "管理实例中的游戏存档" }
                }
                CardContent {
                    div {
                        class: "text-center py-8 text-muted-foreground",
                        p { "暂无游戏存档" }
                        p {
                            class: "text-sm mt-2",
                            "在游戏中创建新世界或点击 \"导入世界\" 按钮"
                        }
                    }
                }
            }
        }
    }
}
