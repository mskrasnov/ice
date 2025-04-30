//! Main functions for work with OpenWeatherMap API, some traits

pub mod current;
pub mod daily;
pub mod geocoding;

use std::fmt::Display;

use anyhow::{Result, anyhow};
use chrono::{DateTime, Local, Timelike};
use serde::{Deserialize, Serialize};

/// Format url into GET/POST-request with given items
pub fn format_url<U, K, V>(url: U, items: impl Into<Vec<(K, V)>>) -> String
where
    U: ToString + Display,
    K: Display,
    V: Display,
{
    let mut url = format!("{url}");
    let items = items.into(); // convert items into Vec<(K, V)>

    for item in items {
        url = format!("{url}{}={}&", item.0, item.1);
    }
    url.pop(); // remove last `&` symbol

    url
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct Location {
    pub lat: f32,
    pub lon: f32,
}

impl Location {
    pub fn from_geo(geo: geocoding::LocationInfo) -> Self {
        Self {
            lat: geo.lat,
            lon: geo.lon,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Main {
    pub feels_like: f32,
    pub pressure: i32,
    pub temp: f32,
    pub temp_max: f32,
    pub temp_min: f32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WeatherMeta {
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

    pub fn get_icon(&self, time: DateTime<Local>) -> &str {
        let time = Time::get(time);
        match self.id {
            200..=232 => match time {
                Time::Day | Time::Evening => "thunder_day",
                Time::Night => "thunder_night",
            },
            300..=314 | 501..=531 => "rain",
            500 => "rain-500",
            600..=623 => "snow",
            701 | 741 | 711 | 721 => "fog",
            731 | 771 => match time {
                Time::Day | Time::Evening => "wind_day",
                Time::Night => "wind_night",
            },
            800 => match time {
                Time::Day => "clear_day",
                Time::Evening => "clear_evening",
                Time::Night => "clear_night",
            },
            801..=803 => match time {
                Time::Day | Time::Evening => "cloud_day-801",
                Time::Night => "cloud_night-801",
            },
            804 => match time {
                Time::Day | Time::Evening => "cloud_day",
                Time::Night => "cloud_night",
            },
            _ => "default",
        }
    }
}

#[derive(Clone, Copy)]
enum Time {
    Evening,
    Day,
    Night,
}

impl Time {
    fn get(time: DateTime<Local>) -> Self {
        let h = time.hour();
        if h >= 9 && h < 20 {
            Self::Day
        } else if h < 9 && h > 5 {
            Self::Evening
        } else {
            Self::Night
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

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Wind {
    pub deg: u16,
    pub gust: Option<f32>,
    pub speed: f32,
}

/// Execution of GET requests to API
pub trait API {
    /// Function for making a GET requests to OpenWeatherMap API
    ///
    /// `url`: the base URL of the request, `url_items`: additional
    /// parameters of the request. For example, `url` =
    /// `https://www.a.ru/penis?`, and `url_items`:
    /// `[("a", "A"), ("b", "B")]`, this will be expanded into the URL
    /// `https://www.a.ru/penis?a=A&b=B`.
    ///
    /// > **Note** that you must explicitly specify the
    /// > `("appid", "YOUR API KEY")` pair in `url_items`.
    fn get_request<'a, U, I>(
        url: U,
        url_items: I,
    ) -> impl std::future::Future<Output = Result<Self>> + Send
    where
        U: ToString + Display + Send,
        I: Into<Vec<(String, String)>> + Send,
        for<'de> Self: Deserialize<'de>,
    {
        async {
            let query = reqwest::get(format_url(url, url_items))
            .await
            .map_err(|err| anyhow!("Ошибка получения данных с сервера. Проверьте подключение к сети и корректность запроса ({err})"))?
            .json::<Self>()
            .await
            .map_err(|err| anyhow!("Ошибка получения JSON с сервера ({err})"))?;

            Ok(query)
        }
    }
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
