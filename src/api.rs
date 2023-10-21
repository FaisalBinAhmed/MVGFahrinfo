use anyhow::Result;
use serde::Deserialize;
pub async fn fetch_station_ids() -> Result<Vec<String>> {
    let full_url = "https://www.mvg.de/.rest/zdm/mvgStationGlobalIds";

    let stations = reqwest::get(full_url).await?.json::<Vec<String>>().await?;
    // println!("{:#?}", stations);
    return Ok(stations);
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")] //to avoid renaming all the fields to snake_case
pub struct StationInfo {
    house_number: String,
    latitude: f32,
    longitude: f32,
    pub name: String,
    place: String,
    post_code: String,
    street: String,
    r#type: String, //type is a reserved keyword in Rust
}

pub async fn fetch_station_info(id: &str) -> Result<Vec<StationInfo>> {
    let full_url = format!("https://www.mvg.de/api/fib/v2/location?query={}", id);

    let resp = reqwest::get(full_url)
        .await?
        .json::<Vec<StationInfo>>()
        .await?;
    // println!("{:#?}", resp);
    // return Ok(resp[0].clone());
    Ok(resp)
}
