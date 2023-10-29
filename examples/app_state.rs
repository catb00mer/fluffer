use fluffer::App;
use std::sync::{Arc, Mutex};

// Type alias for Client<State> **highly recommended**
type Client = fluffer::Client<Arc<Mutex<State>>>;

#[derive(Default)]
struct State {
    visitors: u32,
}

async fn index(c: Client) -> String {
    let mut state = c.state.lock().unwrap();
    state.visitors += 1;

    format!("Visitors: {}", state.visitors)
}

#[tokio::main]
async fn main() {
    let state = Arc::new(Mutex::new(State::default()));

    App::default()
        .state(state) // <- Must be called first.
        .route("/", index)
        .run()
        .await
        .unwrap()
}
