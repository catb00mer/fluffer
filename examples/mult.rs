use fluffer::{async_trait, App, Client, Fluff, GemBytes};

// This file demonstates two valid ways to increase the number of possible return types in a route.

// 1. Return Vec<u8>, and call gem_bytes() on each type before returning it.
async fn vec(c: Client) -> Vec<u8> {
    if let Some(input) = c.input() {
        input.parse::<u32>().ok().gem_bytes().await
    } else {
        Fluff::Input("Input a number!".to_string())
            .gem_bytes()
            .await
    }
}

// 2. Define an enum of all the types you want to return, and implement GemBytes on it
enum Mult {
    OptNum(Option<u32>),
    Fluff(Fluff),
}

#[async_trait]
impl GemBytes for Mult {
    async fn gem_bytes(self) -> Vec<u8> {
        match self {
            Mult::OptNum(v) => v.gem_bytes().await,
            Mult::Fluff(v) => v.gem_bytes().await,
        }
    }
}

async fn mult(c: Client) -> Mult {
    if let Some(input) = c.input() {
        Mult::OptNum(input.parse::<u32>().ok())
    } else {
        Mult::Fluff(Fluff::Input("Input a number!".to_string()))
    }
}

#[tokio::main]
async fn main() {
    let app = App::default()
        .route("/", |_| async {
            "=>/mult Using enums\n=> /vec using Vec<u8>"
        })
        .route("/mult", mult)
        .route("/vec", vec)
        .run()
        .await;

    if let Err(e) = app {
        eprintln!("Error: {e:?}");
    }
}
