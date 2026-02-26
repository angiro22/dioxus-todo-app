use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct ButtonProps {
    height: f32,
    width: f32,
    border_radius: f32, 
    content: String,
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
    rsx! {
        input { class: "button",
            
            r#type: "button",
            style: "
                height: {props.height}rem;
                width: {props.width}rem;
                border-radius: {props.border_radius}rem;
            ",
            value: "{props.content}",
        }
    }
}