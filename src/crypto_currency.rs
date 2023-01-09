use colored::*;
use prettytable::{Cell, Row, Table};
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

pub fn get_top_currencies(api_key: &str) -> u8 {
    println!("{}", "Fetching crypto currencies...".green());

    let weather_data = match fetch_cryptocurrency_data(api_key) {
        Ok(data) => data,
        Err(e) => {
            // early return on failure
            println!("{} {}", "Failed to fetch crypto currencies data".red(), e);
            return 1;
        }
    };

    print_cryptocurrency_data(weather_data);

    // Return success
    0
}

fn fetch_cryptocurrency_data(api_key: &str) -> Result<Cryptocurrency, reqwest::Error> {
    let client = reqwest::blocking::Client::new();
    client
        .get("https://pro-api.coinmarketcap.com/v1/cryptocurrency/listings/latest")
        .query(&[("limit", 10)])
        .query(&[("CMC_PRO_API_KEY", api_key)])
        .send()?
        .error_for_status()?
        .json()
}

fn print_cryptocurrency_data(cryptocurrency_data: Cryptocurrency) {
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

    for i in cryptocurrency_data.data {
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
