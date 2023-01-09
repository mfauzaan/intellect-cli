use clap::{Parser, Subcommand};

mod crypto_currency;
mod weather;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
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

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Command::Weather { city, api_key }) => {
            let _weather = weather::get_weather(city, api_key);
        }
        Some(Command::CryptoCurrency { api_key }) => {
            let _crypto_currency = crypto_currency::get_top_currencies(api_key);
        }
        None => {}
    }
}
