//! User interface based on [iced](https://iced.rs)

pub mod modal;
pub mod widgets;

use std::time::Duration;

use iced::Alignment::Center;
use iced::keyboard::key;
use iced::widget::{
    PickList, button, center, column, container, horizontal_rule, horizontal_space, image, row,
    text, text_input, vertical_space,
};
use iced::{
    Color, Element, Event, Length, Pixels, Subscription, Task, Theme, event, keyboard, time,
};

use crate::conf::{Conf, Location};
use crate::geo::{LocationData, LocationInfo, LocationName};
use crate::sys;
use crate::time::Time;
use crate::weather::{Weather, WeatherData, get_time};

use widgets::*;

/// Default path to the configuration file of Ice
const CONF_PATH: &str = "./assets/ice.toml";

/**************************************************************************
 *                         Window parameters                              *
 **************************************************************************/
const WIN_WIDTH: f32 = 800.;
const WIN_HEIGHT: f32 = 480.;
const WIN_ANTIALIASING: bool = true;
const WIN_NAME: &str = "ice environment";

/**************************************************************************
 *                           User interface                               *
 **************************************************************************/

/// The main window of the Ice app, which has a resolution of 800x480
/// (possibly changeable in the future), with no window decorations and no
/// ability to resize itself
struct Ice {
    /// Configuration of the Ice
    /// `Ice::default()` method parses the configuration file that defined in
    /// the `CONF_PATH` constant. If error return default value, if success -
    /// parsed data.
    conf: Conf,

    /// Location name in the human format. The location name is entered in
    /// the appropriate field, and then this value is used to convert to the
    /// coordinates of the desired location.
    location_str: String,

    /// Ice supports appearance themes. At the moment only standard themes
    /// are supported, but in the future I want to add my own themes that
    /// will switch automatically depending on weather conditions.
    theme: Theme,

    /// This is where the error text is written when performing any actions.
    /// Then, this text will be displayed in a special UI area, if the value
    /// of this field is not equal to `None`.
    is_err: Option<String>,

    /// Type of modal window. If none, no modal windows have displayed
    modal: Option<ModalWindow>,

    /// Information about weather
    weather_data: Option<WeatherData>,

    geocoding_data: Option<LocationData>,
    selected_location: Option<LocationInfo>,

    uptime: u32,
}

impl Default for Ice {
    fn default() -> Self {
        let mut is_err: Option<String> = None;
        let _conf = Conf::parse(CONF_PATH);

        // If _conf.is_err() = true change value of the is_err and return
        // default value of `Conf`
        let conf = match _conf {
            Ok(conf) => conf,
            Err(why) => {
                is_err = Some(why.to_string());
                Conf::default()
            }
        };

        Self {
            conf,
            is_err,
            location_str: String::new(),
            theme: Theme::Dark,
            modal: None,
            weather_data: None,
            geocoding_data: None,
            selected_location: None,
            uptime: 0,
        }
    }
}

/// Actions that the program can perform
#[derive(Debug, Clone)]
enum Message {
    /// The action that is performed when the user clicks on the location
    /// selection button
    LocationSelectorPressed,

    /// The action that is performed when the user starts typing the name
    /// of the location whose weather is to be displayed
    LocationNameEntered(String),

    /// Weather Update. It is performed when the user clicked the “Update
    /// data” button
    RefreshButtonPressed,

    /// First value: weather data, second value: error text
    WeatherDataReceived((Option<WeatherData>, Option<String>)),

    /// First value: location data, second: error text
    LocationDataReceived((Option<LocationData>, Option<String>)),
    LocationSelected(LocationInfo),

    SearchLocationPressed,

    APIKeyChanged(String),
    ThemeChanged(Theme),

    AboutButtonPressed,
    SettingsButtonPressed,
    PoweroffButtonPressed,

    RestartSystem,
    PoweroffSystem,
    ExitProgramm,

    /// Counts the program run time in seconds
    TickUptime,

    /// Some events (by subscription)
    Event(Event),
}

#[derive(Debug, Clone, Copy)]
enum ModalWindow {
    LocationSelector,
    About,
    Settings,
    PowerOff,
}

