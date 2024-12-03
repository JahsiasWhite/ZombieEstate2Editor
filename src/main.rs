use eframe::egui;
use std::path::PathBuf;
mod app;
mod file_manager;
mod xml_handler;
mod config;

fn main() -> Result<(), eframe::Error> {
    env_logger::init();
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(800.0, 600.0)),
        ..Default::default()
    };
    eframe::run_native(
        "Game Mod Manager",
        options,
        Box::new(|cc| Box::new(app::ModManagerApp::new(cc)))
    )
}