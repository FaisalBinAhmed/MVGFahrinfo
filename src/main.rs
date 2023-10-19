#![allow(unused)]
use serde::Deserialize;
use serde_json::from_str;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mvg_fib = "https://www.mvg.de/api/fib/v2";
    let mvg_zdm = "https://www.mvg.de/.rest/zdm/";

    const LOCATION_URL: &'static str = "/location";

    struct EndPoint {
        url: &'static str,
        args: Vec<&'static str>,
    }

    let location_endpoint = EndPoint {
        url: LOCATION_URL,
        args: vec!["query"],
    };

    let station_ids_endpoint = EndPoint {
        url: "mvgStationGlobalIds",
        args: vec![],
    };

    // let resp = fetch_url(mvg_zdm, station_ids_endpoint.url).await?;

    let location_url = format!("{}{}", mvg_fib, LOCATION_URL);

    let resp = fetch_station_info(&location_url, "de:09162:6").await?;
    // println!("{:#?}", resp);
    Ok(())
}

async fn fetch_url(base_url: &str, url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let full_url = format!("{}{}", base_url, url);

    let resp = reqwest::get(full_url).await?.json::<Vec<String>>().await?;
    println!("{:#?}", resp);
    Ok(())
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")] //to avoid renaming all the fields to snake_case
struct StationInfo {
    house_number: String,
    latitude: f32,
    longitude: f32,
    name: String,
    place: String,
    post_code: String,
    street: String,
    // type: String
}

async fn fetch_station_info(url: &str, query: &str) -> Result<(), Box<dyn std::error::Error>> {
    let full_url = format!("{}?query={}", url, query);

    let resp = reqwest::get(full_url)
        .await?
        .json::<Vec<StationInfo>>()
        .await?;
    println!("{:#?}", resp);
    Ok(())
}
