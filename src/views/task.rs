use dioxus::prelude::*;

use crate::components::*;
use crate::Route;
#[component]
pub fn Task() -> Element {
    rsx! {
        document::Stylesheet { href: TASK_SCREEN_CSS }
    }
}