impl Ice {
    fn theme(&self) -> Theme {
        self.theme.clone()
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::batch(vec![
            event::listen().map(Message::Event),
            time::every(Duration::from_secs(1)).map(|_| Message::TickUptime),
            if self.weather_data.is_none() {
                // Wait 1 second and try to update weather data
                // May be used after startup
                time::every(Duration::from_secs(1)).map(|_| Message::RefreshButtonPressed)
            } else {
                // Update weather data every 10 min.
                time::every(Duration::from_secs(600)).map(|_| Message::RefreshButtonPressed)
            },
        ])
    }

    fn set_modal_win(&mut self, modal_win: ModalWindow) {
        self.modal = match self.modal {
            None => Some(modal_win),
            Some(_) => None,
        };
    }

    fn print_error_log(&self) {
        if let Some(err) = &self.is_err {
            eprintln!("ERROR: {err}");
        }
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        self.print_error_log();
        let conf = self.conf.clone();
        let location_str = self.location_str.clone();

        match message {
            Message::LocationSelectorPressed => {
                self.set_modal_win(ModalWindow::LocationSelector);
                iced::widget::focus_next()
            }
            Message::LocationNameEntered(location) => {
                self.location_str = location;
                Task::none()
            }
            Message::RefreshButtonPressed => Task::perform(
                async move {
                    let data = Weather::new(
                        Location {
                            lat: conf.location.lat,
                            lon: conf.location.lon,
                        },
                        &conf.api_key,
                    )
                    .set_units(crate::conf::Units::Metric)
                    .get()
                    .await;

                    match data {
                        Ok(value) => (Some(WeatherData::from_json_value(value).unwrap()), None),
                        Err(why) => (None, Some(why.to_string())),
                    }
                },
                |val| Message::WeatherDataReceived(val),
            ),
            Message::SearchLocationPressed => Task::perform(
                async move {
                    let data = LocationName::from_str(&location_str)
                        .unwrap_or(LocationName::new("Dzerzhinsk"))
                        .get(&conf.api_key)
                        .await;

                    match data {
                        Ok(value) => (Some(LocationData::from_json_value(value).unwrap()), None),
                        Err(why) => (None, Some(why.to_string())),
                    }
                },
                |val| Message::LocationDataReceived(val),
            ),
            Message::WeatherDataReceived(value) => {
                (self.weather_data, self.is_err) = value;

                Task::none()
            }
            Message::LocationDataReceived(value) => {
                (self.geocoding_data, self.is_err) = value;

                Task::none()
            }
            Message::LocationSelected(location) => {
                (self.conf.location.lat, self.conf.location.lon) = (location.lat, location.lon);
                self.selected_location = Some(location);

                // Cleanup
                self.location_str = String::new();
                self.geocoding_data = None;

                // Close this modal window
                self.set_modal_win(ModalWindow::LocationSelector);

                // Task::none()
                self.update(Message::RefreshButtonPressed) // needed for autorefresh weather data
            }
            Message::AboutButtonPressed => {
                self.set_modal_win(ModalWindow::About);
                Task::none()
            }
            Message::SettingsButtonPressed => {
                self.set_modal_win(ModalWindow::Settings);
                Task::none()
            }
            Message::PoweroffButtonPressed => {
                self.set_modal_win(ModalWindow::PowerOff);
                Task::none()
            }
            Message::RestartSystem => {
                if let Err(why) = sys::reboot() {
                    self.is_err = Some(format!("RestartSystem: {why}"));
                }
                Task::none()
            }
            Message::PoweroffSystem => {
                if let Err(why) = sys::poweroff() {
                    self.is_err = Some(format!("PowerOffSystem: {why}"));
                }
                Task::none()
            }
            Message::ExitProgramm => sys::exit_prog(),
            Message::TickUptime => {
                self.uptime += 1;
                Task::none()
            }
            Message::APIKeyChanged(key) => {
                self.conf.api_key = key;
                Task::none()
            }
            Message::ThemeChanged(theme) => {
                self.theme = theme;
                Task::none()
            }
            Message::Event(event) => match event {
                Event::Keyboard(keyboard::Event::KeyPressed {
                    key: keyboard::Key::Named(key::Named::F5),
                    ..
                }) => Task::perform(
                    async move {
                        let data = Weather::new(
                            Location {
                                lat: conf.location.lat,
                                lon: conf.location.lon,
                            },
                            &conf.api_key,
                        )
                        .set_units(crate::conf::Units::Metric)
                        .get()
                        .await;

                        match data {
                            Ok(value) => (Some(WeatherData::from_json_value(value).unwrap()), None),
                            Err(why) => (None, Some(why.to_string())),
                        }
                    },
                    |val| Message::WeatherDataReceived(val),
                ),
                _ => Task::none(),
            },
        }
    }

