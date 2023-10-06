use fluffer::{App, Context};

async fn index(ctx: Context) -> String {
    if let Some(name) = ctx.subject_name() {
        format!("Hello, {name}\n=> /pem Print pem")
    } else {
        format!("Who are you? 🥴")
    }
}

async fn pem(ctx: Context) -> String {
    if let Some(pem) = ctx.pem() {
        format!("Hello, {}", pem)
    } else {
        format!("Who are you? 🥴")
    }
}

#[tokio::main]
async fn main() {
    App::default()
        .route("/", index)
        .route("/pem", pem)
        .run()
        .await
        .unwrap();
}
