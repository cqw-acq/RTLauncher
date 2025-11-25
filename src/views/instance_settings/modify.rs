use dioxus::prelude::*;
use crate::components::ui::{Card, CardHeader, CardTitle, CardContent, CardDescription, Button, ButtonVariant};

/// Instance advanced settings (modify) page
#[component]
pub fn InstanceModify() -> Element {
    rsx! {
        div {
            class: "space-y-6",
            // Header
            div {
                class: "mb-6 flex flex-col md:flex-row md:items-center md:justify-between gap-4",
                h1 {
                    class: "text-3xl font-bold leading-tight",
                    "高级设置"
                }
                div {
                    class: "flex gap-3",
                    Button {
                        variant: ButtonVariant::Outline,
                        class: "px-6 py-2",
                        "重置设置"
                    }
                    Button {
                        variant: ButtonVariant::Default,
                        class: "px-6 py-2",
                        "保存更改"
                    }
                }
            }

            // JVM Arguments card
            Card {
                CardHeader {
                    CardTitle { "JVM 参数" }
                    CardDescription { "配置 Java 虚拟机启动参数" }
                }
                CardContent {
                    class: "space-y-5",
                    div {
                        class: "space-y-2",
                        label {
                            class: "text-sm font-medium",
                            "JVM 参数"
                        }
                        textarea {
                            class: "w-full px-3 py-2 border rounded-md bg-background min-h-[100px]",
                            placeholder: "-XX:+UseG1GC -XX:+ParallelRefProcEnabled -XX:MaxGCPauseMillis=200"
                        }
                    }
                }
            }

            // Game Arguments card
            Card {
                CardHeader {
                    CardTitle { "游戏参数" }
                    CardDescription { "配置 Minecraft 启动参数" }
                }
                CardContent {
                    class: "space-y-5",
                    div {
                        class: "space-y-2",
                        label {
                            class: "text-sm font-medium",
                            "游戏参数"
                        }
                        textarea {
                            class: "w-full px-3 py-2 border rounded-md bg-background min-h-[100px]",
                            placeholder: "--width 1920 --height 1080"
                        }
                    }
                }
            }

            // Environment Variables card
            Card {
                CardHeader {
                    CardTitle { "环境变量" }
                    CardDescription { "配置启动时的环境变量" }
                }
                CardContent {
                    class: "space-y-5",
                    div {
                        class: "space-y-2",
                        label {
                            class: "text-sm font-medium",
                            "环境变量"
                        }
                        textarea {
                            class: "w-full px-3 py-2 border rounded-md bg-background min-h-[100px]",
                            placeholder: "KEY=VALUE"
                        }
                    }
                }
            }

            // Wrapper command card
            Card {
                CardHeader {
                    CardTitle { "启动前命令" }
                    CardDescription { "在游戏启动前执行的命令" }
                }
                CardContent {
                    class: "space-y-5",
                    div {
                        class: "space-y-2",
                        label {
                            class: "text-sm font-medium",
                            "命令"
                        }
                        input {
                            r#type: "text",
                            class: "w-full px-3 py-2 border rounded-md bg-background",
                            placeholder: "输入启动前执行的命令"
                        }
                    }
                }
            }
        }
    }
}
