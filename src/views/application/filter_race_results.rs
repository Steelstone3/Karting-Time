use crate::{commands::messages::Message, models::application::karting_time::KartingTime};
use iced::widget::{column, text, text_input};

impl KartingTime {
    pub fn filter_results_view(&self) -> iced::widget::Column<'_, Message> {
        column!()
            .push(text("Filter").size(24))
            .padding(10)
            .spacing(10)
            .push(text("Track:"))
            .padding(10)
            .spacing(10)
            .push(
                text_input("Enter track filter", &self.application_state.track_query)
                    .on_input(Message::TrackFilterChanged),
            )
            .padding(10)
            .spacing(10)
            .push(text("Date:"))
            .padding(10)
            .spacing(10)
            .push(
                text_input("Enter date filter", &self.application_state.date_query)
                    .on_input(Message::DateFilterChanged),
            )
            .padding(10)
            .spacing(10)
            .push(text("Car Used:"))
            .padding(10)
            .spacing(10)
            .push(
                text_input(
                    "Enter car used filter",
                    &self.application_state.car_used_query,
                )
                .on_input(Message::CarUsedFilterChanged),
            )
            .padding(10)
            .spacing(10)
            .push(text("Championship:"))
            .padding(10)
            .spacing(10)
            .push(
                text_input(
                    "Enter championship filter",
                    &self.application_state.championship_query,
                )
                .on_input(Message::ChampionshipFilterChanged),
            )
            .padding(10)
            .spacing(10)
            .push(text("Session Type:"))
            .padding(10)
            .spacing(10)
            .push(
                text_input(
                    "Enter session type filter",
                    &self.application_state.session_type_query,
                )
                .on_input(Message::SessionTypeFilterChanged),
            )
            .padding(10)
            .spacing(10)
    }
}
