//! Module for fetch weather information from OpenWeatherMap

use std::fmt::Display;

use anyhow::{Result, anyhow};
use chrono::{DateTime, FixedOffset, Timelike};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::conf::{Location, Units};

/// The URL that is used to make GET requests to the API
pub const API_URL: &str = "https://api.openweathermap.org/data/2.5/weather?";

/// Format url into GET/POST-request with given items
pub fn format_url<U, K, V>(url: U, items: impl Into<Vec<(K, V)>>) -> String
where
    U: ToString + Display,
    K: ToString + Display,
    V: ToString + Display,
{
    let mut url = format!("{url}");
    let items = items.into(); // convert items into Vec<(K, V)>

    for item in items {
        url = format!("{url}{}={}&", item.0, item.1);
    }
    url.pop(); // remove last `&` symbol

    url
}

/// Converts the time value in UNIX Timestamp and time zone format to a
/// convenient format for internal representation in the program
pub fn get_time(time: i64, timezone: i32) -> DateTime<FixedOffset> {
    let date = DateTime::from_timestamp(time, 0)
        .unwrap_or_default()
        // WARN: Maybe this unwrap needs to be replaced with something else.
        //                                                       vvvvvvvvv
        .with_timezone(&FixedOffset::east_opt(timezone).unwrap());

    date
}

pub struct Weather<'a> {
    pub location: Location,
    pub units: Units,
    pub api_key: &'a str,
}

impl<'a> Weather<'a> {
    /// Creates new instance of `Weather<'a>`
    pub fn new(loc: Location, key: &'a str) -> Self {
        Self {
            location: loc,
            api_key: key,
            units: Units::default(),
        }
    }

    pub fn set_units(mut self, units: Units) -> Self {
        self.units = units;
        self
    }

