mod pollen_forecast;
mod thredds_catalog;

use pollen_forecast::PollenForecast;
use thredds_catalog::ThreddsCatalog;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let latitude = std::env::args().nth(1).expect("no latitude given");
    let longitude = std::env::args().nth(2).expect("no longitude given");

    let catalog = ThreddsCatalog::get().await?;

    let forecast = PollenForecast::get(
        catalog.get_latest_url(),
        &latitude,
        &longitude,
        catalog.get_latest_start(),
        catalog.get_latest_end(),
    )
    .await?;

    let json = serde_json::to_string(&forecast)?;

    println!("{}", json);

    Ok(())
}
