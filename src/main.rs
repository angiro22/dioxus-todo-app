use dioxus::{prelude::*};

mod components; // components module declaration
pub use components::*;

mod views; // views module declaration
pub use views::*;

static CSS: Asset = asset!("/assets/styling/main.css"); // stylesheet declaration

#[derive(Routable, Clone)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Task {},

    #[route("/task")]
    Home {},

    #[route("/:.._segments")]
    PageNotFound { _segments: Vec<String> }
}

fn main() {
    dioxus::launch(App);
}

#[component]
pub fn App() -> Element {
    rsx! {
        document::Stylesheet { href: CSS } // implementing stylesheet

        Router::<Route> {}
    }
}

#[component]
fn PageNotFound(_segments: Vec<String>) -> Element {
    navigator().push(Route::Home {});
    
    rsx! {}
}