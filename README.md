<div align="center">
  <h1>❄️ Ice</h1>
  <h3>Weather Monitor for Embedded Systems | Погодный монитор для встраиваемых систем</h3>

  [![Rust Version](https://img.shields.io/badge/Rust-1.85%2B-orange?logo=rust)](https://www.rust-lang.org/)
  [![License MIT](https://img.shields.io/badge/License-MIT-blue)](LICENSE)
</div>

## 🤔 What is this?

![](assets/screen.png)

The modern application for get weather forecasts (current, daily). Written for Orange/Raspberry Pi and Armbian OS.

## ✨ Features

- **Current weather:** temperature, pressure and wind;
- **Daily forecast** in the compact mode;
- **System monitoring:** CPU load, RAM usage and disk space;
- **Wi-Fi:** simple and quick Wi-Fi scanning and connecting;
- **Location autodetect** via IP/GPS;
- **Touch-friendly UI** for 800×480 displays

## ️🛠️ Installation

You can install Ice from sources:

```bash
git clone https://github.com/mskrasnov/ice
cd ice

python      ./build.py build aarch64-unknown-linux-gnu
sudo python ./build.py install

sudo systemctl enable ice.service
sudo systemctl start  ice
```

... or install pre-built `*.deb`-package from "Releases" page:

- [Releases](https://github.com/mskrasnov/ice/releases)

## 🎨 Interface

- **Base screen resolution:** 800x480
- **Controls:** optimized for sensor screens
  1. Update
  2. Location
  3. Settings
  4. About program
  5. Exit/shutdown

## 🤖 Technology stack

- **OS:** Armbian Linux (ARM v8.1-A)
- **Language:** Rust
- **i18n support:** `fluent`
- **Interface:** [iced](https://iced.rs)
- **Network stack:** NetworkManager
- **Hardware:** Orange/Raspberry Pi SBC

## 🤝 Contributing

Bug reports and PR are welcome!

1. Fork this repo: `git clone https://github.com/mskrasnov/ice`
2. Go to the cloned directory: `cd ice`
3. Create new branch:
  - `git branch feature/<SOME NEW FEATURE>` **or** `git branch fix/<SOME BUGFIX>`
  - `git checkout <CREATED BRANCH NAME>`
4. some changes...
5. Push changes: `git push origin <CREATED BRANCH NAME>`
6. Create pull request in the GitHub interface
7. Thank you!

## 💸 Support me

If you from Russia you can send me donation:

> 2202206252335406 (Михаил Сергеевич)

## 📜 License

[MIT](LICENSE) © 2025 Michail Krasnov
