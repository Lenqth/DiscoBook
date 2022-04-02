mod gui;
mod save;

use crate::gui::Tour;
use crate::save::*;
use iced::{Application, Settings};

fn main() {
    let app_state = load_settings().unwrap_or_default();

    Tour::run(Settings {
        default_font: Some(include_bytes!("../font/NotoSansJP-Regular.otf")),
        flags: app_state,
        ..Settings::default()
    })
    .unwrap();
}
