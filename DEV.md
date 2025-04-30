# Development notes

## Program architecture

```
main.rs   -> entry point, UI initializing
consts.rs -> constants and global variables
config.rs -> configuration file of the Ice

app.rs  -> main application code
  weather.rs   -> work with weather API's
  network.rs   -> work with Wi-Fi (scan and connect)
  system.rs    -> CPU, RAM and disk space monitoring
  location.rs  -> autodetect user location (and get (lat, lon) coordinates)

api.rs  -> main functions for work with OpenWeatherMap API, 'API' and 'Json' traits
  current.rs   -> get current weather forecast
  daily.rs     -> get daily weather forecast
  geocoding.rs -> get coordinates of given location

ui.rs  -> user interface of program
  widgets.rs   -> custom widgets
  styles.rs    -> custom styles

  current.rs   -> current weather container
  daily.rs     -> daily forecast container

  modal.rs     -> some modal windows (location selector, about program, error windows)

  network.rs   -> network page
  settings.rs  -> settings page
  weather.rs   -> weather forecasts page

  update.rs    -> data modifying
  view.rs      -> main UI interface
```

## Simple interface of the main page

```
+----------------------------------------------------------------+
|                                                                |
| +---+                                                          |
| | U | Dzerzhinsk, Russia                              01:17:52 |
| +---+                                                          |
|                                                                |
| +---------------+                                              |
| |               |                                              |
| |               |                                              |
| |               | 52 C                                         |
| |               |                <DAILY FORECAST PAGE>         |
| |               |                                              |
| |               |                                              |
| +---------------+                                              |
|                                                                |
| <Location> <Settings> <About> <Poweroff>      Uptime: 00:00:00 |
+----------------------------------------------------------------+
```

- Default font size: 20pt
- Temperature: 35pt
- Time & location: 25pt
- Small text: 12-15pt
