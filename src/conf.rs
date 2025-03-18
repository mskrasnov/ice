//! Configuration parser for Ice

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};
use toml;

/// Structure of the program configuration file
#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct Conf {
    /// Units in which data output will be performed
    pub units: Units,

    /// API Key for OWM
    pub api_key: String,

    /// Location from which weather information is to be obtained
    pub location: Location,
}

/// Units in which data output will be performed
#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub enum Units {
    #[default]
    #[serde(rename = "metric")]
    Metric,

    #[serde(rename = "imperial")]
    Imperial,
}

impl ToString for Units {
    fn to_string(&self) -> String {
        match self {
            Self::Metric => "metric",
            Self::Imperial => "imperial",
        }
        .to_string()
    }
}

/// Location from which weather information is to be obtained
#[derive(Debug, Deserialize, Serialize, Default, Clone, Copy, PartialEq)]
pub struct Location {
    /// Latitude
    pub lat: f32,

    /// Longitude
    pub lon: f32,
}

impl ToString for Location {
    fn to_string(&self) -> String {
        format!("{},{}", self.lat, self.lon)
    }
}

impl Conf {
    pub fn parse<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(&path)?;
        let data = toml::from_str(&content)?;

        Ok(data)
    }

    pub fn write<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let content = toml::to_string(&self)?;
        fs::write(&path, content)?;

        Ok(())
    }
}
