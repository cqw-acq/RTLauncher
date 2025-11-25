use dioxus::prelude::*;
use crate::components::ui::{Card, CardHeader, CardTitle, CardDescription, CardContent};

/// Announcement data structure
#[derive(Clone, PartialEq)]
pub struct Announcement {
    pub id: i32,
    pub title: String,
    pub content: String,
}

/// Sample announcements
fn get_announcements() -> Vec<Announcement> {
    vec![
        Announcement {
            id: 1,
            title: "RTLauncher v0.1.0".to_string(),
            content: "Welcome to RTLauncher! The first version is now available.".to_string(),
        },
        Announcement {
            id: 2,
            title: "New Features".to_string(),
            content: "We're working on adding more features. Stay tuned!".to_string(),
        },
        Announcement {
            id: 3,
            title: "Community".to_string(),
            content: "Join our community to get the latest updates and support.".to_string(),
        },
    ]
}

/// Announcement card component displaying announcements in a carousel-like view
#[component]
pub fn AnnouncementCard() -> Element {
    let mut current_index = use_signal(|| 0);
    let announcements = get_announcements();
    let total = announcements.len();
    
    rsx! {
        Card {
            CardHeader {
                CardTitle { "公告栏" }
                CardDescription { "最新消息和更新" }
            }
            CardContent {
                class: "flex flex-col items-center justify-center",
                div {
                    class: "w-full max-w-xs",
                    div {
                        class: "p-4 border rounded-lg transition-all duration-300",
                        if let Some(announcement) = announcements.get(current_index()) {
                            h3 {
                                class: "font-semibold",
                                "{announcement.title}"
                            }
                            p {
                                class: "text-sm text-muted-foreground mt-2",
                                "{announcement.content}"
                            }
                        }
                    }
                    // Navigation dots
                    div {
                        class: "flex justify-center gap-2 mt-4",
                        for i in 0..total {
                            button {
                                class: if i == current_index() {
                                    "w-2 h-2 rounded-full bg-primary"
                                } else {
                                    "w-2 h-2 rounded-full bg-muted"
                                },
                                onclick: move |_| current_index.set(i),
                            }
                        }
                    }
                    // Navigation buttons
                    div {
                        class: "flex justify-between mt-4",
                        button {
                            class: "p-2 rounded-lg hover:bg-accent transition-colors",
                            onclick: move |_| {
                                if current_index() > 0 {
                                    current_index.set(current_index() - 1);
                                } else {
                                    current_index.set(total - 1);
                                }
                            },
                            "←"
                        }
                        button {
                            class: "p-2 rounded-lg hover:bg-accent transition-colors",
                            onclick: move |_| {
                                if current_index() < total - 1 {
                                    current_index.set(current_index() + 1);
                                } else {
                                    current_index.set(0);
                                }
                            },
                            "→"
                        }
                    }
                }
            }
        }
    }
}
