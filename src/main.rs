use dioxus::{prelude::*};

mod components; // components module
pub use components::*;

mod views; // views module 
pub use views::*;

mod db; // database module

static CSS: Asset = asset!("/assets/styling/main.css"); // stylesheet declaration

#[derive(Routable, Clone)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Home {},

    #[route("/task")]
    Task {},

    #[route("/:.._segments")]
    PageNotFound { _segments: Vec<String> }
}

fn main() {
    dioxus::launch(App);
}

#[component]
pub fn App() -> Element {
    // Init db resource
    let db_resource = use_resource(move || async move {
        db::init_db().await
    });

    let pool = match &*db_resource.read() {
        Some(pool) => pool.clone(),
        None => return rsx! { div { "Database loading..." } },
    };

    use_context_provider(|| pool.clone()); // provides the pool to the whole app

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