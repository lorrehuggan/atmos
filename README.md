# Atmos

### Weather Forecast CLI App

==========================

A simple command-line interface (CLI) application built with Rust to fetch and display current weather forecasts from OpenWeatherMap API.

#### Features

- Fetches current weather forecasts from OpenWeatherMap API
- Displays temperature, humidity, wind speed, and other relevant weather conditions

#### Installation

```rust
[package]
name = "atmos"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { version = "1.0", features = ["derive"] }
reqwest = { version = "0.12", features = ["blocking", "json"] }
serde_json = "1.0"
colored = "2.0"
dotenv = "0.15"

```

1. Clone the repositry.
2. Run cargo build to compile the application.
3. Run cargo run to execute the application .

#### Usage

```bash
Welcome to Atmos!
Please enter the city name:
London
```

```bash
Please enter the country code:
uk
```

```bash
Weather in London: overcast clouds 😬
 -> Temperature: 14.86°C (feels like 14.2°C)
 -> Humidity: 69%
 -> Wind Speed: 1.54m/s
 -> Cloudiness: 95%
 -> Longitude: -0.1257
 -> Latitude: 51.5085
Do you want to check the weather for another city? (y/n)
```

#### API Documentation

The application uses the [OpenWeatherMap](https://openweathermap.org/) API to fetch current weather forecasts.

#### License

This application is licensed under the MIT License.
