// Scenario: you wrote a route that contacts some API, or displays some information that you want
// to embed in a greater document.

// The approach is to simply call the desired route function directly; attaching `Client` either by
// cloning or borrowing it.

use fluffer::{App, Client, Fluff};

async fn api(_: Client) -> String {
    format!("* ~ Some API information here!!!!!~")
}

async fn index(c: Client) -> Fluff {
    Fluff::Text(format!(
        "# Some info

## Api Stats
{}

## Footer",
        api(c.clone()).await
    ))
}

#[tokio::main]
async fn main() {
    App::default()
        .route("/", index)
        .route("/api", api) // <- it's not neccessary to make the /api route public, since we call the function directly
        .run()
        .await
        .unwrap()
}
