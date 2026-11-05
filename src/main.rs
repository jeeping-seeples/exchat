mod config;
mod ui;

use anyhow::Result;
use eframe::NativeOptions;
use ui::app::ExchatApp;

fn main() -> Result<()> {
    let settings = config::Settings::load()?;

    let native_options = NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_title("exchat")
            .with_inner_size([1024.0, 720.0])
            .with_min_inner_size([720.0, 480.0]),
        ..Default::default()
    };

    eframe::run_native(
        "exchat",
        native_options,
        Box::new(|cc| Ok(Box::new(ExchatApp::new(cc, settings)))),
    )
    .map_err(|e| anyhow::anyhow!("eframe error: {e}"))
}
