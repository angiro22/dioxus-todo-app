use dioxus::prelude::*;

use crate::{CSS, models::Date};

#[derive(Props, Clone, PartialEq)]
pub struct TaskProps {
    title: String,
    description: Option<String>,
    //date: Date
}

#[component]
pub fn TaskResume(props: TaskProps) -> Element {
    rsx! {
        document::Stylesheet { href: CSS }

        div { 
            div {
                
            }

            h3 { 
                class: "task-title",
                "{props.title}"
            }

            if let Some(desc) = props.description {
                p { class: "task-description", "{desc}" }
            }
        }
    }
}