    fn poweroff<'a>(&self, base: impl Into<Element<'a, Message>>) -> Element<'a, Message> {
        let txt = text("Что вы хотите сделать?");
        let buttons = row![
            horizontal_space(),
            button("Отмена").on_press(Message::PoweroffButtonPressed),
            button("Завершение процесса")
                .on_press(Message::ExitProgramm)
                .style(button::success),
            button("Перезапуск")
                .on_press(Message::RestartSystem)
                .style(button::danger),
            button("Выключение")
                .on_press(Message::PoweroffSystem)
                .style(button::danger),
        ]
        .spacing(5);

        modal(
            base,
            column![txt, buttons]
                .spacing(5)
                .width((WIN_WIDTH / 1.5) as u16),
            Message::PoweroffButtonPressed,
        )
        .into()
    }

    fn settings(&self) -> Element<Message> {
        let theme_selector = PickList::new(Theme::ALL, Some(&self.theme), Message::ThemeChanged);
        let api_key_input = text_input("Введите ключ API сюда...", &self.conf.api_key)
            .on_input(Message::APIKeyChanged);

        column![
            row![text("Тема интерфейса:"), horizontal_space(), theme_selector,]
                .spacing(5)
                .align_y(Center),
            row![text("Ключ API:"), api_key_input,]
                .spacing(5)
                .align_y(Center),
        ]
        .width((WIN_WIDTH / 2.) as u16)
        .spacing(5)
        .into()
    }

    fn location_selector<'a>(&self) -> Element<'a, Message> {
        let input = row![
            text_input(
                "Введите ваше местоположение сюда и нажмите Enter...",
                &self.location_str
            )
            .on_submit(Message::SearchLocationPressed)
            .on_input(Message::LocationNameEntered),
            button("Поиск").on_press(Message::SearchLocationPressed),
        ]
        .spacing(5);

        let results = match &self.geocoding_data {
            Some(res) => {
                let results = PickList::new(
                    res.0.clone(),
                    self.selected_location.clone(),
                    Message::LocationSelected,
                );

                Some(
                    row![
                        text("Выберите местоположение:"),
                        horizontal_space(),
                        results,
                    ]
                    .spacing(5)
                    .align_y(Center),
                )
            }
            None => None,
        };

        let mut content = column![input].spacing(5);
        if let Some(results) = results {
            content = content.push(results);
        }

        container(content).width((WIN_WIDTH / 1.3) as u16).into()
    }

    fn get_scaled_color(&self) -> Color {
        if self.theme.extended_palette().is_dark {
            Color::WHITE
        } else {
            Color::BLACK
        }
        .scale_alpha(0.5)
    }

    fn weather_info<'a>(&self, weather: &'a WeatherData) -> Element<'a, Message> {
        let weather_icon = weather.weather[0].main.get_icon().unwrap_or_default();
        let weather_type = weather.weather[0].get_descr();
        let time = get_time(weather.dt, weather.timezone);

        let mut kv_weather = KeyValueView::new();
        kv_weather.add_item_with_units("Температура:", floor(weather.main.temp), "°C");
        kv_weather.add_item_with_units("Ощущается как:", floor(weather.main.feels_like), "°C");
        kv_weather.add_item_with_units(
            "Давление:",
            floor(weather.main.pressure as f32 / 1.33322),
            " мм.рт.ст",
        );
        kv_weather.add_item(
            "Рассвет:",
            get_time(weather.sys.sunrise, weather.timezone).format("%H:%M"),
        );
        kv_weather.add_item(
            "Закат:",
            get_time(weather.sys.sunset, weather.timezone).format("%H:%M"),
        );
        kv_weather.add_item_with_units("Ветер:", floor(weather.wind.speed), " м/с");
        let kv = kv_weather.view(self.theme.extended_palette().is_dark);

        let location_name = text(match &self.selected_location {
            Some(location) => location.to_string(),
            None => "Не выбранное/неизвестное местоположение".to_string(),
        });

        let scaled = self.get_scaled_color();

        center(container(column![
            location_name,
            center(
                column![
                    vertical_space(),
                    image(format!("res/icons/{}", weather_icon.get_icon_name(&time))),
                    text(format!(
                        "{} | {}",
                        weather_type,
                        time.format("%H:%M").to_string(),
                    ))
                    .size(25),
                    kv,
                    vertical_space(),
                ]
                .align_x(Center)
                .width(Length::Fill)
                .height(Length::Fill)
                .spacing(5),
            ),
            row![
                text("Информация предоставлена OpenWeatherMap")
                    .size(10)
                    .style(move |_| text::Style {
                        color: Some(scaled.clone()),
                    }),
                horizontal_space(),
                text(format!("Время работы: {}", Time::from_secs(self.uptime)))
                    .size(10)
                    .style(move |_| text::Style {
                        color: Some(scaled),
                    }),
            ],
        ]))
        .into()
    }

    fn view(&self) -> Element<Message> {
        let btn = container(
            row![
                button("Местоположение").on_press(Message::LocationSelectorPressed),
                button("Обновить").on_press(Message::RefreshButtonPressed),
                horizontal_space(),
                button(image("res/icons/about.png").width(20).height(20))
                    .on_press(Message::AboutButtonPressed)
                    .style(button::text),
                button(image("res/icons/settings.png").width(20).height(20))
                    .on_press(Message::SettingsButtonPressed)
                    .style(button::text),
                button(image("res/icons/power_off.png").width(20).height(20))
                    .on_press(Message::PoweroffButtonPressed)
                    .style(button::text),
            ]
            .spacing(5),
        )
        .style(container::rounded_box)
        .padding(10);

        let mut weather_area = column![btn,].padding(10).spacing(10);
        // If self.weather_data is `Some(wthr)`, display its content
        // If `None` - display simple placeholder
        if let Some(wthr) = &self.weather_data {
            weather_area = weather_area.push(self.weather_info(wthr));
        } else {
            weather_area = weather_area.push(match &self.is_err {
                Some(err) => placeholder(err),
                None => placeholder(
                    "Нажмите кнопку \"Обновить\" для получения\n\
                          сведений о погоде в заданном месте.",
                ),
            });
        }

        if let Some(modal_win) = self.modal {
            match modal_win {
                ModalWindow::LocationSelector => modal(
                    weather_area,
                    self.location_selector(),
                    Message::LocationSelectorPressed,
                ),
                ModalWindow::About => modal(
                    weather_area,
                    about(Message::AboutButtonPressed),
                    Message::AboutButtonPressed,
                ),
                ModalWindow::Settings => modal(
                    weather_area,
                    self.settings(),
                    Message::SettingsButtonPressed,
                ),
                ModalWindow::PowerOff => self.poweroff(weather_area),
            }
        } else {
            weather_area.into()
        }
    }
}

