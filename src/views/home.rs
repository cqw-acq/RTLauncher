use dioxus::prelude::*;
use crate::components::home::{AnnouncementCard, ProfileCard, Account};

/// The Home page component that will be rendered when the current route is `[Route::Home]`
#[component]
pub fn Home() -> Element {
    let mut is_profile_selector_open = use_signal(|| false);
    let selected_profile = use_signal(|| None::<Account>);
    
    rsx! {
        div {
            class: "relative h-screen ml-16",
            // Main home content
            div {
                class: "absolute inset-0 overflow-hidden",
                // Left side - Announcement card
                div {
                    class: "absolute left-0 top-0 w-full md:w-1/3 lg:w-1/4 h-auto p-4",
                    AnnouncementCard {}
                }
                // Right side - Profile card
                div {
                    class: "absolute right-0 top-0 w-full md:w-1/3 lg:w-1/4 h-full p-4 flex flex-col justify-end",
                    ProfileCard {
                        selected_profile: selected_profile(),
                        on_open_profile_selector: move |_| is_profile_selector_open.set(true),
                    }
                }
            }
        }
    }
}
