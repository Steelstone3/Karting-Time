use iced::Settings;
use models::application::karting_time::KartingTime;

mod commands;
mod controllers;
mod data_models;
mod models;
mod views;

#[deny(clippy::unwrap_used)]
#[deny(clippy::expect_used)]
#[deny(clippy::panic)]
#[deny(unused_must_use)]
pub fn main() -> iced::Result {
    iced::application(KartingTime::boot, KartingTime::update, KartingTime::view)
        .theme(KartingTime::theme)
        .antialiasing(true)
        .settings(Settings {
            id: Some("Karting Time".to_string()),
            ..Default::default()
        })
        .run()
}
