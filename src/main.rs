mod components;
mod backend;

use crate::components::*;
use dioxus::prelude::*;

const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    dioxus::launch(app);
}

#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[layout(NavBar)]
    #[route("/")]
    DogView,

    #[route("/favorites")]
    Favorites,
}

fn app() -> Element {
    rsx! {
        document::Stylesheet { href: MAIN_CSS }
        
        Router::<Route> {}
    }
}
