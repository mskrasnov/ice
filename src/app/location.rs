//! Autodetect user location and get (lat, lon) coordinates

use anyhow::Result;
use reqwest;
use serde::Deserialize;

const API_URL: &str = "http://ip-api.com/json/?fields=country,city,regionName,lat,lon";

#[derive(Debug, Deserialize, Clone)]
pub struct Location {
    pub city: Option<String>,
    pub country: Option<String>,

    #[serde(rename = "regionName")]
    pub region: Option<String>,

    pub lat: f32,
    pub lon: f32,
}

impl Location {
    pub async fn get_by_ip() -> Result<Self> {
        let response = reqwest::get(API_URL).await?.json::<Self>().await?;
        Ok(response)
    }
}
