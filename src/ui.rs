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

use chrono::Timelike;
use iced::{
    Alignment::Center,
    Element, Subscription, Task, Theme, time,
    widget::{button, center, column, container, horizontal_space, image, row, scrollable, text},
};

use crate::{
    api::{current::Current, daily::Daily, geocoding},
    app::location,
    config::Config,
    consts::CONF_PATH,
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

    pub fn theme(&self) -> Theme {
        Theme::GruvboxDark
    }

    pub fn subscription(&self) -> Subscription<Message> {
        let mut scripts =
            vec![time::every(Duration::from_millis(500)).map(|_| Message::UpdateCTime)];

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
                        let loc = location.unwrap();
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
            _ => Task::none(),
        }
    }

    pub fn view(&self) -> Element<Message> {
        let top_panel = row![
            button(text("Обновить").size(Self::TOP_PANEL_TEXT_SIZE))
                .on_press(Message::RefreshButtonPressed),
            text("Dzerzhinsk, Russia").size(Self::TOP_PANEL_TEXT_SIZE),
            horizontal_space(),
            container(
                text(format!(
                    "{}:{}:{}{}",
                    self.ctime.hour(),
                    self.ctime.minute(),
                    if self.ctime.second() < 10 { "0" } else { "" },
                    self.ctime.second(),
                ))
                .size(Self::TOP_PANEL_TEXT_SIZE),
            )
            .padding(5)
            .style(container::rounded_box),
        ]
        .align_y(Center)
        .spacing(10);

        let image = image(format!(
            "./res/icons/{}.png",
            match &self.current_weather {
                Some(current) => current.weather[0].get_icon(self.ctime),
                None => "default",
            }
        ));

        container(column![
            top_panel,
            row![
                image,
                center(scrollable(
                    text(format!("{:#?}", self.current_weather)).size(35)
                ))
            ]
            .spacing(20)
            .align_y(Center)
        ])
        .padding([10, 10])
        .into()
    }
}
