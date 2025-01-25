use crate::{commands::messages::Message, models::application::karting_time::KartingTime};
use iced::widget::{column, text, Column};
use iced_aw::Card;

impl KartingTime {
    pub fn race_view(&self) -> Column<Message> {
        let contents = column!()
            .push(text("Track Name"))
            .spacing(10)
            .padding(10)
            .push(text("Time"))
            .spacing(10)
            .padding(10);

        column!()
            .push(Card::new("Race", contents))
            .spacing(10)
            .padding(10)
    }
}
