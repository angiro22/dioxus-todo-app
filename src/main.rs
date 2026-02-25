use dioxus::{prelude::*};

static CSS: Asset = asset!("/assets/styling/main.css"); // stylesheet declaration

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet { href: CSS } // implementing stylesheet

        main { 
            h1 { "ToDo app" }
        }
    }
}
