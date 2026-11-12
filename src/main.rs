#![cfg_attr(not(test), windows_subsystem = "windows")]
mod hardware;
mod lighting;
mod win_ui;
mod worker;
fn main() {
    win_ui::run();
}
