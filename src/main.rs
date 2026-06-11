mod commands;
mod controllers;
mod data_models;
mod models;
mod views;

pub fn main() -> iced::Result {
    #[cfg(not(test))]
    {
        use crate::models::application::karting_time::KartingTime;
        use iced::Settings;

        iced::application(KartingTime::boot, KartingTime::update, KartingTime::view)
            .theme(KartingTime::theme)
            .antialiasing(true)
            .settings(Settings {
                id: Some("Karting Time".to_string()),
                ..Default::default()
            })
            .run()
    }
    #[cfg(test)]
    {
        Ok(())
    }
}
