use crate::{commands::messages::Message, models::application::karting_time::KartingTime};
use iced::widget::column;

impl KartingTime {
    pub fn search_bar_view(&self) -> iced::widget::Column<Message> {
        column!()
    }
}
