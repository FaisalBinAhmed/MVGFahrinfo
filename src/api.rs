use std::fs::File;

// #[allow(unused, dead_code, unused_)]
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")] //to avoid renaming all the fields to snake_case
pub struct StationInfo {
    pub house_number: String,
    pub latitude: f32,
    pub longitude: f32,
    pub name: String,
    pub place: String,
    pub post_code: String,
    pub street: String,
    pub r#type: String, //type is a reserved keyword in Rust
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")] //to avoid renaming all the fields to snake_case
pub struct DepartureInfo {
    pub planned_departure_time: i64,
    pub realtime: bool,
    pub delay_in_minutes: Option<i64>,
    pub realtime_departure_time: i64, // utc time stamp
    pub transport_type: String,       //"UBAHN",
    pub label: String,                //"U8",
    pub diva_id: String,              //"010U8",
    pub network: String,              //"swm",
    pub train_type: String,           //"",
    pub destination: String,          //"Messestadt Ost",
    pub cancelled: bool,
    pub sev: bool,
    pub platform: Option<i64>,
    pub messages: Vec<String>,
    pub banner_hash: String,          //"",
    pub occupancy: String,            //"UNKNOWN",
    pub stop_point_global_id: String, //"de:09162:6:52:52"
}

pub async fn get_departures(id: &str) -> Result<Vec<DepartureInfo>> {
    let full_url = format!("https://www.mvg.de/api/bgw-pt/v3/departures?globalId={}", id);

    let resp = reqwest::get(full_url)
        .await?
        .json::<Vec<DepartureInfo>>()
        .await?;
    // println!("{:#?}", resp[0]);
    // return Ok(resp[0].clone());
    Ok(resp)
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Station {
    pub name: String,
    pub place: String,
    pub id: String,
    pub diva_id: i64,
    pub abbreviation: Option<String>, //"DBR"
    pub tariff_zones: String,         // "m" , "m|1"
    pub products: Vec<String>,
    pub latitude: f32,
    pub longitude: f32, //type is a reserved keyword in Rust
}

// "name":"Karlsplatz (Stachus)",
//       "place":"München",
//       "id":"de:09162:1",
//       "divaId":1,
//       "abbreviation":"KA",
//       "tariffZones":"m",
//       "products":[
//          "UBAHN",
//          "BUS",
//          "TRAM",
//          "SBAHN"
//       ],
//       "latitude":48.13951,
//       "longitude":11.56613

// todo: we need a way to manually refrest this file
pub async fn get_stations() -> Result<Vec<Station>> {
    if let Ok(file) = File::open("stations.json") {
        //todo: handle the error propagation here, it should fetch from api instead
        let stations = serde_json::from_reader(file)?; //it inferres the type from the function return type and automatically deserializes it
        Ok(stations)
    } else {
        let full_url = "https://www.mvg.de/.rest/zdm/stations";

        let resp = reqwest::get(full_url).await?;

        let stations = resp.json::<Vec<Station>>().await?;
        match save_response_to_json_file(stations.clone()).await {
            Ok(_) => println!("saved stations to file"),
            Err(_) => println!("failed to save stations to file"),
        }
        Ok(stations)
    }
}

async fn save_response_to_json_file(station_response: Vec<Station>) -> Result<()> {
    let json = serde_json::to_string_pretty(&station_response)?;
    std::fs::write("stations.json", json)?; //stores this file in the root directory

    Ok(())
}
