use iced::Settings;
use models::application::karting_time::KartingTime;

mod commands;
mod controllers;
mod models;
mod views;

pub fn main() -> iced::Result {
    iced::application("Karting Time", KartingTime::update, KartingTime::view)
        .theme(KartingTime::theme)
        .antialiasing(true)
        .settings(Settings::default())
        .run()
}
