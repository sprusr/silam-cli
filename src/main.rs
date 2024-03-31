use chrono::{DateTime, SecondsFormat, Utc};
use serde::Deserialize;
use thredds_catalog::ThreddsCatalog;

mod thredds_catalog;

#[derive(Debug, Deserialize)]
enum PollenIndex {
    #[serde(rename(deserialize = "1.0"))]
    VeryLow,
    #[serde(rename(deserialize = "2.0"))]
    Low,
    #[serde(rename(deserialize = "3.0"))]
    Moderate,
    #[serde(rename(deserialize = "4.0"))]
    High,
    #[serde(rename(deserialize = "5.0"))]
    VeryHigh,
}

#[derive(Debug, Deserialize)]
enum PollenType {
    #[serde(rename(deserialize = "-1.0"))]
    Unknown,
    #[serde(rename(deserialize = "1.0"))]
    Alder,
    #[serde(rename(deserialize = "2.0"))]
    Birch,
    #[serde(rename(deserialize = "3.0"))]
    Grass,
    #[serde(rename(deserialize = "4.0"))]
    Olive,
    #[serde(rename(deserialize = "5.0"))]
    Mugwort,
    #[serde(rename(deserialize = "6.0"))]
    Ragweed,
}

#[derive(Debug, Deserialize)]
struct PollenForecast {
    time: DateTime<Utc>,
    #[serde(rename = "latitude[unit=\"degrees_north\"]")]
    latitude: f32,
    #[serde(rename = "longitude[unit=\"degrees_east\"]")]
    longitude: f32,
    #[serde(rename = "POLI[unit=\"\"]")]
    poli: PollenIndex,
    #[serde(rename = "POLISRC[unit=\"\"]")]
    polisrc: PollenType,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let catalog = ThreddsCatalog::get().await?;

    let latitude = std::env::args().nth(1).expect("no latitude given");
    let longitude = std::env::args().nth(2).expect("no longitude given");

    let url = format!(
        "https://silam.fmi.fi/thredds/ncss/{url_path}?var=POLI&var=POLISRC&latitude={latitude}&longitude={longitude}&time_start={time_start}&time_end={time_end}&vertCoord=12.5&accept=csv",
        latitude = latitude,
        longitude = longitude,
        url_path = catalog.get_latest_url(),
        time_start = catalog.get_latest_start().to_rfc3339_opts(SecondsFormat::Secs, true),
        time_end = catalog.get_latest_end().to_rfc3339_opts(SecondsFormat::Secs, true),
    );

    let res = reqwest::get(&url).await?;

    let body = res.text().await?;

    let mut reader = csv::Reader::from_reader(body.as_bytes());

    for result in reader.deserialize() {
        let record: PollenForecast = result?;
        println!("{:?}", record);
    }

    Ok(())
}
