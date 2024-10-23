use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrentUnits {
    time: String,
    interval: String,
    temperature_2m: String,
    relative_humidity_2m: String,
    apparent_temperature: String,
    precipitation: String,
    wind_speed_10m: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Current {
    time: String,
    interval: i32,
    temperature_2m: f32,
    relative_humidity_2m: i32,
    apparent_temperature: f32,
    precipitation: f32,
    wind_speed_10m: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse {
    latitude: f32,
    longitude: f32,
    utc_offset_seconds: i32,
    timezone: String,
    timezone_abbreviation: String,
    current_units: CurrentUnits,
    current: Current,
}
