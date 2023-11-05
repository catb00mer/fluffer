use fluffer::{App, Static};

#[tokio::main]
async fn main() {
    let welcome = "# Welcome! {}";

    App::default()
        .route("/", Static(welcome))
        .run()
        .await
        .unwrap()
}
