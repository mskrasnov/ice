//! User interface based on `iced`

pub mod modal;
pub mod styles;
pub mod widgets;

pub mod current;
pub mod daily;

pub mod network;
pub mod settings;
pub mod weather;

pub mod update;
pub mod view;

use std::{fmt::Debug, time::Duration};

use chrono::{DateTime, Timelike};
use iced::{
    Alignment::Center,
    Element, Subscription, Task, Theme, time,
    widget::{button, center, column, container, horizontal_space, image, row, scrollable, text},
};

use crate::{
    api::{current::Current, daily::Daily, floor, geocoding},
    app::location,
    config::Config,
    consts::CONF_PATH,
    units::Variant,
};

pub fn ui() -> iced::Result {
    iced::application("Ice", Ice::update, Ice::view)
        .window_size((800., 480.))
        .antialiasing(true)
        .decorations(false)
        .subscription(Ice::subscription)
        .theme(Ice::theme)
        .run()
}

pub struct Ice {
    conf: Config,
    error_text: Option<String>,

    current_weather: Option<Current>,
    daily_weather: Option<Daily>,
    geocoding: Option<geocoding::Location>,
    autodetected_location: Option<location::Location>,
    selected_location: Option<geocoding::LocationInfo>,

    uptime: u32,
    ctime: chrono::DateTime<chrono::Local>,
}

#[derive(Debug, Clone)]
pub enum Message {
    /*****************
     * Async actions *
     *****************/
    AutoDetectLocation,
    LocationReceived((Option<location::Location>, Option<String>)),

    GetCurrentWeather,
    CurrentWeatherReceived((Option<Current>, Option<String>)),

    /************************
     * Some service actions *
     ************************/
    UpdateCTime,
    UpdateUptime,

    /*****************
     * Button clicks *
     *****************/
    RefreshButtonPressed,
}

impl Default for Ice {
    fn default() -> Self {
        Self {
            conf: Config::read(CONF_PATH).unwrap_or_default(),
            error_text: None,
            current_weather: None,
            daily_weather: None,
            geocoding: None,
            autodetected_location: None,
            selected_location: None,
            uptime: 0,
            ctime: chrono::offset::Local::now(),
        }
    }
}

impl Ice {
    const TOP_PANEL_TEXT_SIZE: u16 = 25;
    const TEXT_SIZE: u16 = 20;

    pub fn theme(&self) -> Theme {
        let h = self.ctime.hour();
        if h >= 6 && h < 22 {
            Theme::GruvboxLight
        } else {
            Theme::GruvboxDark
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        let mut scripts = vec![
            time::every(Duration::from_millis(500)).map(|_| Message::UpdateCTime),
            time::every(Duration::from_secs(1)).map(|_| Message::UpdateUptime),
        ];

        if self.autodetected_location.is_none() {
            scripts
                .push(time::every(Duration::from_millis(500)).map(|_| Message::AutoDetectLocation));
        }

        if self.current_weather.is_none() {
            scripts.push(time::every(Duration::from_secs(1)).map(|_| Message::GetCurrentWeather));
        }

        Subscription::batch(scripts)
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            /*****************
             * Async actions *
             *****************/
            Message::AutoDetectLocation => Task::perform(
                async move {
                    let loc = location::Location::get_by_ip().await;
                    match loc {
                        Ok(loc) => (Some(loc), None),
                        Err(why) => (None, Some(why.to_string())),
                    }
                },
                |val| Message::LocationReceived(val),
            ),
            Message::LocationReceived(loc) => {
                (self.autodetected_location, self.error_text) = loc;
                Task::none()
            }

            Message::GetCurrentWeather => {
                let appid = self.conf.appid.clone();
                let units = self.conf.units;
                let location = self.autodetected_location.clone();

                Task::perform(
                    async move {
                        if let Some(loc) = location {
                            let current = Current::get(
                                appid,
                                crate::api::Location {
                                    lat: loc.lat,
                                    lon: loc.lon,
                                },
                                units,
                            )
                            .await;

                            match current {
                                Ok(current) => (Some(current), None),
                                Err(why) => (None, Some(why.to_string())),
                            }
                        } else {
                            (None, Some("Неизвестное местоположение".to_string()))
                        }
                    },
                    |val| Message::CurrentWeatherReceived(val),
                )
            }
            Message::CurrentWeatherReceived(current) => {
                (self.current_weather, self.error_text) = current;
                Task::none()
            }

            /************************
             * Some service actions *
             ************************/
            Message::UpdateCTime => {
                self.ctime = chrono::offset::Local::now();
                Task::none()
            }
            Message::UpdateUptime => {
                self.uptime += 1;
                Task::none()
            }
            _ => Task::none(),
        }
    }

    pub fn view(&self) -> Element<Message> {
        let image = container(center(
            column![
                text(match &self.current_weather {
                    Some(current) => current.weather[0].get_descr(),
                    None => "Загружаем инф-цию...",
                })
                .size(Self::TEXT_SIZE),
                image(format!(
                    "./res/icons/{}.png",
                    match &self.current_weather {
                        Some(current) => current.weather[0].get_icon(self.ctime),
                        None => "default",
                    }
                )),
                text(format!(
                    "{}{}",
                    floor(match &self.current_weather {
                        Some(current) => current.main.feels_like,
                        None => 0.,
                    }),
                    Variant::Degrees.to_str(self.conf.units),
                ))
                .size(35)
            ]
            .spacing(10)
            .align_x(Center),
        ))
        .padding(10)
        .width(260);

        let top_panel = row![
            button(text("Обновить").size(Self::TOP_PANEL_TEXT_SIZE))
                .on_press(Message::RefreshButtonPressed),
            text(match &self.current_weather {
                Some(current) => format!("{} ({})", &current.name, &current.sys.country,),
                None => "Загружаем информацию...".to_string(),
            })
            .size(Self::TOP_PANEL_TEXT_SIZE),
            horizontal_space(),
            container(
                text(
                    crate::time::Time::new(&self.ctime)
                        .set_display_mode(crate::time::DisplayMode::TimeDate)
                        .to_string()
                )
                .size(Self::TOP_PANEL_TEXT_SIZE),
            )
            .padding(5)
            .style(container::rounded_box),
        ]
        .align_y(Center)
        .spacing(10)
        .padding(10);

        container(column![
            top_panel,
            row![
                image,
                center(scrollable(
                    text("Тут должен быть почасовой прогноз").size(25),
                ))
            ]
            .spacing(Self::TEXT_SIZE)
            .align_y(Center),
            row![
                text("Информация предоставлена OpenWeatherMap").size(12),
                horizontal_space(),
                text(format!(
                    "Время работы: {}",
                    crate::time::Time::new(
                        &DateTime::from_timestamp(self.uptime.into(), 0).unwrap()
                    )
                    .set_display_mode(crate::time::DisplayMode::TimeWithSeconds)
                ))
                .size(12),
            ]
            .padding(10),
        ])
        .into()
    }
}