fn floor(num: f32) -> i32 {
    let n = if num <= 0. { num - 0.5 } else { num + 0.5 };
    n as i32
}

/// Create and run the main window of Ice
pub fn ui() -> iced::Result {
    iced::application(WIN_NAME, Ice::update, Ice::view)
        .centered()
        .antialiasing(WIN_ANTIALIASING)
        .decorations(false)
        .window_size((WIN_WIDTH, WIN_HEIGHT))
        .resizable(false)
        .theme(Ice::theme)
        .subscription(Ice::subscription)
        .run()
}

fn about<'a, Message>(on_press: Message) -> Element<'a, Message>
where
    Message: 'a + Clone,
{
    column![
        text("Ice").size(25),
        text(format!("ver.{}", env!("CARGO_PKG_VERSION"))).size(20),
        horizontal_rule(Pixels::from(0)),
        text(
            "\
            Ice - простейшая программа, предназначенная для работы на\n\
            микрокомпьютерах Raspberry/Orange Pi или подобных под\n\
            управлением операционной системы Armbian.",
        ),
        text(
            "\
            Всё, что может Ice - выводить на экран разрешением 800х480\n\
            информацию о погоде в данный момент времени. Возможно,\n\
            в будущем я дополню функционал программы, но на данный\n\
            момент мне лень.",
        ),
        text(
            "\
            Copyright (C) 2025 Michail Krasnov\n\
            Связь со мной: <michail383krasnov@mail.ru>\n\
            Донат: 2202 2062 5233 5406 (Сбер)",
        ),
        row![horizontal_space(), button("OK").on_press(on_press),],
    ]
    .width(Length::Shrink)
    .spacing(10)
    .into()
}