    /// Performs the GET-request to OWM
    pub async fn get(&self) -> Result<Value> {
        let query = reqwest::get(format_url(
            API_URL,
            [
                ("appid", self.api_key.to_string()),
                ("units", self.units.to_string()),
                ("lat", self.location.lat.to_string()),
                ("lon", self.location.lon.to_string()),
            ],
        ))
        .await
        .map_err(|_| anyhow!("Failed to perform GET-request"))?
        .json::<Value>()
        .await
        .map_err(|err| anyhow!("Failed to get JSON object:\n{err}"))?;

        Ok(query)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WeatherData {
    // NOTE: items below was commented because it unneeded
    // pub base: String,
    // pub clouds: Clouds,
    // pub cod: usize,
    // pub coord: Location,
    pub dt: i64,
    // pub id: i32,
    pub main: Main,
    // pub name: String,
    pub sys: Sys,
    pub timezone: i32,
    // pub visibility: u32,
    pub weather: Vec<WeatherMeta>,
    pub wind: Wind,
}

impl WeatherData {
    pub fn from_json_value(value: Value) -> Result<Self> {
        let data = serde_json::from_value(value)?;

        Ok(data)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Clouds {
    pub all: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Main {
    pub feels_like: f32,
    // pub grnd_level: i32,
    // pub humidity: u8,
    pub pressure: i32,
    // pub sea_level: i32,
    pub temp: f32,
    pub temp_max: f32,
    pub temp_min: f32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Sys {
    pub country: String,
    // pub id: u32,
    pub sunrise: i64,
    pub sunset: i64,
    // #[serde(rename = "type")]
    // pub _type: u8,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WeatherMeta {
    // pub description: String,
    // pub icon: String,
    pub id: u16,
    pub main: WeatherDescription,
}

impl WeatherMeta {
    pub fn get_descr(&self) -> &str {
        match self.id {
            // Group 2xx: Thunderstorm
            200 => "гроза с небольшим дождём",
            201 => "гроза с дождём",
            202 => "гроза с сильным дождём",
            210 => "небольшая гроза",
            211 => "гроза",
            212 => "сильная гроза",
            221 => "рваная гроза",
            230 => "гроза с лёгким моросящим дождём",
            231 => "гроза с моросящим дождём",
            232 => "гроза с сильным моросящим дождём",

            // Group 3xx: Drizzle
            300 | 310 => "небольшой моросящий дождь",
            301 | 311 | 321 => "моросящий дождь",
            302 | 312 => "сильный моросящий дождь",
            313 => "дождь и морось",
            314 => "сильный моросящий ливень",

            // Group 5xx: Rain
            500 => "дождь",
            501 => "умеренный дождь",
            502 => "сильный дождь",
            503 => "очень сильный дождь",
            504 | 522 => "сильный ливень",
            511 => "ледяной дождь",
            520 => "небольшой ливень",
            521 => "ливень",
            531 => "неровный дождь",

            // Group 6xx: Snow
            600 => "небольшой снег",
            601 => "снег",
            602 => "сильный снег",
            611 => "дождь со снегом",
            612 => "лёгкий дождь со снегом",
            613..623 => "снег с дождём",

            // Group 7xx: Atmosphere
            701 | 741 => "туман",
            711 => "дым",
            721 => "дымка",
            731 => "вихри песка/пыли",
            751 => "песок",
            761 => "пыль",
            762 => "вулканический пепел",
            771 => "шторм",
            781 => "торнадо",

            // Group 800: Clear
            800 => "ясно",

            // Group 80x: Clouds
            801 => "небольшая облачность",
            802 => "рассеянные облака",
            803 => "средняя облачность",
            804 => "пасмурно",

            // Other
            _ => "погода неизвестна",
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum WeatherDescription {
    Thunderstorm,
    Drizzle,
    Rain,
    Snow,
    Mist,
    Smoke,
    Haze,
    Dust,
    Fog,
    Sand,
    Ash,
    Squall,
    Tornado,
    Clear,
    Clouds,
}

pub struct WeatherIcon<'a> {
    pub day: &'a str,
    pub evening: Option<&'a str>,
    pub night: Option<&'a str>,
}

impl<'a> WeatherIcon<'a> {
    pub fn get_icon_name(&self, dt: &DateTime<FixedOffset>) -> &str {
        let hour = dt.hour();
        if hour >= 8 && hour <= 17 {
            self.day
        } else if hour > 17 && hour <= 22 {
            match self.evening {
                Some(evening) => evening,
                None => self.day,
            }
        } else {
            match self.night {
                Some(night) => night,
                None => self.day,
            }
        }
    }
}

impl<'a> Default for WeatherIcon<'a> {
    fn default() -> Self {
        Self {
            day: "default.png",
            evening: Some("default.png"),
            night: Some("default.png"),
        }
    }
}

impl WeatherDescription {
    pub fn get_icon<'a>(&self) -> Option<WeatherIcon<'a>> {
        match self {
            Self::Rain => Some(WeatherIcon {
                day: "rain.png",
                // If items below is None, using the `day` param
                evening: None,
                night: None,
            }),
            Self::Clear => Some(WeatherIcon {
                day: "clear_day.png",
                evening: Some("clear_evening.png"),
                night: Some("clear_night.png"),
            }),
            Self::Clouds => Some(WeatherIcon {
                day: "cloud_day.png",
                evening: None,
                night: Some("cloud_night.png"),
            }),
            Self::Fog => Some(WeatherIcon {
                day: "fog.png",
                evening: None,
                night: None,
            }),
            Self::Snow => Some(WeatherIcon {
                day: "snow.png",
                evening: None,
                night: None,
            }),
            Self::Thunderstorm => Some(WeatherIcon {
                day: "thunder_day.png",
                evening: None,
                night: Some("thunder_night.png"),
            }),
            _ => None, // if None, using the default icon
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Wind {
    pub deg: u16,
    pub gust: Option<f32>,
    pub speed: f32,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn format_url_test() {
        let url = "https://pornhub.com?";
        let items = vec![("aa", "bb"), ("aaa", "bbb")];
        let fmt_url = format_url(&url, items);

        assert_eq!(fmt_url, "https://pornhub.com?aa=bb&aaa=bbb");
    }
}
