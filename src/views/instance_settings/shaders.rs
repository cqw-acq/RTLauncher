use dioxus::prelude::*;
use crate::components::ui::{Card, CardHeader, CardTitle, CardContent, CardDescription, Button, ButtonVariant};

/// Shader data structure
#[derive(Clone, PartialEq)]
struct Shader {
    id: i32,
    name: String,
    version: String,
    author: String,
    size: String,
    enabled: bool,
    quality: String,
}

/// Instance shaders management page - based on rtl-ui design
#[component]
pub fn InstanceShaders() -> Element {
    let mut selected_shaders = use_signal(|| Vec::<i32>::new());
    let mut search_query = use_signal(|| String::new());

    let shaders = vec![
        Shader { id: 1, name: "BSL Shaders".to_string(), version: "8.2.04".to_string(), author: "Capt Tatsu".to_string(), size: "45MB".to_string(), enabled: true, quality: "极高".to_string() },
        Shader { id: 2, name: "SEUS Renewed".to_string(), version: "1.0.1".to_string(), author: "sonicether".to_string(), size: "32MB".to_string(), enabled: false, quality: "高".to_string() },
        Shader { id: 3, name: "Complementary".to_string(), version: "4.7.1".to_string(), author: "EminGTR".to_string(), size: "52MB".to_string(), enabled: false, quality: "超高".to_string() },
        Shader { id: 4, name: "Chocapic13".to_string(), version: "9.1".to_string(), author: "Chocapic13".to_string(), size: "28MB".to_string(), enabled: true, quality: "中".to_string() },
        Shader { id: 5, name: "RRe36".to_string(), version: "7.1.4".to_string(), author: "RRe36".to_string(), size: "38MB".to_string(), enabled: false, quality: "高".to_string() },
    ];

    let filtered_shaders: Vec<&Shader> = shaders.iter()
        .filter(|s| s.name.to_lowercase().contains(&search_query().to_lowercase()) ||
                    s.author.to_lowercase().contains(&search_query().to_lowercase()))
        .collect();

    let enabled_count = shaders.iter().filter(|s| s.enabled).count();
    let selected_count = selected_shaders().len();

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
                        variant: ButtonVariant::Default,
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
                        "添加光影"
                    }
                }
            }

            // Main content - 3 column grid
            div {
                class: "grid grid-cols-1 lg:grid-cols-3 gap-6",
                // Shaders list - 2 columns
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
                                    path { d: "M13 2 3 14h9l-1 8 10-12h-9l1-8z" }
                                }
                                "已安装光影"
                            }
                            CardDescription { "共 {shaders.len()} 个光影，已启用 {enabled_count} 个" }
                            div {
                                class: "mt-4",
                                input {
                                    r#type: "text",
                                    class: "max-w-sm px-3 py-2 border rounded-md bg-background w-full",
                                    placeholder: "搜索光影...",
                                    value: "{search_query}",
                                    oninput: move |e| search_query.set(e.value())
                                }
                            }
                        }
                        CardContent {
                            class: "space-y-2",
                            for s in filtered_shaders.iter() {
                                div {
                                    key: "{s.id}",
                                    class: if selected_shaders().contains(&s.id) {
                                        "group flex items-center justify-between p-4 rounded-lg border cursor-pointer transition-all duration-200 bg-blue-50 dark:bg-blue-950/30 border-blue-300 dark:border-blue-700"
                                    } else {
                                        "group flex items-center justify-between p-4 rounded-lg border cursor-pointer transition-all duration-200 border-slate-200 dark:border-slate-700 hover:bg-slate-50 dark:hover:bg-slate-800/40"
                                    },
                                    onclick: {
                                        let shader_id = s.id;
                                        move |_| {
                                            let mut current = selected_shaders();
                                            if current.contains(&shader_id) {
                                                current.retain(|&x| x != shader_id);
                                            } else {
                                                current.push(shader_id);
                                            }
                                            selected_shaders.set(current);
                                        }
                                    },
                                    div {
                                        class: "flex items-center gap-4 flex-1",
                                        // Checkbox
                                        div {
                                            class: if selected_shaders().contains(&s.id) {
                                                "w-5 h-5 rounded-md border-2 transition-all flex items-center justify-center bg-blue-500 border-blue-500"
                                            } else {
                                                "w-5 h-5 rounded-md border-2 transition-all flex items-center justify-center border-slate-300 dark:border-slate-600"
                                            },
                                            if selected_shaders().contains(&s.id) {
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
                                        // Shader info
                                        div {
                                            class: "flex-1",
                                            div {
                                                class: "flex items-center gap-2",
                                                h3 {
                                                    class: "font-semibold text-sm",
                                                    "{s.name}"
                                                }
                                                if s.enabled {
                                                    span {
                                                        class: "px-2 py-0.5 text-xs rounded-full bg-green-500 text-white",
                                                        "启用中"
                                                    }
                                                }
                                                span {
                                                    class: "px-2 py-0.5 text-xs rounded-full bg-secondary text-secondary-foreground",
                                                    "{s.quality}"
                                                }
                                            }
                                            p {
                                                class: "text-xs text-slate-500 dark:text-slate-400 mt-1",
                                                "版本：{s.version} • 作者：{s.author} • 大小：{s.size}"
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
                                                path { d: "M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z" }
                                                circle { cx: "12", cy: "12", r: "3" }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // Stats and quick actions
                div {
                    class: "space-y-4",
                    // Stats
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
                                    path { d: "M13 2 3 14h9l-1 8 10-12h-9l1-8z" }
                                }
                                "光影统计"
                            }
                        }
                        CardContent {
                            class: "space-y-4",
                            div {
                                class: "flex items-center justify-between p-3 rounded-lg bg-slate-50 dark:bg-slate-800/50",
                                div {
                                    p { class: "text-xs text-slate-500 dark:text-slate-400", "已安装光影" }
                                    p { class: "text-lg font-semibold", "{shaders.len()}" }
                                }
                                svg {
                                    xmlns: "http://www.w3.org/2000/svg",
                                    width: "20",
                                    height: "20",
                                    view_box: "0 0 24 24",
                                    fill: "none",
                                    stroke: "currentColor",
                                    stroke_width: "2",
                                    class: "text-yellow-500",
                                    path { d: "M13 2 3 14h9l-1 8 10-12h-9l1-8z" }
                                }
                            }
                            div {
                                class: "flex items-center justify-between p-3 rounded-lg bg-slate-50 dark:bg-slate-800/50",
                                div {
                                    p { class: "text-xs text-slate-500 dark:text-slate-400", "已启用" }
                                    p { class: "text-lg font-semibold", "{enabled_count}/{shaders.len()}" }
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
                                    path { d: "M13 2 3 14h9l-1 8 10-12h-9l1-8z" }
                                }
                            }
                            div {
                                class: "flex items-center justify-between p-3 rounded-lg bg-slate-50 dark:bg-slate-800/50",
                                div {
                                    p { class: "text-xs text-slate-500 dark:text-slate-400", "总大小" }
                                    p { class: "text-lg font-semibold", "195MB" }
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
                                    path { d: "m7.5 4.27 9 5.15" }
                                    path { d: "M21 8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16Z" }
                                }
                            }
                        }
                    }

                    // Quick actions
                    Card {
                        CardHeader {
                            CardTitle { "快速操作" }
                        }
                        CardContent {
                            class: "space-y-2",
                            Button {
                                variant: ButtonVariant::Outline,
                                class: "w-full",
                                "打开光影文件夹"
                            }
                            Button {
                                variant: ButtonVariant::Outline,
                                class: "w-full",
                                "检查更新"
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
                                onclick: move |_| selected_shaders.set(vec![]),
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
