use dioxus::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::md_navigation_icons::MdArrowBack;

use crate::components::*;
use crate::Route;

static TASK_SCREEN_CSS: Asset = asset!("/assets/styling/task.css");

#[component]
pub fn Task() -> Element {
    rsx! {
        document::Stylesheet { href: TASK_SCREEN_CSS }

        main { id: "task-container",
            Link { to: Route::Home {  },
                button { class: "back-button",
                    Icon {
                        icon: MdArrowBack,
                        width: 30,
                        height: 30,
                        fill: "currentColor"
                    }
                }
            }
        }
    }
}