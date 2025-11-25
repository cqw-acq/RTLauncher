use dioxus::prelude::*;
use crate::Route;
use crate::components::ui::{Card, CardHeader, CardTitle, CardContent};

/// Instance settings layout with sidebar navigation
#[component]
pub fn InstanceSettingsLayout() -> Element {
    rsx! {
        div {
            class: "w-full h-screen flex flex-col ml-16",
            div {
                class: "space-y-4 flex-1 overflow-hidden flex flex-col min-h-0 mx-2 p-2",
                // Instance header
                div {
                    class: "sticky top-0 z-10 bg-background",
                    Card {
                        class: "p-3",
                        div {
                            class: "flex items-center flex-wrap gap-3",
                            // Instance icon
                            div {
                                class: "w-12 h-12 rounded-lg overflow-hidden border-2 border-dashed flex items-center justify-center shrink-0 bg-primary/10",
                                span {
                                    class: "font-bold text-primary",
                                    "R"
                                }
                            }
                            // Instance info
                            div {
                                class: "flex-1 min-w-0 flex flex-col sm:flex-row sm:justify-between sm:items-center gap-2",
                                div {
                                    class: "flex items-center flex-wrap gap-1",
                                    h2 {
                                        class: "text-lg font-bold mr-2",
                                        "RTL World"
                                    }
                                    span {
                                        class: "px-2 py-1 text-xs rounded-full bg-primary text-primary-foreground",
                                        "Minecraft 1.21.8"
                                    }
                                    span {
                                        class: "px-2 py-1 text-xs rounded-full bg-secondary text-secondary-foreground",
                                        "Fabric 0.17.2"
                                    }
                                    span {
                                        class: "px-2 py-1 text-xs rounded-full bg-blue-500 text-white",
                                        "72 Mods"
                                    }
                                }
                                div {
                                    class: "flex items-center gap-2",
                                    Link {
                                        to: Route::Home {},
                                        class: "px-3 py-1.5 text-sm rounded-md border hover:bg-accent",
                                        "返回主页"
                                    }
                                    button {
                                        class: "px-3 py-1.5 text-sm rounded-md bg-primary text-primary-foreground hover:bg-primary/90",
                                        "启动游戏"
                                    }
                                }
                            }
                        }
                    }
                }

                // Main content grid
                div {
                    class: "grid grid-cols-12 gap-3 flex-1 min-h-0",
                    // Sidebar navigation
                    aside {
                        class: "col-span-2",
                        Card {
                            CardHeader {
                                CardTitle { "实例设置" }
                            }
                            CardContent {
                                nav {
                                    class: "flex flex-col space-y-2",
                                    // Basic settings
                                    Link {
                                        to: Route::InstanceBasic {},
                                        class: "flex items-center px-3 py-2 rounded-md text-sm transition-colors hover:bg-accent",
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
                                            class: "mr-2",
                                            path { d: "M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z" }
                                            circle { cx: "12", cy: "12", r: "3" }
                                        }
                                        "基础"
                                    }
                                    // Advanced settings
                                    Link {
                                        to: Route::InstanceModify {},
                                        class: "flex items-center px-3 py-2 rounded-md text-sm transition-colors hover:bg-accent",
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
                                            class: "mr-2",
                                            path { d: "M21.174 6.812a1 1 0 0 0-3.986-3.987L3.842 16.174a2 2 0 0 0-.5.83l-1.321 4.352a.5.5 0 0 0 .623.622l4.353-1.32a2 2 0 0 0 .83-.497z" }
                                            path { d: "m15 5 4 4" }
                                        }
                                        "高级"
                                    }
                                    // Export
                                    Link {
                                        to: Route::InstanceExport {},
                                        class: "flex items-center px-3 py-2 rounded-md text-sm transition-colors hover:bg-accent",
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
                                            class: "mr-2",
                                            circle { cx: "18", cy: "5", r: "3" }
                                            circle { cx: "6", cy: "12", r: "3" }
                                            circle { cx: "18", cy: "19", r: "3" }
                                            line { x1: "8.59", x2: "15.42", y1: "13.51", y2: "17.49" }
                                            line { x1: "15.41", x2: "8.59", y1: "6.51", y2: "10.49" }
                                        }
                                        "导出"
                                    }
                                    // Mods
                                    Link {
                                        to: Route::InstanceMods {},
                                        class: "flex items-center px-3 py-2 rounded-md text-sm transition-colors hover:bg-accent",
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
                                            class: "mr-2",
                                            path { d: "m7.5 4.27 9 5.15" }
                                            path { d: "M21 8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16Z" }
                                            path { d: "m3.3 7 8.7 5 8.7-5" }
                                            path { d: "M12 22V12" }
                                        }
                                        "Mods"
                                    }
                                    // Resources
                                    Link {
                                        to: Route::InstanceResources {},
                                        class: "flex items-center px-3 py-2 rounded-md text-sm transition-colors hover:bg-accent",
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
                                            class: "mr-2",
                                            circle { cx: "13.5", cy: "6.5", r: ".5", fill: "currentColor" }
                                            circle { cx: "17.5", cy: "10.5", r: ".5", fill: "currentColor" }
                                            circle { cx: "8.5", cy: "7.5", r: ".5", fill: "currentColor" }
                                            circle { cx: "6.5", cy: "12.5", r: ".5", fill: "currentColor" }
                                            path { d: "M12 2C6.5 2 2 6.5 2 12s4.5 10 10 10c.926 0 1.648-.746 1.648-1.688 0-.437-.18-.835-.437-1.125-.29-.289-.438-.652-.438-1.125a1.64 1.64 0 0 1 1.668-1.668h1.996c3.051 0 5.555-2.503 5.555-5.555C21.965 6.012 17.461 2 12 2z" }
                                        }
                                        "资源"
                                    }
                                    // Shaders
                                    Link {
                                        to: Route::InstanceShaders {},
                                        class: "flex items-center px-3 py-2 rounded-md text-sm transition-colors hover:bg-accent",
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
                                            class: "mr-2",
                                            path { d: "m12.83 2.18a2 2 0 0 0-1.66 0L2.6 6.08a1 1 0 0 0 0 1.83l8.58 3.91a2 2 0 0 0 1.66 0l8.58-3.9a1 1 0 0 0 0-1.83Z" }
                                            path { d: "m22 17.65-9.17 4.16a2 2 0 0 1-1.66 0L2 17.65" }
                                            path { d: "m22 12.65-9.17 4.16a2 2 0 0 1-1.66 0L2 12.65" }
                                        }
                                        "光影"
                                    }
                                    // Worlds
                                    Link {
                                        to: Route::InstanceWorlds {},
                                        class: "flex items-center px-3 py-2 rounded-md text-sm transition-colors hover:bg-accent",
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
                                            class: "mr-2",
                                            circle { cx: "12", cy: "12", r: "10" }
                                            path { d: "M12 2a14.5 14.5 0 0 0 0 20 14.5 14.5 0 0 0 0-20" }
                                            path { d: "M2 12h20" }
                                        }
                                        "世界"
                                    }
                                    // Screenshots
                                    Link {
                                        to: Route::InstanceScreenshots {},
                                        class: "flex items-center px-3 py-2 rounded-md text-sm transition-colors hover:bg-accent",
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
                                            class: "mr-2",
                                            rect { width: "18", height: "18", x: "3", y: "3", rx: "2", ry: "2" }
                                            circle { cx: "9", cy: "9", r: "2" }
                                            path { d: "m21 15-3.086-3.086a2 2 0 0 0-2.828 0L6 21" }
                                        }
                                        "截图"
                                    }
                                }
                            }
                        }
                    }

                    // Main content area
                    main {
                        class: "col-span-10 overflow-y-auto min-h-0 no-scrollbar",
                        div {
                            class: "space-y-4",
                            Outlet::<Route> {}
                        }
                    }
                }
            }
        }
    }
}
