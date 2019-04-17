use rocket_contrib::json::Json;
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};

const OPEN_WEATHER_ENDPOINT: &str  = "https://api.openweathermap.org/data/2.5/weather";
const CURITIBA_ID: u32 = 6322752;

#[derive(Serialize, Debug)]
pub struct IsItRaining {
    pub response: bool,
    pub timestamp: u32,
}

#[derive(Serialize, Debug)]
pub struct Error {
    error: String,
}

#[get("/is_it_raining")]
pub fn is_it_raining() -> Result<Json<IsItRaining>, Error> {
    let url_request = create_request();
    match execute_request(url_request) {
        Ok(open_weather_response) => Ok(code_to_raining_state(open_weather_response)),
        Err(_) => Err(Error{error:"Open Weather is out of service!".to_string()}),
    }
}

fn create_request() -> String {
    let appkey = match env::var("APPKEY") {
        Ok(key) => key,
        Err(_) => "042b9d1380077ff85a168b9869071e30".to_string(),
    };
    let endpoint = format!("{}?id={}&appid={}", OPEN_WEATHER_ENDPOINT, CURITIBA_ID, appkey);
    endpoint.to_string()
}

fn execute_request(url_request: String) -> Result<OpenWeatherResponse, Box<std::error::Error>> {
    let resp: OpenWeatherResponse = reqwest::get(url_request.as_str())?.json()?;
    Ok(resp)
}

/* https://openweathermap.org/weather-conditions */
fn code_to_raining_state(open_weather_response: OpenWeatherResponse) -> Json<IsItRaining> {
    match open_weather_response.weather[0].id {
        230 ... 531 => Json(IsItRaining {response: true, timestamp: get_timestamp()}),
        _ => Json(IsItRaining {response: false, timestamp: get_timestamp()}),
    }
}

fn get_timestamp() -> u32 {
    let start = SystemTime::now();
    match start.duration_since(UNIX_EPOCH) {
        Ok(s) => s.as_secs() as u32,
        Err(_) => panic!("Something very wrong happend!")
    }
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
