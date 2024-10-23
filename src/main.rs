#[cfg(test)]
mod tests;
mod types;

use reqwest;
use serde_json::to_string_pretty;
use std::error::Error;
use types::ApiResponse;

const BASE_API: &'static str = "https://api.open-meteo.com/v1/forecast?latitude=44.4048&longitude=8.9444&current=temperature_2m,relative_humidity_2m,apparent_temperature,precipitation,wind_speed_10m&timezone=Europe%2FBerlin";

fn main() -> Result<(), Box<dyn Error>> {
    let response: ApiResponse = reqwest::blocking::get(BASE_API)?.json()?;
    let pretty = to_string_pretty(&response)?;
    println!("{}", pretty);

    Ok(())
}

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn Error>> {
//     let client = reqwest::Client::new();
//     let response: ApiResponse = client.get(BASE_API).send().await?.json().await?;

//     let pretty = to_string_pretty(&response)?;
//     println!("{}", pretty);

//     Ok(())
// }
