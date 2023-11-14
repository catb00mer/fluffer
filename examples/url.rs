use fluffer::{App, Client, Fluff};

async fn echo(c: Client) -> Fluff {
    if let Some(input) = c.input() {
        Fluff::Text(format!("The cave echoes...\n\n{input}"))
    } else {
        Fluff::Input("Enter some input".to_string())
    }
}

async fn page(c: Client) -> String {
    let entries = vec![
        "entry 1", "entry 2", "entry 3", "entry 4", "entry 5", "entry 6", "entry 7", "entry 8",
        "entry 9", "entry 10", "entry 11", "entry 12", "entry 13", "entry 14", "entry 15",
        "entry 16", "entry 17", "entry 18", "entry 19", "entry 20", "entry 21", "entry 22",
        "entry 23", "entry 24", "entry 25", "entry 26", "entry 27", "entry 28", "entry 29",
    ];

    let num: usize = c.parameter("p").parse::<usize>().unwrap_or(0);

    let entries_per_page: usize = 5;
    let page: Vec<&str> = entries
        .into_iter()
        .enumerate()
        .filter_map(|(i, x)| {
            if i >= entries_per_page * num && i < entries_per_page * num + entries_per_page {
                Some(x)
            } else {
                None
            }
        })
        .collect();

    format!(
        "```\n{page:#?}\n```\n\n=> /page/{} Next page\n=> /page/{} Prev page\n\n{}",
        num + 1,
        num.checked_sub(1).unwrap_or(0),
        if let Some(input) = c.input() {
            format!("I got user input too!\n```\n{input}\n```\n")
        } else {
            "".to_string()
        }
    )
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    App::default()
        .route("/", |_| async { "=> /echo Echo input\n=> /page/0 Page" })
        .route("/echo", echo)
        .route("/page/:p", page)
        .run()
        .await
        .unwrap();
}
