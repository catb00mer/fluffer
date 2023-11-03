// Scenario: you have a type that implements GemBytes. It formats some displays some information that you want
// to embed in a greater document.

// 1. Call the desired route function.
//
// 2. Use the `ToGemtext` trait (implemented for `GemBytes`) to get the gemtext returned by the
//    type (if any).

use fluffer::{App, Fluff, ToGemtext};

type Client = fluffer::Client<bool>; // ignore this :>

async fn api(c: Client) -> Fluff {
    // randomly returns one of these
    if c.state {
        Fluff::Text("* ~ Some API information here!!!!!~".to_string())
    } else {
        Fluff::NotFound("This route is broked :(".to_string())
    }
}

async fn index(mut c: Client) -> Fluff {
    c.state = rand::random::<bool>();
    Fluff::Text(format!(
        r#"# Welcome to this demo
The stats below will fail to load 1/2 of the time. Keep refreshing.

### -> to_gemtext_err()
{}
### -> to_gemtext().unwrap_or("api broken".to_string())
{}"#,
        api(c.clone()).await.to_gemtext_err().await,
        api(c.clone())
            .await
            .to_gemtext()
            .await
            .unwrap_or("api broken".to_string()),
    ))
}

#[tokio::main]
async fn main() {
    App::default()
        .state(true)
        .route("/", index)
        .route("/api", api) // <- it's not neccessary to make the /api route public, since we call the function directly
        .run()
        .await
        .unwrap()
}
