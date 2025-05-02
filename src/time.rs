//! Time&date formatting

use chrono::{DateTime, Datelike, TimeZone, Timelike};
use std::fmt::{Debug, Display};

pub struct Time<'a, D: TimeZone + Debug> {
    time: &'a DateTime<D>,
    display_mode: DisplayMode,
}

#[derive(Clone, Copy, Default)]
pub enum DisplayMode {
    /// HH:MM
    #[default]
    TimeDefault,

    /// HH:MM:SS
    TimeWithSeconds,

    /// dd:mm:yyyy HH:MM
    TimeDate,

    /// dd:mm:yyyy HH:MM:SS
    TimeWithSecondsDate,
}

impl<'a, D: TimeZone + Debug> Time<'a, D> {
    pub fn new(time: &'a DateTime<D>) -> Self {
        Self {
            time,
            display_mode: DisplayMode::default(),
        }
    }

    pub fn set_display_mode(mut self, mode: DisplayMode) -> Self {
        self.display_mode = mode;
        self
    }
}

impl<'a, D: TimeZone + Debug> Display for Time<'a, D> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let time = &self.time;
        let h_zero = if time.hour() < 10 { "0" } else { "" };
        let m_zero = if time.minute() < 10 { "0" } else { "" };
        let s_zero = if time.second() < 10 { "0" } else { "" };

        let month_zero = if time.month() < 10 { "0" } else { "" };
        let day_zero = if time.day() < 10 { "0" } else { "" };

        write!(
            f,
            "{}",
            match self.display_mode {
                DisplayMode::TimeDefault =>
                    format!("{}{}:{}{}", h_zero, time.hour(), m_zero, time.minute()),
                DisplayMode::TimeWithSeconds => format!(
                    "{}:{}{}:{}{}",
                    time.hour(),
                    m_zero,
                    time.minute(),
                    s_zero,
                    time.second()
                ),
                DisplayMode::TimeDate => format!(
                    "{}{}.{}{}.{} {}:{}{}",
                    day_zero,
                    time.day(),
                    month_zero,
                    time.month(),
                    time.year(),
                    time.hour(),
                    m_zero,
                    time.minute()
                ),
                DisplayMode::TimeWithSecondsDate => format!(
                    "{}{}.{}{}.{} {}:{}{}:{}{}",
                    day_zero,
                    time.day(),
                    month_zero,
                    time.month(),
                    time.year(),
                    time.hour(),
                    m_zero,
                    time.minute(),
                    s_zero,
                    time.second()
                ),
            }
        )
    }
}
