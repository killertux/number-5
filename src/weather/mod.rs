use serde::{Deserialize, Serialize};
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

const OPEN_WEATHER_ENDPOINT: &str = "https://api.openweathermap.org/data/2.5/weather";
const CURITIBA_ID: u32 = 6322752;

#[derive(Serialize, Debug)]
pub enum WeatherResponse {
    IsItRaining { response: bool, timestamp: u32 },
    Error { message: &'static str },
}

pub async fn is_it_raining() -> Result<impl warp::Reply, std::convert::Infallible> {
    let url_request = create_request();

    let response = execute_request(url_request)
        .await
        .map(code_to_raining_state)
        .unwrap_or(WeatherResponse::Error {
            message: "Open Weather is out of service!",
        });

    Ok(warp::reply::json(&response))
}

fn create_request() -> String {
    let appkey = env::var("APPKEY").unwrap_or("put_key_here".to_string());

    format!(
        "{}?id={}&appid={}",
        OPEN_WEATHER_ENDPOINT, CURITIBA_ID, appkey
    )
}

async fn execute_request(
    url_request: String,
) -> Result<OpenWeatherResponse, Box<dyn std::error::Error>> {
    let resp: OpenWeatherResponse = reqwest::get(url_request.as_str()).await?.json().await?;
    Ok(resp)
}

/* https://openweathermap.org/weather-conditions */
fn code_to_raining_state(open_weather_response: OpenWeatherResponse) -> WeatherResponse {
    WeatherResponse::IsItRaining {
        timestamp: get_timestamp(),
        response: match open_weather_response.weather[0].id {
            230..=531 => true,
            _ => false,
        },
    }
}

fn get_timestamp() -> u32 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Something very wrong happend!")
        .as_secs() as u32
}

#[derive(Deserialize, Debug)]
struct OpenWeatherResponse {
    coord: Coord,
    weather: [Weather; 1],
    base: String,
    main: Main,
    visibility: i32,
    wind: Wind,
    clouds: Clouds,
    dt: i32,
    sys: Sys,
    id: i32,
    name: String,
    cod: i32,
}

#[derive(Deserialize, Debug)]
struct Coord {
    lon: f32,
    lat: f32,
}

#[derive(Deserialize, Debug)]
struct Weather {
    id: u32,
    main: String,
    description: String,
    icon: String,
}

#[derive(Deserialize, Debug)]
struct Main {
    temp: f32,
    pressure: i32,
    humidity: i32,
    temp_min: f32,
    temp_max: f32,
}

#[derive(Deserialize, Debug)]
struct Wind {
    speed: f32,
    deg: Option<i32>,
}

#[derive(Deserialize, Debug)]
struct Clouds {
    all: i32,
}

#[derive(Deserialize, Debug)]
struct Sys {
    r#type: i32,
    id: i32,
    message: f32,
    country: String,
    sunrise: i32,
    sunset: i32,
}
