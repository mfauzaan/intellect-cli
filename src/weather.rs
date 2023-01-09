use colored::*;
use prettytable::{Cell, Row, Table};
use serde::Deserialize;

#[derive(Deserialize)]
struct Weather {
    main: Main,
    name: String,
}

#[derive(Deserialize)]
struct Main {
    temp: f64,
    feels_like: f64,
    humidity: f64,
}

pub fn get_weather(city: &str, api_key: &str) -> u8 {
    println!("{}", "Fetching weather from open weather...".green());

    let weather_data = match fetch_weather_data(city, api_key) {
        Ok(data) => data,
        Err(e) => {
            // early return on failure
            println!("Failed to fetch weather data: {}", e);
            return 1;
        }
    };

    print_weather_data(&weather_data);

    // Return success
    0
}

fn fetch_weather_data(city: &str, api_key: &str) -> Result<Weather, reqwest::Error> {
    let client = reqwest::blocking::Client::new();
    client
        .get("https://api.openweathermap.org/data/2.5/weather")
        .query(&[("q", city)])
        .query(&[("appid", api_key)])
        .send()?
        .error_for_status()?
        .json()
}

fn print_weather_data(weather_data: &Weather) {
    let mut table = Table::new();
    table.add_row(Row::new(vec![
        Cell::new("City"),
        Cell::new(&weather_data.name),
    ]));

    table.add_row(Row::new(vec![
        Cell::new("Temperature"),
        Cell::new(&format!("{}°C", weather_data.main.temp)),
    ]));

    table.add_row(Row::new(vec![
        Cell::new("Feels Like"),
        Cell::new(&format!("{}°C", weather_data.main.feels_like)),
    ]));

    table.add_row(Row::new(vec![
        Cell::new("Humidity"),
        Cell::new(&format!("{}%", weather_data.main.humidity)),
    ]));

    table.printstd();
}
