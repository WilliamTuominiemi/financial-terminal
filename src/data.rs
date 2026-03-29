use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct ApiResponse {
    #[serde(rename = "Weekly Time Series")]
    time_series: HashMap<String, WeeklyEntry>,
}

#[derive(Debug, Deserialize)]
pub struct WeeklyEntry {
    #[serde(rename = "1. open")]
    open: String,
    #[serde(rename = "2. high")]
    high: String,
    #[serde(rename = "3. low")]
    low: String,
    #[serde(rename = "4. close")]
    close: String,
}

#[derive(Debug)]
pub struct StockSummary {
    pub date: String,
    pub average: f64,
}

pub fn send(url: &str) -> Result<Vec<StockSummary>, Box<dyn std::error::Error>> {
    let response: ApiResponse = ureq::get(url).call()?.body_mut().read_json()?;

    let summaries = response
        .time_series
        .into_iter()
        .map(|(date, entry)| {
            let open: f64 = entry.open.parse().unwrap_or(0.0);
            let high: f64 = entry.high.parse().unwrap_or(0.0);
            let low: f64 = entry.low.parse().unwrap_or(0.0);
            let close: f64 = entry.close.parse().unwrap_or(0.0);
            let average = (open + high + low + close) / 4.0;
            StockSummary { date, average }
        })
        .collect();

    Ok(summaries)
}
