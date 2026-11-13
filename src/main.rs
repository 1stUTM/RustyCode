#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(dead_code, unused_imports, unused_variables, non_snake_case, overflowing_literals)]

mod app;
mod core;
mod ui;
mod mega;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_title("RustyCode"),
        ..Default::default()
    };
    eframe::run_native(
        "RustyCode",
        options,
        Box::new(|cc| Box::new(app::EditorApp::new(cc))),
    )
}
