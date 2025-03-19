//! Modal windows

use iced::{Element, Theme};

use crate::conf::Conf;

/**************************************************************************
 *                           Modal windows data                           *
 **************************************************************************/

#[derive(Clone)]
pub struct SettingsModal {
    /// Configuration of the Ice
    /// `Ice::default()` method parses the configuration file that defined in
    /// the `CONF_PATH` constant. If error return default value, if success -
    /// parsed data.
    pub conf: Conf,

    /// This is where the error text is written when performing any actions.
    /// Then, this text will be displayed in a special UI area, if the value
    /// of this field is not equal to `None`.
    pub is_err: Option<String>,
}

/**************************************************************************
 *                                Messages                                *
 **************************************************************************/

#[derive(Debug, Clone)]
pub enum SettingsMessage {
    /********************************************
     *               Page selectors             *
     ********************************************/
    BaseSettingsPressed,
    SystemMonitorPressed,

    /********************************************
     *            Interactive actions           *
     ********************************************/
    ThemeChanged(Theme),
    APIKeyChanged(String),
    UpdateTimeChanged(u8),
}

impl SettingsModal {
    pub fn view<'a>(&self) -> Element<'a, SettingsMessage> {
        todo!()
    }
}
