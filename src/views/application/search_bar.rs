use crate::{commands::messages::Message, models::application::karting_time::KartingTime};
use iced::widget::{column, text, text_input};

impl KartingTime {
    pub fn search_bar_view(&self) -> iced::widget::Column<Message> {
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
    }
}
