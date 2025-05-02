//! Printable units

use serde::{Deserialize, Serialize};

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

pub enum Variant {
    Degrees, // градусы (температура)
    Speed,   // скорость (ветра)
}

impl Variant {
    pub fn to_str(&self, units: Units) -> &str {
        match self {
            Self::Degrees => match units {
                Units::Imperial => "°F",
                Units::Metric => "°C",
            },
            Self::Speed => match units {
                Units::Imperial => "km/h",
                Units::Metric => "m/s",
            },
        }
    }
}
