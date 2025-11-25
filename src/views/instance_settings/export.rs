use dioxus::prelude::*;
use crate::components::ui::{Card, CardHeader, CardTitle, CardContent, CardDescription, Button, ButtonVariant};

/// Instance export page
#[component]
pub fn InstanceExport() -> Element {
    rsx! {
        div {
            class: "space-y-6",
            // Header
            div {
                class: "mb-6 flex flex-col md:flex-row md:items-center md:justify-between gap-4",
                h1 {
                    class: "text-3xl font-bold leading-tight",
                    "导出实例"
                }
            }

            // Export options card
            Card {
                CardHeader {
                    CardTitle { "导出选项" }
                    CardDescription { "选择要导出的内容和格式" }
                }
                CardContent {
                    class: "space-y-5",
                    div {
                        class: "space-y-4",
                        div {
                            class: "flex items-center justify-between p-4 rounded-lg border",
                            div {
                                class: "space-y-1",
                                label {
                                    class: "text-sm font-medium",
                                    "包含 Mods"
                                }
                                p {
                                    class: "text-sm text-muted-foreground",
                                    "导出时包含已安装的模组"
                                }
                            }
                            input {
                                r#type: "checkbox",
                                checked: true,
                                class: "w-4 h-4"
                            }
                        }
                        div {
                            class: "flex items-center justify-between p-4 rounded-lg border",
                            div {
                                class: "space-y-1",
                                label {
                                    class: "text-sm font-medium",
                                    "包含资源包"
                                }
                                p {
                                    class: "text-sm text-muted-foreground",
                                    "导出时包含资源包"
                                }
                            }
                            input {
                                r#type: "checkbox",
                                checked: true,
                                class: "w-4 h-4"
                            }
                        }
                        div {
                            class: "flex items-center justify-between p-4 rounded-lg border",
                            div {
                                class: "space-y-1",
                                label {
                                    class: "text-sm font-medium",
                                    "包含存档"
                                }
                                p {
                                    class: "text-sm text-muted-foreground",
                                    "导出时包含游戏存档"
                                }
                            }
                            input {
                                r#type: "checkbox",
                                class: "w-4 h-4"
                            }
                        }
                        div {
                            class: "flex items-center justify-between p-4 rounded-lg border",
                            div {
                                class: "space-y-1",
                                label {
                                    class: "text-sm font-medium",
                                    "包含配置"
                                }
                                p {
                                    class: "text-sm text-muted-foreground",
                                    "导出时包含配置文件"
                                }
                            }
                            input {
                                r#type: "checkbox",
                                checked: true,
                                class: "w-4 h-4"
                            }
                        }
                    }
                    div {
                        class: "pt-4",
                        Button {
                            variant: ButtonVariant::Default,
                            class: "w-full",
                            "导出实例"
                        }
                    }
                }
            }
        }
    }
}
