use clap::{Parser, Subcommand};

mod crypto_currency_client;
mod weather_client;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
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
        Some(Commands::Weather { city, api_key }) => {
            let _weather = weather_client::get_weather(city, api_key);
        }
        Some(Commands::CryptoCurrency { api_key }) => {
            let _crypto_currency = crypto_currency_client::get_top_currencies(api_key);
        }
        None => {}
    }
}
