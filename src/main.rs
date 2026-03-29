use dotenv::dotenv;
use serde::Deserialize;
use std::collections::HashMap;
use std::env;

#[derive(Debug, Deserialize)]
struct ApiResponse {
    #[serde(rename = "Time Series (Daily)")]
    time_series: HashMap<String, StockEntry>,
}

#[derive(Debug, Deserialize)]
struct StockEntry {
    #[serde(rename = "1. open")]
    open: String,
    #[serde(rename = "2. high")]
    high: String,
    #[serde(rename = "3. low")]
    low: String,
    #[serde(rename = "4. close")]
    close: String,
    #[serde(rename = "5. volume")]
    volume: String,
}
fn main() {
    dotenv().ok();
    let api_key = env::var("API_KEY").expect("API_KEY not set");

    match send(
        "https://www.alphavantage.co/query?function=TIME_SERIES_DAILY&symbol=IBM&apikey=demo",
    ) {
        Ok(entries) => {
            for (date, entry) in &entries {
                println!("{}: open={}, close={}", date, entry.open, entry.close);
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn send(url: &str) -> Result<HashMap<String, StockEntry>, Box<dyn std::error::Error>> {
    let response: ApiResponse = ureq::get(url).call()?.body_mut().read_json()?;
    Ok(response.time_series)
}
