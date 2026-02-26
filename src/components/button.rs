use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ButtonProps {
    id: Option<String>,
    height: f32,
    width: f32,
    border_radius: f32, 
    label: String,
    on_click: Option<EventHandler<MouseEvent>>
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
    rsx! {
        button { class: "button", id: props.id.clone(),
            onclick: move |event| {
                if let Some(handler) = &props.on_click { handler.call(event) }
            },
            
            style: "
                height: {props.height}rem;
                width: {props.width}rem;
                border-radius: {props.border_radius}rem;
            ",
            "{props.label}"
        }
    }
}