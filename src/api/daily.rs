//! 5 day weather forecast

use anyhow::Result;
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

use super::{API, Location, Main, WeatherMeta, Wind};
use crate::units::Units;

/// The URL that is used to make GET requests to the API
pub const API_URL: &str = "https://api.openweathermap.org/data/2.5/forecast?";

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Daily {
    pub cnt: usize,
    pub list: Vec<WeatherData>,
    pub city: City,
}

impl API for Daily {}

impl Daily {
    pub async fn get(appid: &str, loc: Location, units: Units) -> Result<Self> {
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

    /// Gets the time and date of the specified weather measurement moment
    ///
    /// `idx` - number of the measured torque (index in the array)
    pub fn get_time(&self, idx: usize) -> DateTime<FixedOffset> {
        let time = DateTime::from_timestamp(self.list[idx].dt, 0)
            .unwrap_or_default()
            .with_timezone(&FixedOffset::east_opt(self.city.timezone).unwrap());
        time
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WeatherData {
    pub dt: i64,
    pub main: Main,
    pub weather: Vec<WeatherMeta>,
    pub wind: Wind,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct City {
    pub id: u64,
    pub name: String,
    pub coord: Location,
    pub country: String,
    pub timezone: i32,
    pub sunrise: i64,
    pub sunset: i64,
}
