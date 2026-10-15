use tray_icon::{
    Icon, TrayIcon, TrayIconBuilder,
    menu::{Menu, MenuItem},
};

pub const OPEN_APP_ID: &str = "open-app";
pub const EXIT_ID: &str = "exit";

pub fn tray() -> TrayIcon {
    let mut rgba = Vec::with_capacity(32 * 32 * 4);

    for y in 0_i32..32 {
        for x in 0_i32..32 {
            let in_icon = (3..29).contains(&x)
                && (3..29).contains(&y)
                && !((x < 9 && y < 9 && (x - 9).pow(2) + (y - 9).pow(2) > 36)
                    || (x > 22 && y < 9 && (x - 22).pow(2) + (y - 9).pow(2) > 36)
                    || (x < 9 && y > 22 && (x - 9).pow(2) + (y - 22).pow(2) > 36)
                    || (x > 22 && y > 22 && (x - 22).pow(2) + (y - 22).pow(2) > 36));
            if !in_icon {
                rgba.extend_from_slice(&[0, 0, 0, 0]);
                continue;
            }

            let dx = x as f64 - 16.0;
            let dy = y as f64 - 20.0;
            let radius = (dx * dx + dy * dy).sqrt();
            let wifi_arc = (radius - 11.0).abs() < 1.1
                || (radius - 7.0).abs() < 1.1
                || (radius - 3.0).abs() < 1.1;
            let wifi_dot = (x - 16).pow(2) + (y - 25).pow(2) <= 2;

            if wifi_arc && y < 20 || wifi_dot {
                rgba.extend_from_slice(&[255, 255, 255, 255]);
            } else {
                rgba.extend_from_slice(&[8, 120, 249, 255]);
            }
        }
    }

    let icon = Icon::from_rgba(rgba, 32, 32).expect("Failed to create tray icon");
    let menu = Menu::new();
    let open_app = MenuItem::with_id(OPEN_APP_ID, "Open App", true, None);
    let exit = MenuItem::with_id(EXIT_ID, "Exit", true, None);
    menu.append(&open_app)
        .expect("Failed to add Open App tray menu item");
    menu.append(&exit)
        .expect("Failed to add Exit tray menu item");

    TrayIconBuilder::new()
        .with_menu(Box::new(menu))
        .with_tooltip("Network Monitor")
        .with_icon(icon)
        .with_menu_on_left_click(false)
        .build()
        .expect("Failed to create tray icon")
}
