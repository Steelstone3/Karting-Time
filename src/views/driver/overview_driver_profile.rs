use crate::{commands::messages::Message, models::application::karting_time::KartingTime};
use iced::widget::{column, row, text, Column};
use iced_aw::Card;

impl KartingTime {
    pub fn overview_driver_profile_view(&self) -> Column<Message> {
        let content = column!()
            .push(text(self.driver_profile.name.to_string()).size(24))
            .padding(10)
            .spacing(10)
            .push(
                row!()
                    .push(text("Races:"))
                    .spacing(10)
                    .push(text(self.driver_profile.get_number_of_races())),
            )
            .push(
                row!()
                    .push(text("Wins:"))
                    .spacing(10)
                    .push(text(self.driver_profile.get_number_of_wins())),
            )
            .push(
                row!()
                    .push(text("Podiums:"))
                    .spacing(10)
                    .push(text(self.driver_profile.get_number_of_podiums())),
            )
            .push(
                row!()
                    .push(text("Top Fives:"))
                    .spacing(10)
                    .push(text(self.driver_profile.get_number_of_top_fives())),
            )
            .push(
                row!()
                    .push(text("Top Tens:"))
                    .spacing(10)
                    .push(text(self.driver_profile.get_number_of_top_tens())),
            )
            .push(
                row!()
                    .push(text("Unique Tracks:"))
                    .spacing(10)
                    .push(text(self.driver_profile.get_number_of_unique_tracks())),
            );

        column!()
            .push(Card::new(
                text(self.driver_profile.name.to_string()),
                content,
            ))
            .padding(10)
            .spacing(10)
    }
}
