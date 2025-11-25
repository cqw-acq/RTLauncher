use crate::Route;
use crate::components::ui::{Card, CardContent};
use dioxus::prelude::*;

/// Sidebar navigation component
#[component]
pub fn Sidebar() -> Element {
    let mut is_dark = use_signal(|| false);
    
    rsx! {
        Card {
            class: "fixed left-0 top-0 h-full w-16 rounded-none border-r flex flex-col justify-between p-2 z-10",
            CardContent {
                class: "p-0 flex flex-col items-center gap-4",
                // Home button
                Link {
                    to: Route::Home {},
                    class: "p-2 rounded-lg hover:bg-accent transition-colors",
                    title: "主页",
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
                        path { d: "m3 9 9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z" }
                        polyline { points: "9 22 9 12 15 12 15 22" }
                    }
                }
                // Download button - now a Link
                Link {
                    to: Route::Download {},
                    class: "p-2 rounded-lg hover:bg-accent transition-colors",
                    title: "下载",
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
                        path { d: "M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" }
                        polyline { points: "7 10 12 15 17 10" }
                        line { x1: "12", x2: "12", y1: "15", y2: "3" }
                    }
                }
                // Network button - now a Link
                Link {
                    to: Route::Lan {},
                    class: "p-2 rounded-lg hover:bg-accent transition-colors",
                    title: "网络",
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
                        path { d: "M12 20h.01" }
                        path { d: "M2 8.82a15 15 0 0 1 20 0" }
                        path { d: "M5 12.859a10 10 0 0 1 14 0" }
                        path { d: "M8.5 16.429a5 5 0 0 1 7 0" }
                    }
                }
                // More button - now a Link
                Link {
                    to: Route::More {},
                    class: "p-2 rounded-lg hover:bg-accent transition-colors",
                    title: "更多",
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
                        circle { cx: "12", cy: "12", r: "1" }
                        circle { cx: "19", cy: "12", r: "1" }
                        circle { cx: "5", cy: "12", r: "1" }
                    }
                }
            }
            CardContent {
                class: "p-0 flex flex-col items-center gap-4",
                // Theme toggle button
                button {
                    class: "p-2 rounded-lg hover:bg-accent transition-colors",
                    title: "切换主题",
                    onclick: move |_| is_dark.set(!is_dark()),
                    if is_dark() {
                        // Moon icon
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
                            path { d: "M12 3a6 6 0 0 0 9 9 9 9 0 1 1-9-9Z" }
                        }
                    } else {
                        // Sun icon
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
                            circle { cx: "12", cy: "12", r: "4" }
                            path { d: "M12 2v2" }
                            path { d: "M12 20v2" }
                            path { d: "m4.93 4.93 1.41 1.41" }
                            path { d: "m17.66 17.66 1.41 1.41" }
                            path { d: "M2 12h2" }
                            path { d: "M20 12h2" }
                            path { d: "m6.34 17.66-1.41 1.41" }
                            path { d: "m19.07 4.93-1.41 1.41" }
                        }
                    }
                }
                // Settings button - now a Link
                Link {
                    to: Route::Settings {},
                    class: "p-2 rounded-lg hover:bg-accent transition-colors",
                    title: "设置",
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
                        path { d: "M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z" }
                        circle { cx: "12", cy: "12", r: "3" }
                    }
                }
            }
        }
    }
}
