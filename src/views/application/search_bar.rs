use crate::{commands::messages::Message, models::application::karting_time::KartingTime};
use iced::widget::{column, row, text, text_input};

impl KartingTime {
    pub fn search_bar_view(&self) -> iced::widget::Column<Message> {
        column!().push(
            row!()
                .push(text("Search:"))
                .padding(10)
                .spacing(10)
                .push(
                    text_input("Enter search query", &self.application_state.search_query)
                        .on_input(Message::SearchChanged),
                )
                .padding(10)
                .spacing(10),
        )
    }
}
