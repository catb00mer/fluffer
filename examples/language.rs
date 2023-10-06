use fluffer::{App, Fluff};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    App::default()
        .route("/", |_| async {
            Fluff::Lang {
                lang: "en,fr".to_string(),
                body: "Greetings! Comment t'allez vous? :D".to_string(),
            }
        })
        .run()
        .await
        .unwrap();
}
