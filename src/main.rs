mod network;
mod tray;
mod ui;

fn main() {
    ui::ui(tray::tray());
}
