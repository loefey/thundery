use chrono::{DateTime, Duration, TimeZone, Utc};
use colored::Colorize;
use dirs::home_dir;
use reqwest::blocking::get;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;
use std::fs::File;
use std::io::Write;

#[derive(Deserialize, Serialize)]
struct Config {
    api_key: String,
    city: String,
    units: String,
    timeplus: i64,
    timeminus: i64,
    showcityname: bool,
    showdate: bool,
    timeformat: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            api_key: String::new(),
            city: String::new(),
            units: String::from("metric"),
            timeplus: 0,
            timeminus: 0,
            showcityname: false,
            showdate: false,
            timeformat: String::from("24"),
        }
    }
}

fn read_config() -> Config {
    let config_path = if cfg!(windows) {
        let mut path = dirs::config_dir().expect("Failed to get config directory");
        path.push("thundery");
        path.push("thundery.toml");
        path
    } else {
        let mut path = home_dir().expect("Failed to get home directory");
        path.push(".config");
        path.push("thundery");
        path.push("thundery.toml");
        path
    };

    if !config_path.exists() {
        let default_config = Config::default();
        let toml_string =
            toml::to_string(&default_config).expect("Failed to serialize default config");
        fs::create_dir_all(config_path.parent().unwrap())
            .expect("Failed to create config directory");
        let mut file = File::create(&config_path).expect("Failed to create config file");
        file.write_all(toml_string.as_bytes())
            .expect("Failed to write default config to file");

        let config_location = config_path.display().to_string();
        println!("No config detected, config made at {}.", config_location);
    }

    let config_content = fs::read_to_string(config_path).expect("Failed to read config file");
    toml::from_str(&config_content).expect("Failed to parse config file")
}

