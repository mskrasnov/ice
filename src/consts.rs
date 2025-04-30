//! Constants and global variables

pub const PROG_NAME: &str = env!("CARGO_PKG_NAME");
pub const PROG_VER: &str = env!("CARGO_PKG_VERSION");
pub const PROG_AUTHOR: &str = "(C) 2025 Michail Krasnov <michail383krasnov@mail.ru>";

pub const CONF_PATH: &str = "./config/ice.toml";
pub const CACHE_DIR_PATH: &str = "./cache/ice/";
pub const GEO_CACHE: &str = "geo.json";
pub const CURRENT_CACHE: &str = "current.json";
pub const DAILY_CACHE: &str = "daily.json";

pub const DEFAULT_WIN_SIZE: (u16, u16) = (800, 480);
