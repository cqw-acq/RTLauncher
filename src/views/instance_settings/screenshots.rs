use dioxus::prelude::*;
use crate::components::ui::{Card, CardHeader, CardTitle, CardContent, CardDescription, Button, ButtonVariant};

/// Screenshot data structure
#[derive(Clone, PartialEq)]
struct Screenshot {
    id: i32,
    name: String,
    date: String,
    size: String,
    resolution: String,
}

/// Instance screenshots page - based on rtl-ui design
#[component]
pub fn InstanceScreenshots() -> Element {
    let mut selected_screenshots = use_signal(|| Vec::<i32>::new());
    let mut search_query = use_signal(|| String::new());

    let screenshots = vec![
        Screenshot { id: 1, name: "2025-08-31_001".to_string(), date: "2天前".to_string(), size: "2MB".to_string(), resolution: "1920x1080".to_string() },
        Screenshot { id: 2, name: "2025-08-30_042".to_string(), date: "3天前".to_string(), size: "3MB".to_string(), resolution: "1920x1080".to_string() },
        Screenshot { id: 3, name: "2025-08-28_015".to_string(), date: "5天前".to_string(), size: "2.5MB".to_string(), resolution: "1920x1080".to_string() },
        Screenshot { id: 4, name: "2025-08-25_089".to_string(), date: "1周前".to_string(), size: "2.2MB".to_string(), resolution: "1920x1080".to_string() },
        Screenshot { id: 5, name: "2025-08-20_156".to_string(), date: "2周前".to_string(), size: "2.8MB".to_string(), resolution: "2560x1440".to_string() },
        Screenshot { id: 6, name: "2025-08-15_203".to_string(), date: "3周前".to_string(), size: "2.1MB".to_string(), resolution: "1920x1080".to_string() },
    ];

    let selected_count = selected_screenshots().len();

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
                        "导入截图"
                    }
                }
            }

            // Main content - 4 column grid
            div {
                class: "grid grid-cols-1 lg:grid-cols-4 gap-6",
                // Screenshots grid - 3 columns
                div {
                    class: "lg:col-span-3",
                    Card {
                        CardHeader {
                            div {
                                class: "flex flex-col md:flex-row md:items-center md:justify-between gap-4",
                                div {
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
                                        "所有截图"
                                    }
                                    CardDescription { "共 {screenshots.len()} 张截图" }
                                }
                                div {
                                    class: "flex gap-2",
                                    select {
                                        class: "px-3 py-2 border rounded-md bg-background text-sm",
                                        option { value: "newest", "最新优先" }
                                        option { value: "oldest", "最早优先" }
                                        option { value: "name", "名称排序" }
                                    }
                                }
                            }
                            div {
                                class: "mt-4",
                                input {
                                    r#type: "text",
                                    class: "max-w-sm px-3 py-2 border rounded-md bg-background w-full",
                                    placeholder: "搜索截图...",
                                    value: "{search_query}",
                                    oninput: move |e| search_query.set(e.value())
                                }
                            }
                        }
                        CardContent {
                            div {
                                class: "grid grid-cols-2 md:grid-cols-3 gap-4",
                                for s in screenshots.iter() {
                                    div {
                                        key: "{s.id}",
                                        class: if selected_screenshots().contains(&s.id) {
                                            "group relative cursor-pointer rounded-lg border overflow-hidden transition-all duration-200 ring-2 ring-blue-500 border-blue-300 dark:border-blue-700"
                                        } else {
                                            "group relative cursor-pointer rounded-lg border overflow-hidden transition-all duration-200 border-slate-200 dark:border-slate-700 hover:border-slate-300 dark:hover:border-slate-600"
                                        },
                                        onclick: {
                                            let screenshot_id = s.id;
                                            move |_| {
                                                let mut current = selected_screenshots();
                                                if current.contains(&screenshot_id) {
                                                    current.retain(|&x| x != screenshot_id);
                                                } else {
                                                    current.push(screenshot_id);
                                                }
                                                selected_screenshots.set(current);
                                            }
                                        },
                                        // Thumbnail placeholder
                                        div {
                                            class: "aspect-video bg-gradient-to-br from-slate-200 to-slate-300 dark:from-slate-700 dark:to-slate-800 flex items-center justify-center relative",
                                            svg {
                                                xmlns: "http://www.w3.org/2000/svg",
                                                width: "32",
                                                height: "32",
                                                view_box: "0 0 24 24",
                                                fill: "none",
                                                stroke: "currentColor",
                                                stroke_width: "2",
                                                class: "text-slate-400",
                                                rect { width: "18", height: "18", x: "3", y: "3", rx: "2", ry: "2" }
                                                circle { cx: "9", cy: "9", r: "2" }
                                                path { d: "m21 15-3.086-3.086a2 2 0 0 0-2.828 0L6 21" }
                                            }
                                            // Selection overlay
                                            if selected_screenshots().contains(&s.id) {
                                                div {
                                                    class: "absolute inset-0 bg-blue-500/20 flex items-center justify-center",
                                                    div {
                                                        class: "w-6 h-6 bg-blue-500 rounded-full border-2 border-white flex items-center justify-center",
                                                        svg {
                                                            class: "w-4 h-4 text-white",
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
                                            }
                                        }
                                        // Info
                                        div {
                                            class: "p-3",
                                            div {
                                                class: "mb-2",
                                                h3 {
                                                    class: "font-medium text-sm truncate",
                                                    "{s.name}"
                                                }
                                                p {
                                                    class: "text-xs text-slate-500 dark:text-slate-400",
                                                    "{s.size} • {s.resolution}"
                                                }
                                            }
                                            span {
                                                class: "px-2 py-0.5 text-xs rounded-full bg-secondary text-secondary-foreground",
                                                "{s.date}"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // Right side - filters and stats
                div {
                    class: "space-y-4",
                    // Filter
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
                                "筛选"
                            }
                        }
                        CardContent {
                            class: "space-y-4",
                            div {
                                label { class: "text-sm font-medium mb-2 block", "日期范围" }
                                select {
                                    class: "w-full px-3 py-2 border rounded-md bg-background text-sm",
                                    option { value: "all", "全部时间" }
                                    option { value: "today", "今天" }
                                    option { value: "week", "本周" }
                                    option { value: "month", "本月" }
                                }
                            }
                            div {
                                label { class: "text-sm font-medium mb-2 block", "分辨率" }
                                select {
                                    class: "w-full px-3 py-2 border rounded-md bg-background text-sm",
                                    option { value: "all", "全部分辨率" }
                                    option { value: "1080p", "1920x1080" }
                                    option { value: "2k", "2560x1440" }
                                    option { value: "4k", "3840x2160" }
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
                                onclick: {
                                    let all_ids: Vec<i32> = screenshots.iter().map(|s| s.id).collect();
                                    move |_| selected_screenshots.set(all_ids.clone())
                                },
                                "全选"
                            }
                            Button {
                                variant: ButtonVariant::Outline,
                                class: "w-full",
                                onclick: move |_| selected_screenshots.set(vec![]),
                                "取消选择"
                            }
                        }
                    }

                    // Stats
                    Card {
                        CardHeader {
                            CardTitle { "统计信息" }
                        }
                        CardContent {
                            class: "space-y-3",
                            div {
                                class: "text-sm",
                                p { class: "text-slate-500 dark:text-slate-400 mb-1", "总截图数" }
                                p { class: "text-2xl font-bold", "{screenshots.len()}" }
                            }
                            div {
                                class: "text-sm",
                                p { class: "text-slate-500 dark:text-slate-400 mb-1", "已选择" }
                                p { class: "text-lg font-semibold", "{selected_count}" }
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
                                    circle { cx: "18", cy: "5", r: "3" }
                                    circle { cx: "6", cy: "12", r: "3" }
                                    circle { cx: "18", cy: "19", r: "3" }
                                    line { x1: "8.59", x2: "15.42", y1: "13.51", y2: "17.49" }
                                    line { x1: "15.41", x2: "8.59", y1: "6.51", y2: "10.49" }
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
                                onclick: move |_| selected_screenshots.set(vec![]),
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
