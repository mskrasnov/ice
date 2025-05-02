pub mod api;
pub mod app;
pub mod config;
pub mod consts;
pub mod ui;
pub mod units;
pub mod time;

fn main() -> iced::Result {
    ui::ui()
}
