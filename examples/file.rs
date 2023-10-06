use fluffer::{App, Fluff};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    App::default()
        .route("/", |_| async {
            // isn't this meta? 😈
            Fluff::File("file.rs".to_string())
        })
        .run()
        .await
        .unwrap();
}
