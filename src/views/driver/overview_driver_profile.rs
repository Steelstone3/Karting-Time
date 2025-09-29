use crate::{commands::messages::Message, models::application::karting_time::KartingTime};
use iced::widget::{Column, column, row, text};
use iced_aw::Card;

impl KartingTime {
    pub fn overview_driver_profile_view(&self) -> Column<'_, Message> {
        let content = column!()
            .push(text(self.driver_profile.name.to_string()).size(24))
            .padding(10)
            .spacing(10)
            .push(
                row!()
                    .push(text("Races:"))
                    .spacing(10)
                    .push(text(self.driver_profile.profile_statistics.races)),
            )
            .push(
                row!()
                    .push(text("Wins:"))
                    .spacing(10)
                    .push(text(self.driver_profile.profile_statistics.wins)),
            )
            .push(
                row!()
                    .push(text("Podiums:"))
                    .spacing(10)
                    .push(text(self.driver_profile.profile_statistics.podiums)),
            )
            .push(
                row!()
                    .push(text("Top Fives:"))
                    .spacing(10)
                    .push(text(self.driver_profile.profile_statistics.top_5)),
            )
            .push(
                row!()
                    .push(text("Top Tens:"))
                    .spacing(10)
                    .push(text(self.driver_profile.profile_statistics.top_10)),
            )
            .push(
                row!()
                    .push(text("Unique Tracks:"))
                    .spacing(10)
                    .push(text(self.driver_profile.profile_statistics.unique_tracks)),
            )
            .push(
                row!()
                    .push(text("Unique Cars:"))
                    .spacing(10)
                    .push(text(self.driver_profile.profile_statistics.unique_cars)),
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
