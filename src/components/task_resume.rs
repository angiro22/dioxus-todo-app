use dioxus::prelude::*;

use crate::{CSS, models::Date};

#[derive(Props, Clone, PartialEq)]
pub struct TaskProps {
    title: String,
    description: Option<String>,
    //date: Date
    done: bool
}

#[component]
pub fn TaskResume(props: TaskProps) -> Element {
    let mut is_done = use_signal(|| props.done);
    let dynamic_class = if is_done() { "completed" } else { "" };

    rsx! {
        document::Stylesheet { href: CSS }
        
        div { class: "task-card",

            input {
                class: "checkbox",
                type: "checkbox",
                checked: is_done,

                oninput: move |_| { is_done.set(!is_done()) }
            }

            div {
                class: "task-info {dynamic_class}",

                h3 { class: "task-title", 
                    "{props.title}" 
                }

                        if let Some(desc) = props.description {
                p { class: "task-description {dynamic_class}", "{desc}" }
                }
            }
        }
    }
}