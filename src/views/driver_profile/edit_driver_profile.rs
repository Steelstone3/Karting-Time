use iced::widget::{column, text, text_input, Column};
use iced_aw::Card;

use crate::{commands::messages::Message, models::application::karting_time::KartingTime};

impl KartingTime {
    pub fn edit_driver_profile_view(&self) -> Column<Message> {
        let edit_profile_contents = column!()
            .push(text("Driver Name:"))
            .spacing(10)
            .padding(10)
            .push(
                text_input("Driver Name", &self.driver_profile.name)
                    .on_input(Message::DriverNameChanged),
            );

        column!()
            .push(Card::new("Edit Driver Profile", edit_profile_contents))
            .spacing(10)
            .padding(10)
    }
}
