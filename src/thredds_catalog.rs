use chrono::{DateTime, Utc};
use quick_xml::de::from_str;
use serde::Deserialize;

const SILAM_EUROPE_POLLEN_CATALOG: &str =
    "https://silam.fmi.fi/thredds/catalog/silam_europe_pollen_v5_9/runs/catalog.xml";

#[derive(Debug, Deserialize)]
pub struct ThreddsCatalog {
    dataset: RootDataset,
}

#[derive(Debug, Deserialize)]
struct RootDataset {
    dataset: Vec<Dataset>,
}

#[derive(Debug, Deserialize)]
struct Dataset {
    #[serde(rename = "@urlPath")]
    url_path: String,
    #[serde(rename = "timeCoverage")]
    time_coverage: TimeCoverage,
}

#[derive(Debug, Deserialize)]
struct TimeCoverage {
    start: DateTime<Utc>,
    end: DateTime<Utc>,
}

impl ThreddsCatalog {
    pub async fn get() -> Result<ThreddsCatalog, Box<dyn std::error::Error>> {
        let res = reqwest::get(SILAM_EUROPE_POLLEN_CATALOG).await?;
        let body = res.text().await?;
        let catalog: ThreddsCatalog = from_str(&body).unwrap();
        Ok(catalog)
    }

    pub fn get_latest_url(&self) -> &String {
        &self.dataset.dataset.first().unwrap().url_path
    }

    pub fn get_latest_start(&self) -> &DateTime<Utc> {
        &self.dataset.dataset.first().unwrap().time_coverage.start
    }

    pub fn get_latest_end(&self) -> &DateTime<Utc> {
        &self.dataset.dataset.first().unwrap().time_coverage.end
    }
}
