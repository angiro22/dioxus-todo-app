use dioxus::{prelude::*};

mod components; // components model declaration
use components::TaskResume;
use components::Button;

mod models; // models model declaration
use models::Date;

static CSS: Asset = asset!("/assets/styling/main.css"); // stylesheet declaration

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: CSS } // implementing stylesheet

        main { 
            h1 {
                id: "app-title",
                "ToDo app" 
            }

            div { class: "task-container",
        
                h2 { 
                    id: "task-counter",
                    "Hai 0 attività"
                }

                TaskResume {
                    title: "Example task 1",
                    description: "lorem ipsum 1",
                    done: false
                }
                
                div { class: "break-line" }

                TaskResume {
                    title: "Example task 2",
                    description: "lorem ipsum 2",
                    done: true
                }
            }

            div { class: "button-container", id: "new-task",

                Button {
                    height: 4.0,
                    width: 4.0,
                    border_radius: 1.0,
                    content: "+",
                }
            }
        }
    }
}
