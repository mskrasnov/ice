pub mod api;
pub mod app;
pub mod config;
pub mod consts;
pub mod ui;

fn main() -> iced::Result {
    ui::ui()
}
