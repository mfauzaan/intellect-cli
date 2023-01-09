use colored::*;
use prettytable::{Cell, Row, Table};
use reqwest::StatusCode;
use serde::Deserialize;

#[derive(Deserialize)]
struct Cryptocurrency {
    data: Vec<Data>,
}

#[derive(Deserialize)]
struct Data {
    name: String,
    symbol: String,
    cmc_rank: u64,
    quote: Quote,
}

#[derive(Deserialize)]
struct Quote {
    #[serde(rename = "USD")]
    usd: Usd,
}

#[derive(Deserialize)]
struct Usd {
    price: f64,
    percent_change_24h: f64,
    percent_change_7d: f64,
    market_cap: f64,
    volume_24h: f64,
}

pub fn get_top_currencies(api_key: &String) -> Result<(), Box<dyn std::error::Error>> {
    println!("Fetching crypto currencies...");

    let url = format!(
        "https://pro-api.coinmarketcap.com/v1/cryptocurrency/listings/latest?limit=10&sort=market_cap&CMC_PRO_API_KEY={}",
        api_key
    );

    let response_data = reqwest::blocking::get(url)?;

    match response_data.status() {
        StatusCode::OK => {
            let crypto_currency_data = response_data.json::<Cryptocurrency>()?;

            // Create a table and add the data to it
            let mut table = Table::new();

            table.add_row(Row::new(vec![
                Cell::new("#"),
                Cell::new("Name"),
                Cell::new("Price"),
                Cell::new("24h %"),
                Cell::new("7d %"),
                Cell::new("Market Cap"),
                Cell::new("24h Volume"),
            ]));

            for i in crypto_currency_data.data {
                // Set ticker based on price moment:
                let ticker = if i.quote.usd.percent_change_24h > 0.0 {
                    "↑"
                } else {
                    "↓"
                };

                table.add_row(Row::new(vec![
                    Cell::new(&format!("{}", i.cmc_rank)),
                    Cell::new(&format!("{} ({}) {}", i.name, i.symbol, ticker)),
                    Cell::new(&format!("${:.1$}", i.quote.usd.price, 2)),
                    Cell::new(&format!("{:.1$}", i.quote.usd.percent_change_24h, 2)),
                    Cell::new(&format!("{:.1$}", i.quote.usd.percent_change_7d, 2)),
                    Cell::new(&format!("${:.1$}", i.quote.usd.market_cap, 2)),
                    Cell::new(&format!("{:.1$}", i.quote.usd.volume_24h, 2)),
                ]));
            }

            table.printstd();
        }
        // Print error;
        _ => {
            println!("{}: {}", "error".red(), response_data.status(),);
        }
    };

    Ok(())
}
