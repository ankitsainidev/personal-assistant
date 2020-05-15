use reqwest;
use serde;
use serde_json;
use std::fmt;
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
#[derive(Debug, serde::Deserialize)]
struct Wind {
    speed: f64,
    deg: i32,
}
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
    wind: Wind,
    dt: i32,
    sys: ExtraInfo,
    timezone: i32,
    id: i32,
    name: String,
    cod: i32,
}

// TODO: show more info in more readable form
impl fmt::Display for WeatherResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{city}   {main}
{description}
Temperature: {temp}",
            city = self.name,
            main = self.weather[0].main,
            description = self.weather[0].description,
            temp = self.main.temp
        )
    }
}

pub async fn now(
    key: String,
    postal: String,
    country_code: String,
) -> Result<WeatherResponse, reqwest::Error> {
    let url = format!(
        "http://api.openweathermap.org/data/2.5/weather?zip={},{}&appid={}",
        postal, country_code, key
    );
    let data = reqwest::get(&url).await?.text().await?;
    Ok(serde_json::from_str(&data).unwrap())
}
