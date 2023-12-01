use axum::{response::Html, routing::get, Router, extract::Path};
use minijinja::render;
use serde::{Serialize, Deserialize};

async fn home() -> Html<&'static str> {
    Html("hello world!")
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
struct Profile {
    name: String,
    items: Vec<Item>
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
struct Item {
    name: String,
}

async fn get_profile(Path(profile_name): Path<String>) -> Html<String> {
    let orders_example = vec![
        Item {
            name: "item1".into()
        },
        Item {
            name: "item2".into()
        }
    ];
    let profile_example = Profile {
        name: profile_name,
        items: orders_example,
    };
    Html(render!(include_str!("../assets/templates/index.html"), profile => profile_example))
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(home))
        .route("/:profile_name", get(get_profile));

    Ok(router.into())
}
