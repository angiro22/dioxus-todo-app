use dioxus::html::div;
use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::md_navigation_icons::*;
use dioxus_free_icons::icons::md_editor_icons::*;
use dioxus_free_icons::icons::md_action_icons::*;
use dioxus_free_icons::icons::md_device_icons::*;

use crate::components::*;
use crate::Route;

static TASK_SCREEN_CSS: Asset = asset!("/assets/styling/task.css");

#[component]
pub fn Task() -> Element {
    let mut description = use_signal(|| String::new());
    
    let on_input = move |evt: FormEvent| {
        description.set(evt.value());
    };

    rsx! {
        document::Stylesheet { href: TASK_SCREEN_CSS }

        main { class: "container",
            Link { to: Route::Home {  },
                button { class: "back-button",
                    Icon {
                        icon: MdArrowBack,
                        width: 30,
                        height: 30,
                        fill: "white"
                    }
                }
            }

            div { class: "info-container",
                h1 { id: "title",
                    "Attività"
                }

                div { class: "content",
                    div { class: "input",
                        Icon {
                            icon: MdNotes,
                            width: 30,
                            height: 30,
                            fill: "white"
                        }

                        textarea {  
                            value: "{description}",
                            oninput: on_input,

                            style: "
                                overflow-y: hidden; 
                                resize: none; 
                                min-height: 50px;
                                height: auto;
                            ",

                            rows: "{description.read().lines().count().max(1)}",
                            cols: "29",

                            placeholder: "Descrizione..."
                        }
                    }

                    div { class: "input",
                        Icon {
                            icon: MdAccessTime,
                            width: 30,
                            height: 30,
                            fill: "white"
                        }

                        input {
                            r#type: "date",
                        }
                    }

                    div { class: "button-container",
                        Button { id: "mark-completed",
                            height: 3.0,
                            width: 15.7,
                            border_radius: 25.0,
                            label: "Segna come completata"
                        }
                    }
                }
            }
        }
    }
}