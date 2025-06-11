mod app;
mod program_options;

use app::Gui;
use eframe::egui;

use eframe::{App};

fn main() -> eframe::Result 
{
    let options = eframe::NativeOptions 
    {
        viewport: egui::ViewportBuilder::default().with_inner_size([900.0, 700.0]),
        ..Default::default()
    };

    let gui = Gui::default();

    eframe::run_native(
        "itsboard-uploader",
        options,
        Box::new(|_cc| 
        {
            Ok(Box::new(gui))
        }),
    )
}