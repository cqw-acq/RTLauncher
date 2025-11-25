use dioxus::prelude::*;

/// Card component - a container with rounded corners and shadow
#[component]
pub fn Card(
    #[props(default)] class: String,
    onclick: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    let base_class = "bg-card text-card-foreground flex flex-col gap-6 rounded-xl border py-6 shadow-sm";
    let full_class = format!("{} {}", base_class, class);
    
    rsx! {
        div {
            class: "{full_class}",
            onclick: move |evt| {
                if let Some(handler) = &onclick {
                    handler.call(evt);
                }
            },
            {children}
        }
    }
}

/// Card header component
#[component]
pub fn CardHeader(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let base_class = "grid auto-rows-min grid-rows-[auto_auto] items-start gap-1.5 px-6";
    let full_class = format!("{} {}", base_class, class);
    
    rsx! {
        div {
            class: "{full_class}",
            {children}
        }
    }
}

/// Card title component
#[component]
pub fn CardTitle(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let base_class = "leading-none font-semibold";
    let full_class = format!("{} {}", base_class, class);
    
    rsx! {
        div {
            class: "{full_class}",
            {children}
        }
    }
}

/// Card description component
#[component]
pub fn CardDescription(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let base_class = "text-muted-foreground text-sm";
    let full_class = format!("{} {}", base_class, class);
    
    rsx! {
        div {
            class: "{full_class}",
            {children}
        }
    }
}

/// Card content component
#[component]
pub fn CardContent(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let base_class = "px-6";
    let full_class = format!("{} {}", base_class, class);
    
    rsx! {
        div {
            class: "{full_class}",
            {children}
        }
    }
}

/// Card footer component
#[component]
pub fn CardFooter(
    #[props(default)] class: String,
    children: Element,
) -> Element {
    let base_class = "flex items-center px-6";
    let full_class = format!("{} {}", base_class, class);
    
    rsx! {
        div {
            class: "{full_class}",
            {children}
        }
    }
}
