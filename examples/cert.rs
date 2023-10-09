use fluffer::{App, Context};

async fn subject(ctx: Context) -> String {
    ctx.subject_name().unwrap_or("Who are you? 🥴".to_string())
}

async fn pem(ctx: Context) -> String {
    if let Some(pem) = ctx.pem() {
        format!("{}", pem)
    } else {
        format!("Who are you? 🥴")
    }
}

#[tokio::main]
async fn main() {
    App::default()
        .route("/", |_| async {
            "=> /pem ctx.pem()\n=> /subject ctx.subject_name()\n"
        })
        .route("/subject", subject)
        .route("/pem", pem)
        .run()
        .await
        .unwrap();
}
