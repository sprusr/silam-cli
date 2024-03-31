use chrono::{DateTime, Days, SecondsFormat, Timelike, Utc};
use serde::Deserialize;

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
    let latitude = std::env::args().nth(1).expect("no latitude given");
    let longitude = std::env::args().nth(2).expect("no longitude given");

    let url = format!(
        "https://silam.fmi.fi/thredds/ncss/silam_europe_pollen_v5_9/runs/{run_name}?var=POLI&var=POLISRC&latitude={latitude}&longitude={longitude}&time_start={time_start}&time_end={time_end}&vertCoord=12.5&accept=csv",
        latitude = latitude,
        longitude = longitude,
        run_name = format!(
            "silam_europe_pollen_v5_9_RUN_{timestamp}",
            timestamp = Utc::now().with_hour(0).unwrap().with_minute(0).unwrap().with_second(0).unwrap().to_rfc3339_opts(SecondsFormat::Secs, true),
        ),
        time_start = Utc::now().with_hour(1).unwrap().with_minute(0).unwrap().with_second(0).unwrap().to_rfc3339_opts(SecondsFormat::Secs, true),
        time_end = Utc::now().with_hour(0).unwrap().with_minute(0).unwrap().with_second(0).unwrap().checked_add_days(Days::new(5)).unwrap().to_rfc3339_opts(SecondsFormat::Secs, true),
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
