use fluffer::{App, Client, Fluff};

async fn api(_: Client) -> Fluff {
    if rand::random::<bool>() {
        Fluff::Text("* ~ Some API information here!!!!!~".to_string())
    } else {
        Fluff::NotFound("Pretend api failed".to_string())
    }
}

async fn index(c: Client) -> String {
    format!(
        r#"The stats below will fail to load 1/2 of the time. Keep refreshing.

## Routes within routes
You can use the `Client::render` function to render a function route as gemtext, such that it can be embedded into another gemtext document.

In this example, the `/api` route uses the same logic as the text beneath the header below.

The usefulness of this is honestly up in the air. I just think it's really cool 💀

=> /api Api
### c.render(api).await
{}"#,
        c.render(api).await
    )
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
