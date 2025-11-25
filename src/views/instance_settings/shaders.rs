use dioxus::prelude::*;
use crate::components::ui::{Card, CardHeader, CardTitle, CardContent, CardDescription, Button, ButtonVariant};

/// Instance shaders management page
#[component]
pub fn InstanceShaders() -> Element {
    rsx! {
        div {
            class: "space-y-6",
            // Header
            div {
                class: "mb-6 flex flex-col md:flex-row md:items-center md:justify-between gap-4",
                h1 {
                    class: "text-3xl font-bold leading-tight",
                    "光影管理"
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
                        "添加光影"
                    }
                }
            }

            // Shaders list card
            Card {
                CardHeader {
                    CardTitle { "已安装的光影" }
                    CardDescription { "管理实例中的光影包" }
                }
                CardContent {
                    div {
                        class: "text-center py-8 text-muted-foreground",
                        p { "暂无安装的光影包" }
                        p {
                            class: "text-sm mt-2",
                            "点击 \"添加光影\" 按钮来安装新的光影包"
                        }
                    }
                }
            }
        }
    }
}
