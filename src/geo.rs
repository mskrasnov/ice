//! Convert location name into coordinates. Using geocoding API

use std::fmt::Display;

use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::conf::Location;
use crate::weather::format_url;

/// The URL that is used to make GET requests to the API
pub const API_URL: &str = "http://api.openweathermap.org/geo/1.0/direct?";
pub const LIMIT: &str = "5";

#[derive(Debug, Clone)]
pub struct LocationName {
    city: String,
    state: Option<String>,
    country: Option<String>,
}

impl LocationName {
    pub fn new<C: ToString>(city: C) -> Self {
        Self {
            state: None,
            country: None,
            city: city.to_string(),
        }
    }

    pub fn set_state<S: ToString>(mut self, state: S) -> Self {
        self.state = Some(state.to_string());
        self
    }

    pub fn set_country<C: ToString>(mut self, country: C) -> Self {
        self.country = Some(country.to_string());
        self
    }

    pub fn from_str(name: &str) -> Option<Self> {
        let chunks = name.split(',').collect::<Vec<_>>();
        let len = chunks.len();

        match len {
            1 => Some(Self {
                city: chunks[0].to_string(),
                state: None,
                country: None,
            }),
            2 => Some(Self {
                city: chunks[0].to_string(),
                state: Some(chunks[1].to_string()),
                country: None,
            }),
            3 => Some(Self {
                city: chunks[0].to_string(),
                state: Some(chunks[1].to_string()),
                country: Some(chunks[2].to_string()),
            }),
            _ => None,
        }
    }
}

impl Display for LocationName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut location = format!("{}", &self.city);

        if let Some(state) = &self.state {
            location = format!("{location},{state}");
        }
        if let Some(country) = &self.country {
            location = format!("{location},{country}");
        }

        write!(f, "{}", location)
    }
}

impl LocationName {
    /// Performs the GET-request to OWM
    pub async fn get(&self, api_key: &str) -> Result<Value> {
        let query = reqwest::get(format_url(
            API_URL,
            [
                ("q", format!("{self}")),
                ("limit", LIMIT.to_string()),
                ("appid", api_key.to_string()),
            ],
        ))
        .await?
        .json::<Value>()
        .await?;

        Ok(query)
    }
}

impl Location {
    pub fn from_json_value(value: Value) -> Result<Self> {
        let data = serde_json::from_value(value)?;

        Ok(data)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LocationData(pub Vec<LocationInfo>);

impl LocationData {
    pub fn from_json_value(value: Value) -> Result<Self> {
        let data = serde_json::from_value(value)?;

        Ok(data)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LocationInfo {
    pub name: String,
    pub local_names: Option<LocalNames>,
    pub lat: f32,
    pub lon: f32,
    pub country: String,
    pub state: Option<String>,
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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LocalNames {
    pub bg: Option<String>,
    pub de: Option<String>,
    pub en: Option<String>,
    pub ru: Option<String>,
}
