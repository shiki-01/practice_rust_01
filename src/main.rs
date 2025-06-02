use dioxus::{logger::tracing::info, prelude::*};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");

fn main() {
    #[cfg(feature = "server")]
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(launch_server());
    #[cfg(not(feature = "server"))]
    dioxus::launch(App);
}

#[cfg(feature = "server")]
async fn launch_server() {
    dioxus::logger::initialize_default();

    let socket_addr = dioxus_cli_config::fullstack_adders_or_localhost();

    let router = axum::Router::new()
        .serve_dixous_application(ServerConfigBuilder::new(), App)
        .into_make_service();

    let listener = tokio::net::TcpListener::bind(soket_addr).await.unwrap();
    axum::serve(listener, router).await.unwrap();

}

#[derive(Clone)]
struct TitleState(String);

#[derive(serde::Deserialize)]
struct DogApi {
    message: String,
}

#[component]
fn App() -> Element {
    use_context_provider(|| TitleState("HotDog! 🌭".to_string()));

    rsx! {
        document::Stylesheet { href: MAIN_CSS }
        Title {}
        DogView {}
    }
}

#[component]
fn Title() -> Element {
    let title = use_context::<TitleState>();

    rsx!(
        div {
            id: "title",
            h1 { "{title.0}" }
        }
    )
}

#[component]
fn DogView() -> Element {
    let mut img_src = use_resource(|| async move {
        reqwest::get("https://dog.ceo/api/breeds/image/random")
            .await
            .unwrap()
            .json::<DogApi>()
            .await
            .unwrap()
            .message
    });

    rsx!(
        div {
            id: "dogview",
            img {
                src: "{img_src.cloned().unwrap_or_default()}",
            }
        }
        div {
            id: "buttons",
            button { onclick: move |_| img_src.restart(), id: "skip", "skip" }
            button { onclick: move |_| img_src.restart(), id: "save", "save" }
        }
    )
}

#[server]
async fn save_dog(image: String) -> Result<(), ServerFnError> {
    Ok(())
}
