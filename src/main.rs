use colored::*;

use reqwest::blocking::Client;
use serde::Deserialize;

#[derive(Deserialize)]
struct ExchangeRate {
    #[serde(rename = "USDBRL")]
    usd_brl: Rate,
}

#[derive(Deserialize)]
struct Rate {
    bid: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let response = client
        .get("https://economia.awesomeapi.com.br/last/USD-BRL")
        .send()?;

    let exchange_rate: ExchangeRate = response.json()?;
    println!(
        "Dollar/BRL Rate: {}",
        exchange_rate.usd_brl.bid.bright_green()
    );

    Ok(())
}
