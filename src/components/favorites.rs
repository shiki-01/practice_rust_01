use dioxus::prelude::*;

#[component]
pub fn Favorites() -> Element {
    let favorites = use_resource(crate::backend::list_dogs).suspend()?;

    rsx! {
        div { id: "favorites",
            div { id: "favorites-container",
                for (_, url) in favorites().unwrap() {
                    div {
                        key: _,
                        class: "favorite-dog",
                        img { src: "{url}" }
                    }
                }
            }
        }
    }
}
