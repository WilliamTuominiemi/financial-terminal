use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();
    let api_key = env::var("API_KEY").expect("API_KEY not set");

    let symbol = "AAPL";
    let url = format!(
        "https://www.alphavantage.co/query?function=TIME_SERIES_WEEKLY&symbol={symbol}&apikey={api_key}"
    );

    match send(&url) {
        Ok(body) => println!("{}", body),
        Err(e) => eprintln!("Request failed: {}", e),
    }
}

fn send(url: &str) -> Result<String, ureq::Error> {
    let body = ureq::get(url).call()?.body_mut().read_to_string()?;
    Ok(body)
}
