//! # ice
//!
//! Simple environment for ArmbianOS/other aarch64 Linux system

pub mod conf;
pub mod geo;
pub mod sys;
pub mod time;
pub mod weather;

pub mod ui;

fn main() -> iced::Result {
    ui::ui()
}
