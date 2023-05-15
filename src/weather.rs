use std::fmt::Display;

use reqwest::Client;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Location {
    pub name: String,
    pub region: String,
}

#[derive(Deserialize)]
pub struct Weather {
    pub temp_c: f32,
    pub temp_f: f32,
}

#[derive(Deserialize)]
pub struct Forecast {
    pub location: Location,
    pub current: Weather,
}

#[derive(Debug)]
pub struct CouldNotFindLocation {
    place: String,
}
impl Display for CouldNotFindLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Could not find location '{}'", self.place)
    }
}
impl std::error::Error for CouldNotFindLocation {}

pub async fn get_forecast(
    place: &str,
    api_key: &str,
    client: &Client,
) -> Result<Forecast, Box<dyn std::error::Error>> {
    const WEATHER_REQUEST: &str = "https://api.weatherapi.com/v1/current.json";

    let url = format!("{}?key={}&q={}", WEATHER_REQUEST, api_key, place);
    let request = client.get(url).build().unwrap();
    let res = client.execute(request).await?;
    if !res.status().is_success() {
        let box_err = Box::new(CouldNotFindLocation {
            place: place.to_owned(),
        });
        return Err(box_err);
    };

    let res = res.json::<Forecast>().await?;
    Ok(res)
}
