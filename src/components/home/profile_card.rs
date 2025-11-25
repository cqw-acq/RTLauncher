use dioxus::prelude::*;
use crate::components::ui::{Card, CardContent, Button, ButtonVariant, ButtonSize};

/// Account/Profile data structure
#[derive(Clone, PartialEq, Default)]
pub struct Account {
    pub name: String,
    pub status: String,
    pub avatar_url: Option<String>,
}

/// Profile card component displaying user profile and game launch options
#[component]
pub fn ProfileCard(
    selected_profile: Option<Account>,
    on_open_profile_selector: EventHandler<()>,
) -> Element {
    let profile = selected_profile.unwrap_or(Account {
        name: "RTL User".to_string(),
        status: "Ready to play".to_string(),
        avatar_url: None,
    });
    
    let first_char = profile.name.chars().next().unwrap_or('R').to_uppercase().to_string();
    
    rsx! {
        Card {
            class: "transition-all duration-700 ease-in-out h-full flex flex-col justify-between group relative overflow-hidden border hover:border-blue-500/50 hover:shadow-xl",
            // Glow effect background
            div {
                class: "absolute inset-0 opacity-0 group-hover:opacity-20 transition-opacity duration-300 pointer-events-none bg-gradient-to-r from-blue-500 to-cyan-400"
            }
            // Border glow effect
            div {
                class: "absolute inset-0 rounded-lg opacity-0 group-hover:opacity-100 transition-opacity duration-300 pointer-events-none",
                style: "box-shadow: inset 0 0 20px rgba(59, 130, 246, 0.3), inset 0 0 40px rgba(6, 182, 212, 0.2)"
            }
            // Main content area
            CardContent {
                class: "flex-grow flex flex-col items-center justify-center transition-all duration-700 ease-in-out relative z-10",
                Card {
                    class: "cursor-pointer transition-all duration-700 ease-in-out flex flex-row items-center p-3 w-full hover:bg-accent",
                    onclick: move |_| on_open_profile_selector.call(()),
                    // Avatar
                    div {
                        class: "w-12 h-12 rounded-full overflow-hidden",
                        div {
                            class: "bg-primary/10 border-2 border-dashed rounded-full w-full h-full flex items-center justify-center",
                            span {
                                class: "font-medium text-primary",
                                "{first_char}"
                            }
                        }
                    }
                    // User info
                    div {
                        class: "ml-3 flex flex-col",
                        span {
                            class: "font-bold text-base",
                            "{profile.name}"
                        }
                        span {
                            class: "text-muted-foreground text-xs",
                            "{profile.status}"
                        }
                    }
                }
            }
            // Bottom button area
            CardContent {
                class: "flex items-center space-x-4 relative z-10",
                div {
                    class: "w-full",
                    div {
                        class: "text-center text-sm text-muted-foreground mb-2",
                        "1.21.7 Fabric"
                    }
                    Button {
                        variant: ButtonVariant::Default,
                        size: ButtonSize::Lg,
                        class: "w-full mb-2",
                        "启动游戏"
                    }
                    div {
                        class: "flex space-x-2 mt-2",
                        Button {
                            variant: ButtonVariant::Secondary,
                            class: "flex-1",
                            "版本管理"
                        }
                    }
                }
            }
        }
    }
}
