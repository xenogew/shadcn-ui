use dioxus::prelude::*;
use tailwind_fuse::*;

#[derive(TwClass)]
#[tw(
    class = "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg]:size-4 [&_svg]:shrink-0"
)]
pub struct ButtonClass {
    pub variant: ButtonVariant,
    pub size: ButtonSize,
}

#[derive(PartialEq, TwVariant)]
pub enum ButtonVariant {
    #[tw(
        default,
        class = "bg-primary text-primary-foreground hover:bg-primary/90")]
    Default,
    #[tw(class = "bg-destructive text-destructive-foreground hover:bg-destructive/90")]
    Destructive,
    #[tw(class = "border border-input bg-background hover:bg-accent hover:text-accent-foreground")]
    Outline,
    #[tw(class = "bg-secondary text-secondary-foreground hover:bg-secondary/80")]
    Secondary,
    #[tw(class = "hover:bg-accent hover:text-accent-foreground")]
    Ghost,
    #[tw(class = "text-primary underline-offset-4 hover:underline")]
    Link,
}

#[derive(PartialEq, TwVariant)]
pub enum ButtonSize {
    #[tw(default, class = "h-10 px-4 py-2")]
    Default,
    #[tw(class = "h-9 rounded-md px-3")]
    Sm,
    #[tw(class = "h-11 rounded-md px-8")]
    Lg,
    #[tw(class = "h-10 w-10")]
    Icon,
}

#[derive(PartialEq, Props, Clone)]
pub struct ButtonProps {
    #[props(default)]
    pub variant: ButtonVariant,
    #[props(default)]
    pub size: ButtonSize,

    // Global attributes
    #[props(default)]
    pub autofocus: bool,
    #[props(default)]
    pub class: Option<String>,
    #[props(default)]
    pub id: Option<String>,
    #[props(default)]
    pub style: String,

    // Attributes from `button`
    #[props(default)]
    pub command: Option<String>,
    #[props(default)]
    pub commandfor: Option<String>,
    #[props(default)]
    pub disabled: bool,
    #[props(default)]
    pub form: Option<String>,
    #[props(default)]
    pub formaction: Option<String>,
    #[props(default)]
    pub formenctype: Option<String>,
    #[props(default)]
    pub formmethod: Option<String>,
    #[props(default)]
    pub formnovalidate: bool,
    #[props(default)]
    pub formtarget: Option<String>,
    #[props(default)]
    pub name: Option<String>,
    #[props(default)]
    pub popovertarget: Option<String>,
    #[props(default)]
    pub popovertargetaction: Option<String>,
    #[props(default)]
    pub r#type: Option<String>,
    #[props(default)]
    pub value: Option<String>,

    // Event handler attributes
    #[props(default)]
    pub onclick: Option<EventHandler<MouseEvent>>,

    #[props(default)]
    pub as_child: bool,
    #[props(default)]
    pub children: Element,
}

#[component]
pub fn Button(props: &'static ButtonProps) -> Element {
    let class = ButtonClass {
        variant: props.variant,
        size: props.size,
    }
    .with_class(
        props.class.clone().unwrap_or_default(),
    );

    if props.as_child {
        return props.children.clone();
    }

    rsx! {
        button {
            autofocus: props.autofocus,
            class: "{class}",
            id: props.id.clone(),
            style: "{props.style}",
            disabled: props.disabled,
            r#type: props.r#type.clone(),
            onclick: move |evt| {
                if let Some(handler) = &props.onclick {
                    handler.call(evt);
                }
            },
            {props.children.clone()}
        }
    }
}
