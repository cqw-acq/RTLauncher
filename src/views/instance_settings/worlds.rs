use dioxus::prelude::*;
use crate::components::ui::{Card, CardHeader, CardTitle, CardContent, CardDescription, Button, ButtonVariant};

/// World data structure
#[derive(Clone, PartialEq)]
struct World {
    id: i32,
    name: String,
    mode: String,
    version: String,
    play_time: String,
    last_played: String,
    size: String,
}

/// Instance worlds management page - based on rtl-ui design
#[component]
pub fn InstanceWorlds() -> Element {
    let mut selected_worlds = use_signal(|| Vec::<i32>::new());
    let mut search_query = use_signal(|| String::new());

    // Sample worlds data matching rtl-ui
    let worlds = vec![
        World { id: 1, name: "RTL World".to_string(), mode: "生存".to_string(), version: "1.21.8".to_string(), play_time: "24小时".to_string(), last_played: "今天".to_string(), size: "450MB".to_string() },
        World { id: 2, name: "Creative World".to_string(), mode: "创造".to_string(), version: "1.21.8".to_string(), play_time: "12小时".to_string(), last_played: "2天前".to_string(), size: "280MB".to_string() },
        World { id: 3, name: "Adventure Map".to_string(), mode: "冒险".to_string(), version: "1.21.7".to_string(), play_time: "8小时".to_string(), last_played: "1周前".to_string(), size: "320MB".to_string() },
        World { id: 4, name: "Skyblock".to_string(), mode: "生存".to_string(), version: "1.21.8".to_string(), play_time: "36小时".to_string(), last_played: "5天前".to_string(), size: "350MB".to_string() },
        World { id: 5, name: "Testing".to_string(), mode: "创造".to_string(), version: "1.21.8".to_string(), play_time: "4小时".to_string(), last_played: "3天前".to_string(), size: "150MB".to_string() },
        World { id: 6, name: "Old Backup".to_string(), mode: "生存".to_string(), version: "1.20.4".to_string(), play_time: "120小时".to_string(), last_played: "2周前".to_string(), size: "500MB".to_string() },
    ];

    let filtered_worlds: Vec<&World> = worlds.iter()
        .filter(|w| w.name.to_lowercase().contains(&search_query().to_lowercase()))
        .collect();

    let selected_count = selected_worlds().len();

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
                            path { d: "M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" }
                            polyline { points: "7 10 12 15 17 10" }
                            line { x1: "12", x2: "12", y1: "15", y2: "3" }
                        }
                        "导入存档"
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
                            class: "mr-1",
                            path { d: "M5 12h14" }
                            path { d: "M12 5v14" }
                        }
                        "创建新世界"
                    }
                }
            }

            // Main content - 3 column grid
            div {
                class: "grid grid-cols-1 lg:grid-cols-3 gap-6",
                // World list - 2 columns
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
                                    circle { cx: "12", cy: "12", r: "10" }
                                    path { d: "M12 2a14.5 14.5 0 0 0 0 20 14.5 14.5 0 0 0 0-20" }
                                    path { d: "M2 12h20" }
                                }
                                "已创建存档"
                            }
                            CardDescription { "共 {worlds.len()} 个世界" }
                            div {
                                class: "mt-4",
                                input {
                                    r#type: "text",
                                    class: "max-w-sm px-3 py-2 border rounded-md bg-background w-full",
                                    placeholder: "搜索存档...",
                                    value: "{search_query}",
                                    oninput: move |e| search_query.set(e.value())
                                }
                            }
                        }
                        CardContent {
                            class: "space-y-2",
                            for w in filtered_worlds.iter() {
                                div {
                                    key: "{w.id}",
                                    class: if selected_worlds().contains(&w.id) {
                                        "group flex items-center justify-between p-4 rounded-lg border cursor-pointer transition-all duration-200 bg-blue-50 dark:bg-blue-950/30 border-blue-300 dark:border-blue-700"
                                    } else {
                                        "group flex items-center justify-between p-4 rounded-lg border cursor-pointer transition-all duration-200 border-slate-200 dark:border-slate-700 hover:bg-slate-50 dark:hover:bg-slate-800/40"
                                    },
                                    onclick: {
                                        let world_id = w.id;
                                        move |_| {
                                            let mut current = selected_worlds();
                                            if current.contains(&world_id) {
                                                current.retain(|&x| x != world_id);
                                            } else {
                                                current.push(world_id);
                                            }
                                            selected_worlds.set(current);
                                        }
                                    },
                                    div {
                                        class: "flex items-center gap-4 flex-1",
                                        // Checkbox
                                        div {
                                            class: if selected_worlds().contains(&w.id) {
                                                "w-5 h-5 rounded-md border-2 transition-all flex items-center justify-center bg-blue-500 border-blue-500"
                                            } else {
                                                "w-5 h-5 rounded-md border-2 transition-all flex items-center justify-center border-slate-300 dark:border-slate-600"
                                            },
                                            if selected_worlds().contains(&w.id) {
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
                                        // World info
                                        div {
                                            class: "flex-1",
                                            div {
                                                class: "flex items-center gap-2",
                                                h3 {
                                                    class: "font-semibold text-sm",
                                                    "{w.name}"
                                                }
                                                span {
                                                    class: "px-2 py-0.5 text-xs rounded-full bg-secondary text-secondary-foreground",
                                                    "{w.mode}"
                                                }
                                            }
                                            p {
                                                class: "text-xs text-slate-500 dark:text-slate-400 mt-1",
                                                "版本：{w.version} • 游戏时间：{w.play_time} • 最后游戏：{w.last_played}"
                                            }
                                        }
                                        div {
                                            class: "text-xs text-slate-500 dark:text-slate-400",
                                            "{w.size}"
                                        }
                                    }
                                    // Hover buttons
                                    div {
                                        class: "flex gap-2 opacity-0 group-hover:opacity-100 transition-opacity duration-200 ml-2",
                                        Button {
                                            variant: ButtonVariant::Ghost,
                                            class: "gap-1 h-8 px-2",
                                            "进入"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // Right side info cards
                div {
                    class: "space-y-4",
                    // Storage info
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
                                    line { x1: "22", x2: "2", y1: "12", y2: "12" }
                                    path { d: "M5.45 5.11 2 12v6a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-6l-3.45-6.89A2 2 0 0 0 16.76 4H7.24a2 2 0 0 0-1.79 1.11z" }
                                    line { x1: "6", x2: "6.01", y1: "16", y2: "16" }
                                    line { x1: "10", x2: "10.01", y1: "16", y2: "16" }
                                }
                                "存储信息"
                            }
                        }
                        CardContent {
                            class: "space-y-4",
                            div {
                                div {
                                    class: "flex justify-between mb-2 text-sm",
                                    span { "存储空间" }
                                    span { "2.0GB / 10GB" }
                                }
                                div {
                                    class: "w-full bg-slate-200 dark:bg-slate-700 rounded-full h-2",
                                    div {
                                        class: "bg-blue-500 h-2 rounded-full",
                                        style: "width: 20%"
                                    }
                                }
                            }
                            div {
                                class: "space-y-2 text-sm",
                                div {
                                    class: "flex justify-between",
                                    span { class: "text-slate-600 dark:text-slate-400", "总存档数" }
                                    span { class: "font-semibold", "{worlds.len()}" }
                                }
                                div {
                                    class: "flex justify-between",
                                    span { class: "text-slate-600 dark:text-slate-400", "已选择" }
                                    span { class: "font-semibold", "{selected_count}" }
                                }
                            }
                        }
                    }

                    // Recent activity
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
                                    rect { width: "18", height: "18", x: "3", y: "4", rx: "2", ry: "2" }
                                    line { x1: "16", x2: "16", y1: "2", y2: "6" }
                                    line { x1: "8", x2: "8", y1: "2", y2: "6" }
                                    line { x1: "3", x2: "21", y1: "10", y2: "10" }
                                }
                                "最近活动"
                            }
                        }
                        CardContent {
                            class: "space-y-2 text-sm",
                            div {
                                class: "flex justify-between",
                                span { "最近游玩" }
                                span { class: "text-slate-500", "RTL World" }
                            }
                            div {
                                class: "flex justify-between",
                                span { "总游戏时长" }
                                span { class: "font-semibold", "204小时" }
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
                                onclick: move |_| {
                                    selected_worlds.set(worlds.iter().map(|w| w.id).collect());
                                },
                                "全选"
                            }
                            Button {
                                variant: ButtonVariant::Outline,
                                class: "w-full",
                                onclick: move |_| selected_worlds.set(vec![]),
                                "取消选择"
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
                                    rect { width: "14", height: "14", x: "8", y: "8", rx: "2", ry: "2" }
                                    path { d: "M4 16c-1.1 0-2-.9-2-2V4c0-1.1.9-2 2-2h10c1.1 0 2 .9 2 2" }
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
                                    path { d: "M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" }
                                    polyline { points: "7 10 12 15 17 10" }
                                    line { x1: "12", x2: "12", y1: "15", y2: "3" }
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
                                onclick: move |_| selected_worlds.set(vec![]),
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
