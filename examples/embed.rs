use fluffer::{App, AsGemtext, Client, Fluff, GemBytes};

async fn api(_: Client) -> Fluff {
    if rand::random::<bool>() {
        Fluff::Text("* ~ Some API information here!!!!!~".to_string())
    } else {
        Fluff::NotFound("This route is broked :(".to_string())
    }
}

async fn index(c: Client) -> Fluff {
    Fluff::Text(format!(
        r#"The stats below will fail to load 1/2 of the time. Keep refreshing.

## Routes within routes
Here we call the api route, call gem_bytes() on it, and call as_gemtext() on the bytes.

This allows us to embed the api route in this document.

=> /api api

{}"#,
        api(c).await.gem_bytes().await.as_gemtext()
    ))
}

#[tokio::main]
async fn main() {
    App::default()
        .route("/", index)
        .route("/api", api)
        .run()
        .await
        .unwrap()
}
