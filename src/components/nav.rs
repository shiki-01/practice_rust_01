use crate::Route;
use dioxus::prelude::*;

#[component]
pub fn NavBar() -> Element {
    rsx! {
        div {
            id: "title",
            Link {
                to: Route::DogView,
                h1 { "HotDog! 🌭" }
            }
            Link {
                id: "heart",
                to: Route::Favorites,
                h1 { "Like 💛" }
            }
        }
        Outlet::<Route> {}
    }
}