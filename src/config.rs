//! Configuration file of the Ice

use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};
use toml;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub coords: Option<(f32, f32)>,
    pub units: Units,
    pub appid: String,
    pub autodetect_location: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            coords: Some((56.3287, 44.002)),
            units: Units::default(),
            appid: "26896f0fe821b98790eeae3a316f3358".to_string(),
            autodetect_location: true,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, Copy)]
pub enum Units {
    #[serde(rename = "imperial")]
    Imperial,

    #[default]
    #[serde(rename = "metric")]
    Metric,
}

impl ToString for Units {
    fn to_string(&self) -> String {
        match self {
            Self::Imperial => "imperial",
            Self::Metric => "metric",
        }
        .to_string()
    }
}

impl Config {
    pub fn read<P: AsRef<Path>>(path: P) -> Result<Self> {
        let contents = fs::read_to_string(&path).map_err(|err| {
            anyhow!("Ошибка чтения конфига ({err}). Проверьте его наличие и доступ к нему")
        })?;
        let data = toml::from_str(&contents).map_err(|err| {
            anyhow!("Ошибка парсинга конфига. Проверьте его корректность.\n\n{err}")
        })?;
        Ok(data)
    }

    pub fn write<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let data = toml::to_string(&self).map_err(|err| {
            anyhow!("Ошибка сериализации конфига. Проверьте корректность данных.\n\n{err}")
        })?;
        fs::write(path, data).map_err(|err| {
            anyhow!("Ошибка записи конфига ({err}). Проверьте доступ к нему в ФС.")
        })?;

        Ok(())
    }
}
