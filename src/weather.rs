use colored::*;
use prettytable::{Cell, Row, Table};
use reqwest::StatusCode;
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

pub fn get_weather(city: &String, api_key: &String) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "Fetching weather from open weather...".green());

    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}",
        city, api_key
    );

    let response_data = reqwest::blocking::get(url)?;

    match response_data.status() {
        StatusCode::OK => {
            let weather_data = response_data.json::<Weather>()?;

            // Create a table and add the data to it
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
        // Print error;
        _ => {
            println!("{}: {}", "error".red(), response_data.status(),);
        }
    };

    Ok(())
}
