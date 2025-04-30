//! Get coordinates of given location

use anyhow::Result;
use serde::{Deserialize, Serialize};

use super::API;

/// The URL that is used to make GET requests to the API
pub const API_URL: &str = "http://api.openweathermap.org/geo/1.0/direct?";

/// Max items in the API response
pub const LIMIT: &str = "5";

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Location(pub Vec<LocationInfo>);

impl API for Location {}

impl Location {
    pub async fn get(location: &str, appid: &str) -> Result<Self> {
        Location::get_request(
            API_URL,
            [
                ("q".to_string(), location.to_string()),
                ("limit".to_string(), LIMIT.to_string()),
                ("appid".to_string(), appid.to_string()),
            ],
        )
        .await
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LocationInfo {
    pub name: String,
    pub country: String,
    pub state: Option<String>,
    pub local_names: Option<LocalNames>,
    pub lat: f32,
    pub lon: f32,
}

impl ToString for LocationInfo {
    fn to_string(&self) -> String {
        if let Some(state) = &self.state {
            format!("{} ({}, {})", &self.name, &state, &self.country)
        } else {
            format!("{} ({})", &self.name, &self.country)
        }
    }
}

impl PartialEq for LocationInfo {
    fn eq(&self, other: &Self) -> bool {
        &self.name == &other.name && &self.country == &other.country && &self.state == &other.state
    }

    fn ne(&self, other: &Self) -> bool {
        &self.name != &other.name || &self.country != &other.country || &self.state != &other.state
    }
}

impl Eq for LocationInfo {}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct LocalNames {
    pub bg: Option<String>,
    pub de: Option<String>,
    pub en: Option<String>,
    pub ru: Option<String>,
}
