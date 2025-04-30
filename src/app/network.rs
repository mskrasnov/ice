//! Work with Wi-Fi (scan and connect)

use anyhow::Result;
use std::collections::HashMap;
use zbus::{Connection, zvariant::Value};

pub async fn scan_wifi() -> Result<Vec<String>> {
    let connection = Connection::system().await?;
    let proxy = zbus::Proxy::new(
        &connection,
        "org.freedesktop.NetworkManager",
        "unix:path=/org/freedesktop/NetworkManager/Devices/0",
        "org.freedesktop.NetworkManager.Device.Wireless",
    )
    .await?;

    let access_points: Vec<String> = proxy
        .call_method("GetAllAccessPoints", &())
        .await?
        .body()
        .deserialize()?;

    Ok(access_points)
}

pub async fn connect_wifi(ssid: &str, pass: &str) -> Result<()> {
    let connection = Connection::system().await?;
    let settings = zbus::Proxy::new(
        &connection,
        "org.freedesktop.NetworkManager",
        "/org/freedesktop/NetworkManager/Settings",
        "org.freedesktop.NetworkManager.Settings",
    )
    .await?;

    let mut config = HashMap::new();
    config.insert("ssid", Value::new(ssid));
    config.insert("psk", Value::new(pass));

    settings.call_method("AddConnection", &(config)).await?;
    Ok(())
}
