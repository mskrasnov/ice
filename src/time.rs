//! Convert seconds to hours, minutes and seconds

use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub struct Time {
    pub hours: u16,
    pub mins: u8,
    pub secs: u8,
}

impl Time {
    pub fn from_secs(secs: u32) -> Self {
        // WARN: SHIT CODE
        let hours: u16 = (secs / 3600) as u16;
        let mins: u8 = ((secs - 3600 * hours as u32) / 60) as u8;
        let s: u8 = (secs - (3600 * hours) as u32 - 60 * mins as u32) as u8;

        Self {
            hours,
            mins,
            secs: s,
        }
    }
}

impl Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{}{}:{}{}",
            self.hours,
            if self.mins < 10 { "0" } else { "" },
            self.mins,
            if self.secs < 10 { "0" } else { "" },
            self.secs
        )
    }
}
