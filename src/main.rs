use clap::Parser;

#[derive(Parser)]
struct Cli {
    /// ex. http://example.com
    url: String,

    #[arg(short = 'X', long = "request", value_name = "method")]
    method: Option<String>,
}

enum Method {
    Get,
    Post,
}

fn main() {
    let cli = Cli::parse();

    let url = cli.url;
    let method = if let Some(m) = cli.method {
        // 「-X POST」でも「-X post」でも対応できるようにto_uppercase()しておく
        match m.to_uppercase().as_str() {
            "POST" => Method::Post,
            "GET" => Method::Get,
            // POST, GET以外のメソッドは非対応
            _ => unimplemented!(),
        }
    } else {
        Method::Get
    };

    let client = reqwest::blocking::Client::new();
    let response = match method {
        Method::Get => client.get(url).send().unwrap(),
        Method::Post => client.post(url).send().unwrap(),
    };
    println!("{}", response.text().unwrap());
}
