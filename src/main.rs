#![allow(unused)]
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

    let resp = fetch_url(mvg_zdm, station_ids_endpoint.url).await?;
    println!("{:#?}", resp);
    Ok(())
}

async fn fetch_url(base_url: &str, url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let full_url = format!("{}{}", base_url, url);

    let resp = reqwest::get(full_url)
        .await?
        .json::<serde_json::Value>()
        .await?;
    println!("{:#?}", resp);
    Ok(())
}
