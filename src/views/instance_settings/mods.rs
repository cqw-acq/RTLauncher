use dioxus::prelude::*;
use crate::components::ui::{Card, CardHeader, CardTitle, CardContent, CardDescription, Button, ButtonVariant};

/// Mod data structure
#[derive(Clone, PartialEq)]
struct Mod {
    id: i32,
    name: String,
    version: String,
    author: String,
    category: String,
    enabled: bool,
}

/// Instance mods management page - based on rtl-ui design
#[component]
pub fn InstanceMods() -> Element {
    let mut selected_mods = use_signal(|| Vec::<i32>::new());
    let mut search_query = use_signal(|| String::new());

    // Sample mods data matching rtl-ui
    let mods = vec![
        Mod { id: 1, name: "JEI (Just Enough Items)".to_string(), version: "10.2.1.1004".to_string(), author: "mezz".to_string(), category: "UI".to_string(), enabled: true },
        Mod { id: 2, name: "Sodium".to_string(), version: "0.5.8".to_string(), author: "CaffeineMC".to_string(), category: "优化".to_string(), enabled: true },
        Mod { id: 3, name: "Lithium".to_string(), version: "0.11.2".to_string(), author: "CaffeineMC".to_string(), category: "优化".to_string(), enabled: true },
        Mod { id: 4, name: "Phosphor".to_string(), version: "0.8.1".to_string(), author: "CaffeineMC".to_string(), category: "优化".to_string(), enabled: false },
        Mod { id: 5, name: "Botania".to_string(), version: "438".to_string(), author: "Vazkii".to_string(), category: "魔法".to_string(), enabled: true },
    ];

    let filtered_mods: Vec<&Mod> = mods.iter()
        .filter(|m| m.name.to_lowercase().contains(&search_query().to_lowercase()) || 
                    m.author.to_lowercase().contains(&search_query().to_lowercase()))
        .collect();

    let enabled_count = mods.iter().filter(|m| m.enabled).count();
    let selected_count = selected_mods().len();

    rsx! {
        div {
            class: "space-y-6",
            // Header - matching rtl-ui exactly
            div {
                class: "mb-6 flex flex-col md:flex-row md:items-center md:justify-between gap-4",
                h1 {
                    class: "text-3xl font-bold leading-tight",
                    "模组管理"
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
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
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
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            class: "mr-1",
                            path { d: "M5 12h14" }
                            path { d: "M12 5v14" }
                        }
                        "添加模组"
                    }
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
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            class: "mr-1",
                            path { d: "M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" }
                            polyline { points: "7 10 12 15 17 10" }
                            line { x1: "12", x2: "12", y1: "15", y2: "3" }
                        }
                        "导出模组信息"
                    }
                }
            }

            // Main content - 4 column grid like rtl-ui
            div {
                class: "grid grid-cols-1 lg:grid-cols-4 gap-6",
                // Installed mods - 3 columns
                div {
                    class: "lg:col-span-3",
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
                                    path { d: "m7.5 4.27 9 5.15" }
                                    path { d: "M21 8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16Z" }
                                    path { d: "m3.3 7 8.7 5 8.7-5" }
                                    path { d: "M12 22V12" }
                                }
                                "已安装模组"
                            }
                            CardDescription { "共 {mods.len()} 个模组，已启用 {enabled_count} 个" }
                            div {
                                class: "mt-4",
                                input {
                                    r#type: "text",
                                    class: "max-w-sm px-3 py-2 border rounded-md bg-background w-full",
                                    placeholder: "搜索模组...",
                                    value: "{search_query}",
                                    oninput: move |e| search_query.set(e.value())
                                }
                            }
                        }
                        CardContent {
                            class: "space-y-2",
                            for m in filtered_mods.iter() {
                                div {
                                    key: "{m.id}",
                                    class: if selected_mods().contains(&m.id) {
                                        "group flex items-center justify-between p-4 rounded-lg border cursor-pointer transition-all duration-200 bg-blue-50 dark:bg-blue-950/30 border-blue-300 dark:border-blue-700"
                                    } else {
                                        "group flex items-center justify-between p-4 rounded-lg border cursor-pointer transition-all duration-200 border-slate-200 dark:border-slate-700 hover:bg-slate-50 dark:hover:bg-slate-800/40"
                                    },
                                    onclick: {
                                        let mod_id = m.id;
                                        move |_| {
                                            let mut current = selected_mods();
                                            if current.contains(&mod_id) {
                                                current.retain(|&x| x != mod_id);
                                            } else {
                                                current.push(mod_id);
                                            }
                                            selected_mods.set(current);
                                        }
                                    },
                                    div {
                                        class: "flex items-center gap-4 flex-1",
                                        // Checkbox
                                        div {
                                            class: if selected_mods().contains(&m.id) {
                                                "w-5 h-5 rounded-md border-2 transition-all flex items-center justify-center bg-blue-500 border-blue-500"
                                            } else {
                                                "w-5 h-5 rounded-md border-2 transition-all flex items-center justify-center border-slate-300 dark:border-slate-600"
                                            },
                                            if selected_mods().contains(&m.id) {
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
                                        // Mod info
                                        div {
                                            class: "flex-1",
                                            div {
                                                class: "flex items-center gap-2",
                                                h3 {
                                                    class: "font-semibold text-sm",
                                                    "{m.name}"
                                                }
                                                span {
                                                    class: "px-2 py-0.5 text-xs rounded-full bg-secondary text-secondary-foreground",
                                                    "{m.category}"
                                                }
                                            }
                                            p {
                                                class: "text-xs text-slate-500 dark:text-slate-400 mt-1",
                                                "版本：{m.version} • 作者：{m.author}"
                                            }
                                        }
                                    }
                                    // Hover buttons
                                    div {
                                        class: "flex gap-2 opacity-0 group-hover:opacity-100 transition-opacity duration-200",
                                        Button {
                                            variant: ButtonVariant::Ghost,
                                            class: "gap-1 h-8 px-2",
                                            if m.enabled {
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
                                            } else {
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

                // Categories - 1 column
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
                                    rect { x: "3", y: "3", width: "7", height: "7" }
                                    rect { x: "14", y: "3", width: "7", height: "7" }
                                    rect { x: "14", y: "14", width: "7", height: "7" }
                                    rect { x: "3", y: "14", width: "7", height: "7" }
                                }
                                "模组分类"
                            }
                        }
                        CardContent {
                            class: "space-y-3",
                            div {
                                class: "flex items-center justify-between p-2 rounded-lg hover:bg-slate-50 dark:hover:bg-slate-800 transition-colors cursor-pointer",
                                span { class: "text-sm font-medium", "优化" }
                                span { class: "px-2 py-0.5 text-xs rounded-full bg-secondary text-secondary-foreground", "15" }
                            }
                            div {
                                class: "flex items-center justify-between p-2 rounded-lg hover:bg-slate-50 dark:hover:bg-slate-800 transition-colors cursor-pointer",
                                span { class: "text-sm font-medium", "界面" }
                                span { class: "px-2 py-0.5 text-xs rounded-full bg-secondary text-secondary-foreground", "8" }
                            }
                            div {
                                class: "flex items-center justify-between p-2 rounded-lg hover:bg-slate-50 dark:hover:bg-slate-800 transition-colors cursor-pointer",
                                span { class: "text-sm font-medium", "世界生成" }
                                span { class: "px-2 py-0.5 text-xs rounded-full bg-secondary text-secondary-foreground", "12" }
                            }
                            div {
                                class: "flex items-center justify-between p-2 rounded-lg hover:bg-slate-50 dark:hover:bg-slate-800 transition-colors cursor-pointer",
                                span { class: "text-sm font-medium", "魔法" }
                                span { class: "px-2 py-0.5 text-xs rounded-full bg-secondary text-secondary-foreground", "6" }
                            }
                            div {
                                class: "flex items-center justify-between p-2 rounded-lg hover:bg-slate-50 dark:hover:bg-slate-800 transition-colors cursor-pointer",
                                span { class: "text-sm font-medium", "科技" }
                                span { class: "px-2 py-0.5 text-xs rounded-full bg-secondary text-secondary-foreground", "18" }
                            }
                        }
                    }
                }
            }

            // Floating toolbar when items are selected - matching rtl-ui island style
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
                                    path { d: "M17.94 17.94A10.07 10.07 0 0 1 12 20c-7 0-11-8-11-8a18.45 18.45 0 0 1 5.06-5.94M9.9 4.24A9.12 9.12 0 0 1 12 4c7 0 11 8 11 8a18.5 18.5 0 0 1-2.16 3.19m-6.72-1.07a3 3 0 1 1-4.24-4.24" }
                                    line { x1: "1", x2: "23", y1: "1", y2: "23" }
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
                                    path { d: "M3 12a9 9 0 1 0 9-9 9.75 9.75 0 0 0-6.74 2.74L3 8" }
                                    path { d: "M3 3v5h5" }
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
                                svg {
                                    xmlns: "http://www.w3.org/2000/svg",
                                    width: "16",
                                    height: "16",
                                    view_box: "0 0 24 24",
                                    fill: "none",
                                    stroke: "currentColor",
                                    stroke_width: "2",
                                    polygon { points: "12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2" }
                                }
                            }
                            Button {
                                variant: ButtonVariant::Ghost,
                                class: "gap-1 h-8 px-3 rounded-full hover:bg-slate-100 dark:hover:bg-slate-800",
                                onclick: move |_| selected_mods.set(vec![]),
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
