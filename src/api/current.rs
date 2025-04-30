//! Get current weather forecast

use anyhow::Result;
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use super::{API, Location, Main, WeatherMeta, Wind};
use crate::config::Units;

/// The URL that is used to make GET requests to the API
pub const API_URL: &str = "https://api.openweathermap.org/data/2.5/weather?";

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Current {
    // pub base: String,
    pub name: String,
    pub coord: Location,
    pub main: Main,
    pub sys: Sys,
    pub dt: i64,
    pub timezone: i32,
    pub weather: Vec<WeatherMeta>,
    pub wind: Wind,
}

impl API for Current {}

impl Current {
    pub async fn get<A: ToString>(appid: A, loc: Location, units: Units) -> Result<Self> {
        Self::get_request(
            API_URL,
            [
                ("appid".to_string(), appid.to_string()),
                ("units".to_string(), units.to_string()),
                ("lat".to_string(), loc.lat.to_string()),
                ("lon".to_string(), loc.lon.to_string()),
            ],
        )
        .await
    }

    pub fn get_time(&self) -> DateTime<FixedOffset> {
        let time = DateTime::from_timestamp(self.dt, 0)
            .unwrap_or_default()
            .with_timezone(&FixedOffset::east_opt(self.timezone).unwrap());
        time
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Sys {
    pub country: String,
    pub sunrise: i64,
    pub sunset: i64,
}
