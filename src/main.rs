mod gui;

use iced::{Application, Settings};

use crate::gui::Tour;

fn main() {
    Tour::run(Settings {
        default_font: Some(include_bytes!("../font/NotoSansJP-Regular.otf")),
        ..Settings::default()
    })
    .unwrap();
}
