#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod app;
pub mod img2br;
pub mod state;

use app::ImgMagicksApp;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Img-magicks",
        options,
        Box::new(|cc| Box::new(ImgMagicksApp::new(cc))),
    )
}
