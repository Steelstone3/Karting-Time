use iced::Settings;
use models::application::karting_time::KartingTime;

mod commands;
mod controllers;
mod models;
mod views;

#[deny(clippy::unwrap_used)]
#[deny(clippy::expect_used)]
#[deny(clippy::panic)]
#[deny(unused_must_use)]
pub fn main() -> iced::Result {
    iced::application("Karting Time", KartingTime::update, KartingTime::view)
        .theme(KartingTime::theme)
        .antialiasing(true)
        .settings(Settings::default())
        .run()
}
