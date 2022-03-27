
mod gui;

use iced::Application;
use rustcord::{Rustcord, EventHandlers, RichPresenceBuilder, User};
use std::time::SystemTime;

use crate::gui::Tour;

fn main() { 
    Tour::run(iced::Settings::default());
}