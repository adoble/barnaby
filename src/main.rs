#![allow(dead_code)]

mod gui;
mod model;
mod parse;

use gui::BarnabyApp;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1024.0, 768.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Barnaby",
        native_options,
        Box::new(|cc| Box::new(BarnabyApp::new(cc))),
    )
}
