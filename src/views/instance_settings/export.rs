use dioxus::prelude::*;
use crate::components::ui::{Card, CardHeader, CardTitle, CardContent, CardDescription, Button, ButtonVariant};

/// Export item data structure
#[derive(Clone, PartialEq)]
struct ExportItem {
    id: String,
    name: String,
    size: String,
    count: String,
}

/// Instance export page - based on rtl-ui design
#[component]
pub fn InstanceExport() -> Element {
    let mut selected_items = use_signal(|| vec![
        "mods".to_string(),
        "config".to_string(),
        "resourcepacks".to_string(),
        "shaderpacks".to_string(),
        "worlds".to_string(),
    ]);

    let export_items = vec![
        ExportItem { id: "mods".to_string(), name: "模组文件".to_string(), size: "1.2GB".to_string(), count: "72 个文件".to_string() },
        ExportItem { id: "config".to_string(), name: "配置文件".to_string(), size: "8MB".to_string(), count: "15 个文件".to_string() },
        ExportItem { id: "resourcepacks".to_string(), name: "资源包".to_string(), size: "256MB".to_string(), count: "4 个文件".to_string() },
        ExportItem { id: "shaderpacks".to_string(), name: "光影包".to_string(), size: "128MB".to_string(), count: "3 个文件".to_string() },
        ExportItem { id: "worlds".to_string(), name: "存档文件".to_string(), size: "512MB".to_string(), count: "6 个世界".to_string() },
    ];

    let selected_count = selected_items().len();
    let total_count = export_items.len();
    let estimated_size = selected_count * 384 / 5;

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
                div {
                    class: "flex gap-3",
                    Button {
                        variant: ButtonVariant::Outline,
                        class: "px-6 py-2",
                        onclick: move |_| selected_items.set(vec![]),
                        "清空选择"
                    }
                    Button {
                        variant: ButtonVariant::Outline,
                        class: "px-6 py-2",
                        onclick: {
                            let items = export_items.clone();
                            move |_| selected_items.set(items.iter().map(|i| i.id.clone()).collect())
                        },
                        "全选"
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
                            path { d: "M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" }
                            polyline { points: "7 10 12 15 17 10" }
                            line { x1: "12", x2: "12", y1: "15", y2: "3" }
                        }
                        "开始导出"
                    }
                }
            }

            div {
                class: "grid grid-cols-1 lg:grid-cols-3 gap-6",
                // Main content - 2 columns
                div {
                    class: "lg:col-span-2 space-y-6",
                    // Export options card
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
                                "导出选项"
                            }
                            CardDescription { "选择要导出的内容，共 {selected_count}/{total_count} 项" }
                        }
                        CardContent {
                            class: "space-y-3",
                            for item in export_items.iter() {
                                div {
                                    key: "{item.id}",
                                    class: if selected_items().contains(&item.id) {
                                        "group flex items-center justify-between p-4 rounded-lg border cursor-pointer transition-all duration-200 bg-blue-50 dark:bg-blue-950/30 border-blue-300 dark:border-blue-700"
                                    } else {
                                        "group flex items-center justify-between p-4 rounded-lg border cursor-pointer transition-all duration-200 border-slate-200 dark:border-slate-700 hover:bg-slate-50 dark:hover:bg-slate-800/40"
                                    },
                                    onclick: {
                                        let item_id = item.id.clone();
                                        move |_| {
                                            let mut current = selected_items();
                                            if current.contains(&item_id) {
                                                current.retain(|x| x != &item_id);
                                            } else {
                                                current.push(item_id.clone());
                                            }
                                            selected_items.set(current);
                                        }
                                    },
                                    div {
                                        class: "flex items-center gap-4 flex-1",
                                        // Checkbox
                                        div {
                                            class: if selected_items().contains(&item.id) {
                                                "w-5 h-5 rounded-md border-2 transition-all flex items-center justify-center bg-blue-500 border-blue-500"
                                            } else {
                                                "w-5 h-5 rounded-md border-2 transition-all flex items-center justify-center border-slate-300 dark:border-slate-600"
                                            },
                                            if selected_items().contains(&item.id) {
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
                                        // Item info
                                        div {
                                            class: "flex-1",
                                            div {
                                                class: "flex items-center gap-3",
                                                h3 {
                                                    class: "font-semibold text-sm",
                                                    "{item.name}"
                                                }
                                                span {
                                                    class: "px-2 py-0.5 text-xs rounded-full bg-secondary text-secondary-foreground",
                                                    "{item.size}"
                                                }
                                            }
                                            p {
                                                class: "text-xs text-slate-500 dark:text-slate-400 mt-1",
                                                "{item.count}"
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    // Export progress card
                    Card {
                        CardHeader {
                            CardTitle { "导出进度" }
                            CardDescription { "估计导出文件大小和时间" }
                        }
                        CardContent {
                            class: "space-y-4",
                            div {
                                div {
                                    class: "flex justify-between mb-2",
                                    span { class: "text-sm font-medium", "总大小" }
                                    span { class: "text-sm font-semibold", "{estimated_size}MB" }
                                }
                                div {
                                    class: "w-full bg-slate-200 dark:bg-slate-700 rounded-full h-2",
                                    div {
                                        class: "bg-blue-500 h-2 rounded-full transition-all",
                                        style: "width: {selected_count * 100 / total_count}%"
                                    }
                                }
                            }
                            div {
                                class: "grid grid-cols-2 gap-4 pt-2",
                                div {
                                    p { class: "text-xs text-slate-500 dark:text-slate-400 mb-1", "预计导出时间" }
                                    p { class: "text-lg font-semibold", "{(estimated_size / 512).max(1)}分钟" }
                                }
                                div {
                                    p { class: "text-xs text-slate-500 dark:text-slate-400 mb-1", "平均速度" }
                                    p { class: "text-lg font-semibold", "~8.5MB/s" }
                                }
                            }
                            // Alert
                            div {
                                class: "flex items-start gap-3 p-4 rounded-lg border border-amber-200 bg-amber-50 dark:border-amber-800 dark:bg-amber-950/30",
                                svg {
                                    xmlns: "http://www.w3.org/2000/svg",
                                    width: "20",
                                    height: "20",
                                    view_box: "0 0 24 24",
                                    fill: "none",
                                    stroke: "currentColor",
                                    stroke_width: "2",
                                    class: "text-amber-500 shrink-0",
                                    circle { cx: "12", cy: "12", r: "10" }
                                    line { x1: "12", x2: "12", y1: "8", y2: "12" }
                                    line { x1: "12", x2: "12.01", y1: "16", y2: "16" }
                                }
                                div {
                                    h4 { class: "font-medium text-sm", "导出提示" }
                                    p { class: "text-sm text-muted-foreground mt-1", "导出过程可能需要{(estimated_size / 512).max(1)}分钟左右，请保证硬盘有足够的空间。" }
                                }
                            }
                        }
                    }
                }

                // Right side cards
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
                                p { class: "text-xs text-slate-500 dark:text-slate-400 mb-2", "可用空间" }
                                p { class: "text-2xl font-bold", "256GB" }
                            }
                            div {
                                p { class: "text-xs text-slate-500 dark:text-slate-400 mb-2", "已用空间" }
                                p { class: "text-lg font-semibold", "2.5GB" }
                                p { class: "text-xs text-slate-500 dark:text-slate-400", "占硬盘 1%" }
                            }
                        }
                    }

                    // Export format
                    Card {
                        CardHeader {
                            CardTitle { "导出格式" }
                        }
                        CardContent {
                            class: "space-y-3",
                            div {
                                class: "space-y-2",
                                p { class: "text-sm font-medium", "选择导出格式" }
                                div {
                                    class: "space-y-2",
                                    label {
                                        class: "flex items-center gap-2 p-2 rounded border cursor-pointer hover:bg-slate-50 dark:hover:bg-slate-800",
                                        input {
                                            r#type: "radio",
                                            name: "format",
                                            checked: true,
                                            class: "w-4 h-4"
                                        }
                                        span { class: "text-sm", "MineBBS 格式" }
                                    }
                                    label {
                                        class: "flex items-center gap-2 p-2 rounded border cursor-pointer hover:bg-slate-50 dark:hover:bg-slate-800",
                                        input {
                                            r#type: "radio",
                                            name: "format",
                                            class: "w-4 h-4"
                                        }
                                        span { class: "text-sm", "Modrinth Modpack" }
                                    }
                                    label {
                                        class: "flex items-center gap-2 p-2 rounded border cursor-pointer hover:bg-slate-50 dark:hover:bg-slate-800",
                                        input {
                                            r#type: "radio",
                                            name: "format",
                                            class: "w-4 h-4"
                                        }
                                        span { class: "text-sm", "打包文件夹" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
