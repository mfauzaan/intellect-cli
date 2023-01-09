use clap::{Parser, Subcommand};
use std::process::ExitCode;

mod crypto_currency;
mod weather;

use crypto_currency::get_top_currencies;
use weather::get_weather;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Whats the weather like today?
    Weather {
        /// List of the current city
        #[arg(short, long)]
        city: String,

        /// API key from open weather map
        #[arg(short, long)]
        api_key: String,
    },
    /// How's the crypto currency market doing?
    CryptoCurrency {
        /// API key from Coin Market Cap
        #[arg(short, long)]
        api_key: String,
    },
}

#[tokio::main]
async fn main() -> ExitCode {
    let cli = Cli::parse();

    let code = match &cli.command {
        Command::Weather { city, api_key } => get_weather(city, api_key).await,
        Command::CryptoCurrency { api_key } => get_top_currencies(api_key).await,
    };

    ExitCode::from(code)
}
