use rocket_contrib::json::Json;
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};

const OPEN_WEATHER_ENDPOINT: &str  = "https://api.openweathermap.org/data/2.5/weather";
const CURITIBA_ID: u32 = 6322752;

#[derive(Serialize, Debug)]
pub enum WeatherResponse {
    IsItRaining {
        response: bool,
        timestamp: u32,
    },
    Error {error: &'static str},
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
    cod: i32
}

#[get("/is-it-raining")]
pub fn is_it_raining() -> Json<WeatherResponse> {
    let url_request = create_request();
    let response : WeatherResponse = match execute_request(url_request) {
        Ok(open_weather_response) => code_to_raining_state(open_weather_response),
        Err(_) => WeatherResponse::Error{error:"Open Weather is out of service!"},
    };

    Json(response)
}

fn create_request() -> String {
    let appkey = env::var("APPKEY")
                    .unwrap_or("put_key_here".to_string());

    format!("{}?id={}&appid={}", OPEN_WEATHER_ENDPOINT, CURITIBA_ID, appkey)
}

fn execute_request(url_request: String) -> Result<OpenWeatherResponse, Box<std::error::Error>> {
    let resp: OpenWeatherResponse = reqwest::get(url_request.as_str())?.json()?;
    Ok(resp)
}

/* https://openweathermap.org/weather-conditions */
fn code_to_raining_state(open_weather_response: OpenWeatherResponse) -> WeatherResponse {
    match open_weather_response.weather[0].id {
        230 ... 531 => WeatherResponse::IsItRaining {response: true, timestamp: get_timestamp()},
        _ => WeatherResponse::IsItRaining {response: false, timestamp: get_timestamp()},
    }
}

fn get_timestamp() -> u32 {
    SystemTime::now().duration_since(UNIX_EPOCH)
        .expect("Something very wrong happend!")
        .as_secs() as u32
}
