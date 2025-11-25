use dioxus::prelude::*;
use crate::components::ui::{Card, CardHeader, CardTitle, CardContent, CardDescription, Button, ButtonVariant};

/// Instance basic settings page
#[component]
pub fn InstanceBasic() -> Element {
    rsx! {
        div {
            class: "space-y-6",
            // Header
            div {
                class: "mb-6 flex flex-col md:flex-row md:items-center md:justify-between gap-4",
                h1 {
                    class: "text-3xl font-bold leading-tight",
                    "基础设置"
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

            div {
                class: "grid grid-cols-1 lg:grid-cols-3 gap-6",
                // Main content - 2 columns
                div {
                    class: "lg:col-span-2 space-y-6",
                    // Instance info card
                    Card {
                        CardHeader {
                            CardTitle {
                                class: "flex items-center gap-2",
                                svg {
                                    xmlns: "http://www.w3.org/2000/svg",
                                    width: "20",
                                    height: "20",
                                    view_box: "0 0 24 24",
                                    fill: "none",
                                    stroke: "currentColor",
                                    stroke_width: "2",
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    circle { cx: "12", cy: "12", r: "10" }
                                    path { d: "M12 16v-4" }
                                    path { d: "M12 8h.01" }
                                }
                                "实例信息"
                            }
                            CardDescription { "配置实例的基本信息" }
                        }
                        CardContent {
                            class: "space-y-5",
                            div {
                                class: "grid grid-cols-1 md:grid-cols-2 gap-5",
                                div {
                                    class: "space-y-2",
                                    label {
                                        class: "text-sm font-medium",
                                        "实例名称"
                                    }
                                    input {
                                        r#type: "text",
                                        class: "w-full px-3 py-2 border rounded-md bg-background",
                                        placeholder: "输入实例名称"
                                    }
                                }
                                div {
                                    class: "space-y-2",
                                    label {
                                        class: "text-sm font-medium",
                                        "实例ID"
                                    }
                                    input {
                                        r#type: "text",
                                        class: "w-full px-3 py-2 border rounded-md bg-background opacity-50",
                                        placeholder: "自动生成",
                                        disabled: true
                                    }
                                }
                            }
                        }
                    }

                    // Launch options card
                    Card {
                        CardHeader {
                            CardTitle {
                                class: "flex items-center gap-2",
                                svg {
                                    xmlns: "http://www.w3.org/2000/svg",
                                    width: "20",
                                    height: "20",
                                    view_box: "0 0 24 24",
                                    fill: "none",
                                    stroke: "currentColor",
                                    stroke_width: "2",
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    polygon { points: "6 3 20 12 6 21 6 3" }
                                }
                                "启动选项"
                            }
                            CardDescription { "配置游戏的基本启动参数" }
                        }
                        CardContent {
                            class: "space-y-5",
                            div {
                                class: "flex items-center justify-between",
                                div {
                                    class: "space-y-1",
                                    label {
                                        class: "text-sm font-medium",
                                        "版本隔离"
                                    }
                                    p {
                                        class: "text-sm text-muted-foreground",
                                        "为不同版本使用独立的运行环境"
                                    }
                                }
                                input {
                                    r#type: "checkbox",
                                    class: "w-4 h-4"
                                }
                            }
                            div {
                                class: "space-y-2",
                                label {
                                    class: "text-sm font-medium",
                                    "游戏窗口标题"
                                }
                                input {
                                    r#type: "text",
                                    class: "w-full px-3 py-2 border rounded-md bg-background",
                                    placeholder: "Minecraft* 1.21.8"
                                }
                            }
                            div {
                                class: "space-y-2",
                                label {
                                    class: "text-sm font-medium",
                                    "Java 选择"
                                }
                                select {
                                    class: "w-full px-3 py-2 border rounded-md bg-background",
                                    option { value: "java17", "Java 17 (推荐)" }
                                    option { value: "java21", "Java 21" }
                                    option { value: "java11", "Java 11" }
                                    option { value: "java8", "Java 8" }
                                }
                            }
                        }
                    }

                    // Memory config card
                    Card {
                        CardHeader {
                            CardTitle {
                                class: "flex items-center gap-2",
                                svg {
                                    xmlns: "http://www.w3.org/2000/svg",
                                    width: "20",
                                    height: "20",
                                    view_box: "0 0 24 24",
                                    fill: "none",
                                    stroke: "currentColor",
                                    stroke_width: "2",
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    path { d: "M13 2 3 14h9l-1 8 10-12h-9l1-8z" }
                                }
                                "游戏内存"
                            }
                            CardDescription { "调整游戏内存分配" }
                        }
                        CardContent {
                            class: "space-y-5",
                            div {
                                class: "space-y-4",
                                div {
                                    class: "flex items-start space-x-3 p-4 rounded-lg border cursor-pointer hover:bg-accent",
                                    input {
                                        r#type: "radio",
                                        name: "memory",
                                        value: "follow-global",
                                        checked: true,
                                        class: "mt-1"
                                    }
                                    div {
                                        class: "flex-1",
                                        div {
                                            class: "font-medium",
                                            "跟随全局设置"
                                        }
                                        p {
                                            class: "text-sm text-muted-foreground mt-1",
                                            "使用全局内存配置"
                                        }
                                    }
                                }
                                div {
                                    class: "flex items-start space-x-3 p-4 rounded-lg border cursor-pointer hover:bg-accent",
                                    input {
                                        r#type: "radio",
                                        name: "memory",
                                        value: "auto-config",
                                        class: "mt-1"
                                    }
                                    div {
                                        class: "flex-1",
                                        div {
                                            class: "font-medium",
                                            "自动配置"
                                        }
                                        p {
                                            class: "text-sm text-muted-foreground mt-1",
                                            "根据系统自动分配内存"
                                        }
                                    }
                                }
                                div {
                                    class: "flex items-start space-x-3 p-4 rounded-lg border cursor-pointer hover:bg-accent",
                                    input {
                                        r#type: "radio",
                                        name: "memory",
                                        value: "manual-config",
                                        class: "mt-1"
                                    }
                                    div {
                                        class: "flex-1",
                                        div {
                                            class: "font-medium",
                                            "手动配置"
                                        }
                                        p {
                                            class: "text-sm text-muted-foreground mt-1",
                                            "手动设置最大内存"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // Side panel - 1 column
                div {
                    class: "space-y-6",
                    // Launch settings card
                    Card {
                        CardHeader {
                            CardTitle {
                                class: "flex items-center gap-2",
                                svg {
                                    xmlns: "http://www.w3.org/2000/svg",
                                    width: "20",
                                    height: "20",
                                    view_box: "0 0 24 24",
                                    fill: "none",
                                    stroke: "currentColor",
                                    stroke_width: "2",
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    polygon { points: "6 3 20 12 6 21 6 3" }
                                }
                                "启动设置"
                            }
                        }
                        CardContent {
                            class: "space-y-5",
                            div {
                                class: "flex items-center justify-between",
                                div {
                                    class: "space-y-1",
                                    label {
                                        class: "text-sm font-medium",
                                        "自动连接服务器"
                                    }
                                    p {
                                        class: "text-sm text-muted-foreground",
                                        "启动时自动连接"
                                    }
                                }
                                input {
                                    r#type: "checkbox",
                                    class: "w-4 h-4"
                                }
                            }
                            div {
                                class: "flex items-center justify-between",
                                div {
                                    class: "space-y-1",
                                    label {
                                        class: "text-sm font-medium",
                                        "自动全屏"
                                    }
                                    p {
                                        class: "text-sm text-muted-foreground",
                                        "启动时全屏"
                                    }
                                }
                                input {
                                    r#type: "checkbox",
                                    class: "w-4 h-4"
                                }
                            }
                            div {
                                class: "flex items-center justify-between",
                                div {
                                    class: "space-y-1",
                                    label {
                                        class: "text-sm font-medium",
                                        "显示控制台"
                                    }
                                    p {
                                        class: "text-sm text-muted-foreground",
                                        "显示调试控制台"
                                    }
                                }
                                input {
                                    r#type: "checkbox",
                                    class: "w-4 h-4"
                                }
                            }
                        }
                    }

                    // Quick actions card
                    Card {
                        CardHeader {
                            CardTitle {
                                class: "flex items-center gap-2",
                                svg {
                                    xmlns: "http://www.w3.org/2000/svg",
                                    width: "20",
                                    height: "20",
                                    view_box: "0 0 24 24",
                                    fill: "none",
                                    stroke: "currentColor",
                                    stroke_width: "2",
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    path { d: "m6 14 1.5-2.9A2 2 0 0 1 9.24 10H20a2 2 0 0 1 1.94 2.5l-1.54 6a2 2 0 0 1-1.95 1.5H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h3.9a2 2 0 0 1 1.69.9l.81 1.2a2 2 0 0 0 1.67.9H18a2 2 0 0 1 2 2v2" }
                                }
                                "快速操作"
                            }
                        }
                        CardContent {
                            div {
                                class: "space-y-3",
                                Button {
                                    variant: ButtonVariant::Outline,
                                    class: "w-full",
                                    svg {
                                        xmlns: "http://www.w3.org/2000/svg",
                                        width: "16",
                                        height: "16",
                                        view_box: "0 0 24 24",
                                        fill: "none",
                                        stroke: "currentColor",
                                        stroke_width: "2",
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        class: "mr-2",
                                        path { d: "m6 14 1.5-2.9A2 2 0 0 1 9.24 10H20a2 2 0 0 1 1.94 2.5l-1.54 6a2 2 0 0 1-1.95 1.5H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h3.9a2 2 0 0 1 1.69.9l.81 1.2a2 2 0 0 0 1.67.9H18a2 2 0 0 1 2 2v2" }
                                    }
                                    "打开游戏文件夹"
                                }
                                Button {
                                    variant: ButtonVariant::Outline,
                                    class: "w-full",
                                    svg {
                                        xmlns: "http://www.w3.org/2000/svg",
                                        width: "16",
                                        height: "16",
                                        view_box: "0 0 24 24",
                                        fill: "none",
                                        stroke: "currentColor",
                                        stroke_width: "2",
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        class: "mr-2",
                                        path { d: "M3 12a9 9 0 1 0 9-9 9.75 9.75 0 0 0-6.74 2.74L3 8" }
                                        path { d: "M3 3v5h5" }
                                    }
                                    "重置所有设置"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
