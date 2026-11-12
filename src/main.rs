#![cfg_attr(not(test), windows_subsystem = "windows")]
mod brand;
mod hardware;
mod library;
mod lighting;
mod preferences;
mod sources;
mod tray;
mod win_ui;
mod worker;
fn main() {
    win_ui::run();
}
