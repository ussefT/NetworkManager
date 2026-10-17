# Network Manager

![image](https://github.com/ussefT/NetworkManager/image.png)

A small Linux desktop app for keeping an eye on network activity. Network Manager is built with Rust and GTK4, and gives you a full dashboard as well as a compact window you can open from the system tray.

[Fa](https://github.com/ussefT/NetworkManager/README-fa.md)

> **Status:** Early development (`0.1.0`). The interface and behavior may change.

## What it does

- Lists the network interfaces detected when the app starts and lets you choose one to monitor.
- Shows download and upload readings, cumulative transfer totals, and a scrolling activity graph.
- Provides a full dashboard and a smaller dashboard with the same selected interface and monitoring state.
- Lets you pause and resume monitoring.
- Offers light and dark themes.
- Adds a system-tray icon. Click it to show or hide the compact window; use its menu to open the full dashboard or exit.
- Hides the dashboard when its window close button is used, so the app can remain available in the tray.

## A note about the readings

The dashboard currently labels its download and upload readings as speeds, but the values are calculated from the system's cumulative received and transmitted byte counters. They are not yet calculated as changes per second. The graph plots those same readings. The total fields show cumulative transfer counters.

## Requirements

- Linux desktop session with GTK4 and a system tray/status notifier supported by your desktop environment.
- Rust and Cargo (Rust 1.85 or newer, for the Rust 2024 edition).
- GTK4 development files and `pkg-config`, installed using your distribution's package manager.

For Debian or Ubuntu, the GTK build dependencies can be installed with:

```bash
sudo apt install build-essential pkg-config libgtk-4-dev
```

Install Rust with [rustup](https://rustup.rs/) if it is not already available.

### Library documentation

- [gtk4](https://docs.rs/gtk4/0.11.5/gtk4/) — GTK4 bindings for Rust, used to build the desktop interface.
- [sysinfo](https://docs.rs/sysinfo/0.39.6/sysinfo/) — system information library used to read network interface counters.
- [tray-icon](https://docs.rs/tray-icon/0.26.1/tray_icon/) — system tray icon and menu support.

## Build and run

From the project directory:

```bash
cargo check
cargo run
```

To build an optimized executable:

```bash
cargo build --release
```

The resulting binary is at `target/release/my-gtk-app`.

## Project layout

- `src/main.rs` starts the tray icon and GTK application.
- `src/ui.rs` builds the dashboards, graph, controls, and themes.
- `src/network.rs` reads interface counters through `sysinfo`.
- `src/tray.rs` creates the system-tray icon and its menu.

## Contributing

Contributions are very welcome! Whether you have spotted a bug, thought of a useful feature, improved the documentation, or simply want to help make Network Manager better, your input is appreciated.

### Found a bug or have an idea?

Please [open an issue](https://github.com/ussefT/NetworkManager/issues). A helpful report includes:

- What you expected to happen and what actually happened.
- Steps to reproduce the problem.
- Your Linux distribution and desktop environment.
- Any relevant error messages or screenshots.

For feature ideas, tell us what problem the change would solve. It’s perfectly fine to open an issue just to start a conversation—there’s no need to have all the answers first.

### Want to contribute code?

1. Open an issue first for larger changes so we can discuss the approach.
2. Fork the repository and create a branch for your change.
3. Make your change and run `cargo check`.
4. Open a pull request with a short description of what you changed and why.

Small fixes and documentation improvements are welcome too. Thank you for taking the time to contribute!