#![allow(dead_code)]

mod lang_proc;
mod model;
mod user_interface;

use user_interface::BarnabyApp;

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
