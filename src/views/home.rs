use dioxus::prelude::*;

use crate::components::*;
use crate::Route;

#[component]
pub fn Home() -> Element {
    rsx! {
        main { 
            h1 {
                id: "app-title",
                "To Do app" 
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

            div { class: "button-container",

                Link { to: Route::Task {},
                    
                    Button { id: "new-task",
                        height: 4.0,
                        width: 4.0,
                        border_radius: 1.0,
                        label: "+"
                    }
                }
            }
        }
    }
}