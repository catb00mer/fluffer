use fluffer::{App, Client};

// Here we don't have to unwrap the reqwest.
async fn cat(_: Client) -> reqwest::Result<reqwest::Response> {
    reqwest::get("https://cataas.com/cat").await
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    App::default()
        .route("/", |_| async { "=> /cat 😺 c a t" })
        .route("/cat", cat)
        .run()
        .await
        .unwrap();
}