fn main() {
    let config = read_config();

    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&units={}&APPID={}",
        config.city, config.units, config.api_key
    );

    let response = get(&url).expect("Failed to send request");

    if response.status().is_success() {
        let weather_data: Value = response.json().expect("Failed to parse JSON");

        let weather = weather_data["weather"][0]["main"]
            .as_str()
            .unwrap_or("Unknown");
        let temp = weather_data["main"]["temp"].as_f64().unwrap_or(0.0);
        let wind_speed = weather_data["wind"]["speed"].as_f64().unwrap_or(0.0);
        let sunrise = weather_data["sys"]["sunrise"].as_i64().unwrap_or(0);
        let sunset = weather_data["sys"]["sunset"].as_i64().unwrap_or(0);

        let windspeedunits = match config.units.as_str() {
            "metric" => "m/s",
            "imperial" => "mph",
            _ => "m/s",
        };

        let temp_unit = match config.units.as_str() {
            "metric" => "°C",
            "imperial" => "°F",
            _ => "K",
        };

        let temp_str = format!("{:.1}{}", temp, temp_unit);
        let wind_speed_str = format!("{:.1} {}", wind_speed, windspeedunits);

        let sunrise_datetime: DateTime<Utc> = Utc.timestamp_opt(sunrise, 0).unwrap();
        let sunset_datetime: DateTime<Utc> = Utc.timestamp_opt(sunset, 0).unwrap();

        let adjusted_sunrise =
            sunrise_datetime + Duration::hours(config.timeplus) - Duration::hours(config.timeminus);
        let adjusted_sunset =
            sunset_datetime + Duration::hours(config.timeplus) - Duration::hours(config.timeminus);

        let time_format = match config.timeformat.as_str() {
            "12" => "%I:%M %p",
            _ => "%H:%M",
        };

        let sunrisestring = adjusted_sunrise.format(time_format).to_string();
        let sunsetstring = adjusted_sunset.format(time_format).to_string();

        let date = if config.showdate {
            let now = Utc::now();
            now.format("%x").to_string()
        } else {
            String::new()
        };

        let date_label = if config.showdate { "Date: " } else { "" };
        let date_value = if config.showdate { date } else { String::new() };

        let output = match weather {
            "Clear" => format!(
                r#"                    {}
          \   /     {}
           .-.      {}
        ‒ (   ) ‒   {}
           ʻ-ʻ      {}
          /   \     {}
                    {}{}"#,
                if config.showcityname {
                    format!("City: {}", config.city).bold().green().to_string()
                } else {
                    String::new()
                },
                "Weather: Clear".yellow().bold().to_string(),
                format!("Temperature: {temp_str}").red().to_string(),
                format!("Wind speed: {wind_speed_str}").green().to_string(),
                format!("Sunrise: {sunrisestring}").yellow().to_string(),
                format!("Sunset: {sunsetstring}").blue().to_string(),
                date_label.white().to_string(),
                date_value.white().to_string()
            ),
            "Clouds" => format!(
                r#"                      {}
            .--.      {}
         .-(    ).    {}
        (___.__)__)   {}
                      {}
                      {}
                      {}{}"#,
                if config.showcityname {
                    format!("City: {}", config.city).bold().green().to_string()
                } else {
                    String::new()
                },
                "Weather: Clouds".bold().magenta().to_string(),
                format!("Temperature {temp_str}").red().to_string(),
                format!("Wind Speed: {wind_speed_str}").green().to_string(),
                format!("Sunrise: {sunrisestring}").yellow().to_string(),
                format!("Sunset: {sunsetstring}").blue().to_string(),
                date_label.white().to_string(),
                date_value.white().to_string()
            ),
            "Rain" => format!(
                r#"                      {}
            .--.      {}
         .-(    ).    {}
        (___.__)__)   {}
         ʻ‚ʻ‚ʻ‚ʻ‚ʻ    {}
                      {}
                      {}{}"#,
                if config.showcityname {
                    format!("City: {}", config.city).bold().green().to_string()
                } else {
                    String::new()
                },
                "Weather: Rain".bold().blue().to_string(),
                format!("Temperature {temp_str}").red().to_string(),
                format!("Wind Speed: {wind_speed_str}").green().to_string(),
                format!("Sunrise: {sunrisestring}").yellow().to_string(),
                format!("Sunset: {sunsetstring}").blue().to_string(),
                date_label.white().to_string(),
                date_value.white().to_string()
            ),
            "Snow" => format!(
                r#"                      {}
            .--.      {}
         .-(    ).    {}
        (___.__)__)   {}
          * * * *     {}
         * * * *      {}
                      {}{}"#,
                if config.showcityname {
                    format!("City: {}", config.city).bold().green().to_string()
                } else {
                    String::new()
                },
                "Weather: Snow".bold().magenta().to_string(),
                format!("Temperature {temp_str}").white().to_string(),
                format!("Wind Speed: {wind_speed_str}").green().to_string(),
                format!("Sunrise: {sunrisestring}").yellow().to_string(),
                format!("Sunset: {sunsetstring}").blue().to_string(),
                date_label.white().to_string(),
                date_value.white().to_string()
            ),
            "Thundestorm" => format!(
                r#"                      {}
            .--.      {}
         .-(    ).    {}
        (___.__)__)   {}
           /_  /_     {}
            /  /      {}
                      {}{}"#,
                if config.showcityname {
                    format!("City: {}", config.city).bold().green().to_string()
                } else {
                    String::new()
                },
                "Weather: Thunderstorm".bold().yellow().to_string(),
                format!("Temperature {temp_str}").red().to_string(),
                format!("Wind Speed: {wind_speed_str}").green().to_string(),
                format!("Sunrise: {sunrisestring}").yellow().to_string(),
                format!("Sunset: {sunsetstring}").blue().to_string(),
                date_label.white().to_string(),
                date_value.white().to_string()
            ),
            _ => format!(
                r#"                      {}
            .--.      {}
         .-(    ).    {}
        (___.__)__)   {}
                      {}
                      {}
                      {}{}"#,
                if config.showcityname {
                    format!("City: {}", config.city).bold().green().to_string()
                } else {
                    String::new()
                },
                "Weather: {weather}".bold().red().to_string(),
                format!("Temperature {temp_str}").red().to_string(),
                format!("Wind Speed: {wind_speed_str}").green().to_string(),
                format!("Sunrise: {sunrisestring}").yellow().to_string(),
                format!("Sunset: {sunsetstring}").blue().to_string(),
                date_label.white().to_string(),
                date_value.white().to_string()
            ),
        };
        println!("{}", output);
    } else {
        eprintln!(
            "Failed to fetch weather data: your api key and city are probably missing from the config file."
        );
    }
}
