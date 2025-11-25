use dioxus::prelude::*;

/// Button variant types
#[derive(Clone, Copy, PartialEq, Default)]
pub enum ButtonVariant {
    #[default]
    Default,
    Destructive,
    Outline,
    Secondary,
    Ghost,
    Link,
}

/// Button size types
#[derive(Clone, Copy, PartialEq, Default)]
pub enum ButtonSize {
    #[default]
    Default,
    Sm,
    Lg,
    Icon,
}

/// Button component with different variants and sizes
#[component]
pub fn Button(
    #[props(default)] variant: ButtonVariant,
    #[props(default)] size: ButtonSize,
    #[props(default)] class: String,
    #[props(default)] disabled: bool,
    onclick: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    let base_class = "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium transition-all outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2";
    
    let variant_class = match variant {
        ButtonVariant::Default => "bg-primary text-primary-foreground shadow hover:bg-primary/90",
        ButtonVariant::Destructive => "bg-destructive text-white shadow hover:bg-destructive/90",
        ButtonVariant::Outline => "border bg-background shadow hover:bg-accent hover:text-accent-foreground",
        ButtonVariant::Secondary => "bg-secondary text-secondary-foreground shadow hover:bg-secondary/80",
        ButtonVariant::Ghost => "hover:bg-accent hover:text-accent-foreground",
        ButtonVariant::Link => "text-primary underline-offset-4 hover:underline",
    };
    
    let size_class = match size {
        ButtonSize::Default => "h-9 px-4 py-2",
        ButtonSize::Sm => "h-8 rounded-md gap-1.5 px-3",
        ButtonSize::Lg => "h-10 rounded-md px-6",
        ButtonSize::Icon => "h-9 w-9",
    };
    
    let disabled_class = if disabled {
        "pointer-events-none opacity-50"
    } else {
        ""
    };
    
    let full_class = format!("{} {} {} {} {}", base_class, variant_class, size_class, disabled_class, class);
    
    rsx! {
        button {
            class: "{full_class}",
            disabled: disabled,
            onclick: move |evt| {
                if let Some(handler) = &onclick {
                    handler.call(evt);
                }
            },
            {children}
        }
    }
}
