use reqwest;
use serde;
use serde_json;
use std::fmt;
use std::fs;
use std::path::Path;

// Structs for deserializing response
/* Todo: Due to api response instability change the implementation to use
default serde_json::Value enum */
#[derive(Debug, serde::Deserialize)]
struct Coordinates {
    lon: f32,
    lat: f32,
}
#[derive(Debug, serde::Deserialize)]
struct WeatherOverview {
    id: i32,
    main: String,
    description: String,
    icon: String,
}

#[derive(Debug, serde::Deserialize)]
struct WeatherMain {
    temp: f64,
    feels_like: f64,
    temp_min: f64,
    temp_max: f64,
    pressure: f64,
    humidity: f64,
}
// #[derive(Debug, serde::Deserialize)]
// struct Wind {
//     speed: f64,
//     deg: i32,
// }
#[derive(Debug, serde::Deserialize)]
struct Clouds {
    all: i32,
}
#[derive(Debug, serde::Deserialize)]
struct ExtraInfo {
    r#type: i32,
    id: i32,
    country: String,
    sunrise: i32,
    sunset: i32,
}
#[derive(Debug, serde::Deserialize)]
pub struct WeatherResponse {
    coord: Coordinates,
    weather: Vec<WeatherOverview>,
    base: String,
    main: WeatherMain,
    visibility: i32,

    //wind field is not stable
    wind: serde_json::Value,
    dt: i32,
    sys: ExtraInfo,
    timezone: i32,
    id: i32,
    name: String,
    cod: i32,
}

// Structus for deserializing config
#[derive(Debug, serde::Deserialize)]
struct Apis {
    open_weather_map: String,
}
#[derive(Debug, serde::Deserialize)]
struct Location {
    postal_code: String,
    country_code: String,
}
#[derive(Debug, serde::Deserialize)]
struct Locations {
    home: Location,
}
#[derive(Debug, serde::Deserialize)]
struct Config {
    api_keys: Apis,
    locations: Locations,
}

// TODO: show more info in more readable form
impl fmt::Display for WeatherResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{city}   {main}
{description}
Temperature:
    {temp:.2} °C
    feels like: {temp_feel:.2} °C
Wind:
    Speed: {wind:.2} m/s
    Direction: {wind_deg}° ",
            city = self.name,
            main = self.weather[0].main,
            description = self.weather[0].description,
            wind = self.wind["speed"],
            wind_deg = self.wind["deg"],
            //Kelvin to celcius
            temp = self.main.temp - 273.15,
            temp_feel = self.main.feels_like - 273.15,
        )
    }
}

pub async fn now(
    key: &str,
    postal: &str,
    country_code: &str,
) -> Result<WeatherResponse, reqwest::Error> {
    let url = format!(
        "http://api.openweathermap.org/data/2.5/weather?zip={},{}&appid={}",
        postal, country_code, key
    );
    let data = reqwest::get(&url).await?.text().await?;
    Ok(serde_json::from_str(&data).unwrap())
}

pub async fn from_config(path: &Path) -> Result<WeatherResponse, reqwest::Error> {
    let weather_config_string: String = fs::read_to_string(path)
        .expect("Can't open weather config file")
        .parse()
        .expect("Can't read content of config file");
    let weather_config: Config = serde_json::from_str(&weather_config_string).unwrap();
    let Config {
        api_keys: Apis {
            open_weather_map: weather_api_key,
        },
        locations:
            Locations {
                home:
                    Location {
                        postal_code,
                        country_code,
                    },
            },
    } = &weather_config;
    now(weather_api_key, postal_code, country_code).await
}

#[cfg(test)]
mod tests {
    // interacts with file I/O and should be tested with --test-threads=1

    use super::*;
    use crate::setup;
    use std::io::ErrorKind;

    fn setup() {
        match fs::remove_dir_all("./testing_files") {
            Err(e) => {
                if e.kind() != ErrorKind::NotFound {
                    panic!("can't setup");
                }
            }
            _ => {}
        }
        setup::main("./testing_files");
    }

    #[tokio::test]
    async fn api_connection() {
        self::setup();
        let config_path = Path::new("./testing_files/.pat/config.json");
        from_config(&config_path)
            .await
            .expect("Error connecting to api");
    }

    #[tokio::test]
    #[ignore]
    async fn extensive_api_test() {
        self::setup();
        let config_path = Path::new("./testing_files/.pat/config.json");
        let data: WeatherResponse = from_config(&config_path)
            .await
            .expect("Error connecting to api");
        let data_string = format!("{}", data);
        if data_string.contains("null") {
            panic!("some field is missing");
        }
    }
}
