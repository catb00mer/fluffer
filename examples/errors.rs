use fluffer::{async_trait, App, Client, GemBytes};

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

async fn custom(_: Client) -> Result<String, CustomErr> {
    let f = std::fs::read_to_string("badfile")?; // << gets converted into CustomErr
    Ok(f)
}

async fn anyhow(_: Client) -> anyhow::Result<String> {
    let f = std::fs::read_to_string("badfile")?;
    Ok(f)
}

#[tokio::main]
async fn main() {
    let app = App::default()
        .route("/", |_| async {
            r#"> We're trying to read a file that doesn't exist.

Route 1 uses a custom error type we created by deriving `thiserror`, and implementing `GemBytes` on it.

Route 2 uses `anyhow::Result`, which simply displays the error message as a temporary failure.

=> /custom Custom
=> /anyhow Anyhow"#
        })
        .route("/custom", custom)
        .route("/anyhow", anyhow)
        .run()
        .await;

    // << Print app errors to stderr instead of panicking
    if let Err(e) = app {
        eprintln!("Error: {e}");
    }
}
