use colored::*;
use dotenv::dotenv;
use serde::Deserialize;
use std::env;
use std::io;

#[derive(Deserialize, Debug)]
struct WeatherResponse {
    weather: Vec<Weather>,
    main: Main,
    wind: Wind,
    name: String,
    clouds: Clouds,
    coord: Coord,
}

#[derive(Deserialize, Debug)]
struct Weather {
    description: String,
}

#[derive(Deserialize, Debug)]
struct Main {
    temp: f32,
    feels_like: f32,
    humidity: f32,
}

#[derive(Deserialize, Debug)]
struct Wind {
    speed: f32,
}

#[derive(Deserialize, Debug)]
struct Clouds {
    all: f32,
}

#[derive(Deserialize, Debug)]
struct Coord {
    lon: f32,
    lat: f32,
}

fn get_weather_info(
    city: &str,
    country: &str,
    api_key: String,
) -> Result<WeatherResponse, reqwest::Error> {
    let url: String = format!(
        "http://api.openweathermap.org/data/2.5/weather?q={},{}&appid={}&units=metric",
        city, country, api_key
    );

    let response = reqwest::blocking::get(&url)?;
    let response_json: WeatherResponse = response.json::<WeatherResponse>()?;

    Ok(response_json)
}

fn display_weather_info(repsponse: &WeatherResponse) {
    let description = &repsponse.weather[0].description;
    let temp = &repsponse.main.temp;
    let humidity = repsponse.main.humidity;
    let wind_speed = repsponse.wind.speed;
    let feels_like = repsponse.main.feels_like;
    let clouds = repsponse.clouds.all;
    let lon = repsponse.coord.lon;
    let lat = repsponse.coord.lat;

    let weather_info = format!(
    "Weather in {}: {} {} \n -> Temperature: {}°C (feels like {}°C) \n -> Humidity: {}% \n -> Wind Speed: {}m/s \n -> Cloudiness: {}% \n -> Longitude: {} \n -> Latitude: {}",
    repsponse.name,
    description,
    get_temp_emoji(*temp),
    temp,
    feels_like,
    humidity,
    wind_speed,
    clouds,
    lon,
    lat
    );

    let weather_text_colored: colored::ColoredString = match description.as_str() {
        "clear sky" => weather_info.yellow(),
        "few clouds" => weather_info.yellow(),
        "scattered clouds" => weather_info.yellow(),
        "broken clouds" => weather_info.yellow(),
        "shower rain" => weather_info.blue(),
        "rain" => weather_info.blue(),
        "thunderstorm" => weather_info.blue(),
        "snow" => weather_info.white(),
        "mist" => weather_info.white(),
        "smoke" => weather_info.white(),
        "haze" => weather_info.white(),
        "dust" => weather_info.white(),
        "fog" => weather_info.white(),
        _ => weather_info.normal(),
    };

    println!("{}", weather_text_colored);

    fn get_temp_emoji(temp: f32) -> &'static str {
        match temp {
            (..0.0) => "🧊",
            (0.0..10.0) => "🥶",
            (10.0..20.0) => "😬",
            (20.0..30.0) => "😅",
            _ => "🥵",
        }
    }
}

fn main() {
    dotenv().ok();
    println!("{}", "Welcome to Atmos!".bright_yellow());
    loop {
        // Get the city name from the user
        println!("{}", "Please enter the city name:".bright_green());
        let mut city = String::new();
        io::stdin()
            .read_line(&mut city)
            .expect("Failed to read line");
        let city: &str = city.trim();

        // Get the country code from the user
        println!("{}", "Please enter the country code:".bright_green());
        let mut country = String::new();
        io::stdin()
            .read_line(&mut country)
            .expect("Failed to read line");
        let country: &str = country.trim();

        let api_key = env::var("API_KEY").expect("API_KEY must be set");

        match get_weather_info(city, country, api_key) {
            Ok(response) => {
                display_weather_info(&response);
            }
            Err(error) => {
                println!("{}", error);
            }
        }

        println!(
            "{}",
            "Do you want to check the weather for another city? (y/n)".bright_green()
        );

        let mut response = String::new();

        io::stdin()
            .read_line(&mut response)
            .expect("Failed to read line");

        let response = response.trim().to_lowercase();

        if response != "y" {
            break;
        }

        println!("{}", "Thank you for using Atmos!".bright_yellow());
    }
}
