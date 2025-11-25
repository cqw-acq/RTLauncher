use dioxus::prelude::*;
use crate::components::ui::{Card, CardHeader, CardTitle, CardContent, CardDescription, Button, ButtonVariant};

/// Resource data structure
#[derive(Clone, PartialEq)]
struct Resource {
    id: i32,
    name: String,
    resource_type: String,
    version: String,
    enabled: bool,
    size: String,
    author: String,
}

/// Instance resources management page - based on rtl-ui design
#[component]
pub fn InstanceResources() -> Element {
    let mut selected_resources = use_signal(|| Vec::<i32>::new());
    let mut search_query = use_signal(|| String::new());

    let resources = vec![
        Resource { id: 1, name: "Default Resources".to_string(), resource_type: "资源包".to_string(), version: "1.21.8".to_string(), enabled: true, size: "123MB".to_string(), author: "Mojang".to_string() },
        Resource { id: 2, name: "Simplistic Texture Pack".to_string(), resource_type: "资源包".to_string(), version: "1.21.8".to_string(), enabled: true, size: "456MB".to_string(), author: "Designer".to_string() },
        Resource { id: 3, name: "Data Pack".to_string(), resource_type: "数据包".to_string(), version: "1.21.8".to_string(), enabled: true, size: "45MB".to_string(), author: "Creator".to_string() },
        Resource { id: 4, name: "Custom Pack".to_string(), resource_type: "资源包".to_string(), version: "1.21.7".to_string(), enabled: false, size: "234MB".to_string(), author: "User".to_string() },
        Resource { id: 5, name: "Addon Pack".to_string(), resource_type: "附加包".to_string(), version: "1.21.8".to_string(), enabled: true, size: "89MB".to_string(), author: "Developer".to_string() },
    ];

    let filtered_resources: Vec<&Resource> = resources.iter()
        .filter(|r| r.name.to_lowercase().contains(&search_query().to_lowercase()) ||
                    r.author.to_lowercase().contains(&search_query().to_lowercase()))
        .collect();

    let enabled_count = resources.iter().filter(|r| r.enabled).count();
    let selected_count = selected_resources().len();

    rsx! {
        div {
            class: "space-y-6",
            // Header
            div {
                class: "mb-6 flex flex-col md:flex-row md:items-center md:justify-between gap-4",
                h1 {
                    class: "text-3xl font-bold leading-tight",
                    "资源管理"
                }
                div {
                    class: "flex gap-3",
                    Button {
                        variant: ButtonVariant::Outline,
                        class: "px-6 py-2 gap-2",
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            width: "16",
                            height: "16",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            class: "mr-1",
                            path { d: "m6 14 1.5-2.9A2 2 0 0 1 9.24 10H20a2 2 0 0 1 1.94 2.5l-1.54 6a2 2 0 0 1-1.95 1.5H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h3.9a2 2 0 0 1 1.69.9l.81 1.2a2 2 0 0 0 1.67.9H18a2 2 0 0 1 2 2v2" }
                        }
                        "打开文件夹"
                    }
                    Button {
                        variant: ButtonVariant::Outline,
                        class: "px-6 py-2 gap-2",
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            width: "16",
                            height: "16",
                            view_box: "0 0 24 24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            class: "mr-1",
                            path { d: "M5 12h14" }
                            path { d: "M12 5v14" }
                        }
                        "添加资源"
                    }
                }
            }

            // Main content - 3 column grid
            div {
                class: "grid grid-cols-1 lg:grid-cols-3 gap-6",
                // Resources list - 2 columns
                div {
                    class: "lg:col-span-2",
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
                                    path { d: "m7.5 4.27 9 5.15" }
                                    path { d: "M21 8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16Z" }
                                    path { d: "m3.3 7 8.7 5 8.7-5" }
                                    path { d: "M12 22V12" }
                                }
                                "已安装资源"
                            }
                            CardDescription { "共 {resources.len()} 个资源，已启用 {enabled_count} 个" }
                            div {
                                class: "mt-4",
                                input {
                                    r#type: "text",
                                    class: "max-w-sm px-3 py-2 border rounded-md bg-background w-full",
                                    placeholder: "搜索资源...",
                                    value: "{search_query}",
                                    oninput: move |e| search_query.set(e.value())
                                }
                            }
                        }
                        CardContent {
                            class: "space-y-2",
                            for r in filtered_resources.iter() {
                                div {
                                    key: "{r.id}",
                                    class: if selected_resources().contains(&r.id) {
                                        "group flex items-center justify-between p-4 rounded-lg border cursor-pointer transition-all duration-200 bg-blue-50 dark:bg-blue-950/30 border-blue-300 dark:border-blue-700"
                                    } else {
                                        "group flex items-center justify-between p-4 rounded-lg border cursor-pointer transition-all duration-200 border-slate-200 dark:border-slate-700 hover:bg-slate-50 dark:hover:bg-slate-800/40"
                                    },
                                    onclick: {
                                        let resource_id = r.id;
                                        move |_| {
                                            let mut current = selected_resources();
                                            if current.contains(&resource_id) {
                                                current.retain(|&x| x != resource_id);
                                            } else {
                                                current.push(resource_id);
                                            }
                                            selected_resources.set(current);
                                        }
                                    },
                                    div {
                                        class: "flex items-center gap-4 flex-1",
                                        // Checkbox
                                        div {
                                            class: if selected_resources().contains(&r.id) {
                                                "w-5 h-5 rounded-md border-2 transition-all flex items-center justify-center bg-blue-500 border-blue-500"
                                            } else {
                                                "w-5 h-5 rounded-md border-2 transition-all flex items-center justify-center border-slate-300 dark:border-slate-600"
                                            },
                                            if selected_resources().contains(&r.id) {
                                                svg {
                                                    class: "w-3 h-3 text-white",
                                                    fill: "none",
                                                    view_box: "0 0 24 24",
                                                    stroke: "currentColor",
                                                    path {
                                                        stroke_linecap: "round",
                                                        stroke_linejoin: "round",
                                                        stroke_width: "3",
                                                        d: "M5 13l4 4L19 7"
                                                    }
                                                }
                                            }
                                        }
                                        // Resource info
                                        div {
                                            class: "flex-1",
                                            div {
                                                class: "flex items-center gap-2",
                                                h3 {
                                                    class: "font-semibold text-sm",
                                                    "{r.name}"
                                                }
                                                span {
                                                    class: "px-2 py-0.5 text-xs rounded-full bg-secondary text-secondary-foreground",
                                                    "{r.resource_type}"
                                                }
                                            }
                                            p {
                                                class: "text-xs text-slate-500 dark:text-slate-400 mt-1",
                                                "版本：{r.version} • 作者：{r.author} • 大小：{r.size}"
                                            }
                                        }
                                    }
                                    // Hover buttons
                                    div {
                                        class: "flex gap-2 opacity-0 group-hover:opacity-100 transition-opacity duration-200",
                                        Button {
                                            variant: ButtonVariant::Ghost,
                                            class: "gap-1 h-8 px-2",
                                            svg {
                                                xmlns: "http://www.w3.org/2000/svg",
                                                width: "16",
                                                height: "16",
                                                view_box: "0 0 24 24",
                                                fill: "none",
                                                stroke: "currentColor",
                                                stroke_width: "2",
                                                path { d: "M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z" }
                                                circle { cx: "12", cy: "12", r: "3" }
                                            }
                                        }
                                        Button {
                                            variant: ButtonVariant::Ghost,
                                            class: "gap-1 h-8 px-2",
                                            svg {
                                                xmlns: "http://www.w3.org/2000/svg",
                                                width: "16",
                                                height: "16",
                                                view_box: "0 0 24 24",
                                                fill: "none",
                                                stroke: "currentColor",
                                                stroke_width: "2",
                                                path { d: "M3 12a9 9 0 1 0 9-9 9.75 9.75 0 0 0-6.74 2.74L3 8" }
                                                path { d: "M3 3v5h5" }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // Stats - 1 column
                div {
                    Card {
                        class: "h-full",
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
                                    rect { width: "18", height: "18", x: "3", y: "3", rx: "2", ry: "2" }
                                    circle { cx: "9", cy: "9", r: "2" }
                                    path { d: "m21 15-3.086-3.086a2 2 0 0 0-2.828 0L6 21" }
                                }
                                "资源统计"
                            }
                        }
                        CardContent {
                            class: "space-y-4",
                            div {
                                class: "flex items-center justify-between p-3 rounded-lg bg-slate-50 dark:bg-slate-800/50",
                                div {
                                    p { class: "text-xs text-slate-500 dark:text-slate-400", "资源包" }
                                    p { class: "text-lg font-semibold", "3" }
                                }
                                svg {
                                    xmlns: "http://www.w3.org/2000/svg",
                                    width: "20",
                                    height: "20",
                                    view_box: "0 0 24 24",
                                    fill: "none",
                                    stroke: "currentColor",
                                    stroke_width: "2",
                                    class: "text-blue-500",
                                    rect { width: "18", height: "18", x: "3", y: "3", rx: "2", ry: "2" }
                                    circle { cx: "9", cy: "9", r: "2" }
                                    path { d: "m21 15-3.086-3.086a2 2 0 0 0-2.828 0L6 21" }
                                }
                            }
                            div {
                                class: "flex items-center justify-between p-3 rounded-lg bg-slate-50 dark:bg-slate-800/50",
                                div {
                                    p { class: "text-xs text-slate-500 dark:text-slate-400", "数据包" }
                                    p { class: "text-lg font-semibold", "1" }
                                }
                                svg {
                                    xmlns: "http://www.w3.org/2000/svg",
                                    width: "20",
                                    height: "20",
                                    view_box: "0 0 24 24",
                                    fill: "none",
                                    stroke: "currentColor",
                                    stroke_width: "2",
                                    class: "text-green-500",
                                    path { d: "m7.5 4.27 9 5.15" }
                                    path { d: "M21 8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16Z" }
                                }
                            }
                            div {
                                class: "flex items-center justify-between p-3 rounded-lg bg-slate-50 dark:bg-slate-800/50",
                                div {
                                    p { class: "text-xs text-slate-500 dark:text-slate-400", "已启用" }
                                    p { class: "text-lg font-semibold", "{enabled_count}/{resources.len()}" }
                                }
                                svg {
                                    xmlns: "http://www.w3.org/2000/svg",
                                    width: "20",
                                    height: "20",
                                    view_box: "0 0 24 24",
                                    fill: "none",
                                    stroke: "currentColor",
                                    stroke_width: "2",
                                    class: "text-amber-500",
                                    path { d: "M3 12a9 9 0 1 0 9-9 9.75 9.75 0 0 0-6.74 2.74L3 8" }
                                    path { d: "M3 3v5h5" }
                                }
                            }
                        }
                    }
                }
            }

            // Floating toolbar
            if selected_count > 0 {
                div {
                    class: "fixed bottom-8 left-1/2 transform -translate-x-1/2 z-50",
                    div {
                        class: "bg-white dark:bg-slate-900 border border-slate-200 dark:border-slate-700 rounded-full shadow-2xl p-4 flex items-center gap-3",
                        div {
                            class: "text-sm font-medium px-3 whitespace-nowrap",
                            "已选择 {selected_count}"
                        }
                        div {
                            class: "w-px h-6 bg-slate-200 dark:bg-slate-700"
                        }
                        div {
                            class: "flex gap-2",
                            Button {
                                variant: ButtonVariant::Ghost,
                                class: "gap-1 h-8 px-3 rounded-full hover:bg-slate-100 dark:hover:bg-slate-800",
                                svg {
                                    xmlns: "http://www.w3.org/2000/svg",
                                    width: "16",
                                    height: "16",
                                    view_box: "0 0 24 24",
                                    fill: "none",
                                    stroke: "currentColor",
                                    stroke_width: "2",
                                    path { d: "M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z" }
                                    circle { cx: "12", cy: "12", r: "3" }
                                }
                            }
                            Button {
                                variant: ButtonVariant::Ghost,
                                class: "gap-1 h-8 px-3 rounded-full hover:bg-slate-100 dark:hover:bg-slate-800",
                                svg {
                                    xmlns: "http://www.w3.org/2000/svg",
                                    width: "16",
                                    height: "16",
                                    view_box: "0 0 24 24",
                                    fill: "none",
                                    stroke: "currentColor",
                                    stroke_width: "2",
                                    path { d: "M17.94 17.94A10.07 10.07 0 0 1 12 20c-7 0-11-8-11-8a18.45 18.45 0 0 1 5.06-5.94M9.9 4.24A9.12 9.12 0 0 1 12 4c7 0 11 8 11 8a18.5 18.5 0 0 1-2.16 3.19m-6.72-1.07a3 3 0 1 1-4.24-4.24" }
                                    line { x1: "1", x2: "23", y1: "1", y2: "23" }
                                }
                            }
                            Button {
                                variant: ButtonVariant::Ghost,
                                class: "gap-1 h-8 px-3 rounded-full hover:bg-red-50 dark:hover:bg-red-950/30 text-red-600 hover:text-red-700",
                                svg {
                                    xmlns: "http://www.w3.org/2000/svg",
                                    width: "16",
                                    height: "16",
                                    view_box: "0 0 24 24",
                                    fill: "none",
                                    stroke: "currentColor",
                                    stroke_width: "2",
                                    path { d: "M3 6h18" }
                                    path { d: "M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6" }
                                    path { d: "M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2" }
                                }
                            }
                            Button {
                                variant: ButtonVariant::Ghost,
                                class: "gap-1 h-8 px-3 rounded-full hover:bg-slate-100 dark:hover:bg-slate-800",
                                onclick: move |_| selected_resources.set(vec![]),
                                svg {
                                    xmlns: "http://www.w3.org/2000/svg",
                                    width: "16",
                                    height: "16",
                                    view_box: "0 0 24 24",
                                    fill: "none",
                                    stroke: "currentColor",
                                    stroke_width: "2",
                                    path { d: "M18 6 6 18" }
                                    path { d: "m6 6 12 12" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
