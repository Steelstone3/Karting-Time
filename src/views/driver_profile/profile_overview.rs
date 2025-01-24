use crate::{commands::messages::Message, models::application::karting_time::KartingTime};
use iced::widget::{column, text, Column};
use iced_aw::Card;

impl KartingTime {
    pub fn overview_view(&self) -> Column<Message> {
        if self.driver_profile.name.is_empty() {
            return column!();
        }

        let content = column!()
            .push(text(self.driver_profile.name.to_string()).size(24))
            .padding(10)
            .spacing(10)
            .push(text("Races:"))
            .padding(10)
            .spacing(10)
            .push(text("Wins:"))
            .padding(10)
            .spacing(10)
            .push(text("Podiums:"))
            .padding(10)
            .spacing(10)
            .push(text("Top Fives:"))
            .padding(10)
            .spacing(10)
            .push(text("Unique Tracks:"))
            .padding(10)
            .spacing(10);

        column!()
            .push(Card::new(
                text(self.driver_profile.name.to_string()),
                content,
            ))
            .padding(10)
            .spacing(10)
    }
}
