use fluffer::{async_trait, App, Client, Fluff, GemBytes};

#[derive(thiserror::Error, Debug)]
enum CustomErr {
    #[error("IO no worky :(")]
    FileNoWorky(#[from] std::io::Error),
}

#[async_trait]
impl GemBytes for CustomErr {
    async fn gem_bytes(self) -> Vec<u8> {
        // Decide which status is appropriate for each error
        let status = match self {
            _ => 40,
        };

        format!("{} {}\r\n", status, self).into_bytes()
    }
}

async fn file(_: Client) -> Result<Fluff, CustomErr> {
    let f = std::fs::read_to_string("./static/file.rs")?; // << gets converted into CustomErr

    Ok(Fluff::Document {
        mime: "text/rust".to_string(),
        body: f,
    })
}

#[tokio::main]
async fn main() {
    let app = App::default().route("/", file).run().await;

    // << Print app errors to stderr instead of panicking
    if let Err(e) = app {
        eprintln!("Error: {e:?}");
    }
}
