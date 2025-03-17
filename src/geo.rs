//! Convert location name into coordinates. Using geocoding API

use std::fmt::Display;

use anyhow::Result;
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
