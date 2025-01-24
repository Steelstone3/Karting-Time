use crate::{commands::messages::Message, models::application::karting_time::KartingTime};
use iced::{
    widget::{column, text},
    Renderer, Theme,
};
use iced_aw::widgets::Card;

impl KartingTime {
    pub fn results_view(&self) -> iced::widget::Column<Message> {
        if self.driver_profile.name.is_empty() {
            column!()
        } else {
            let mut column = column![];

            column = column
                .push(text("Results").size(24))
                .padding(10)
                .spacing(10);

            for result in self.result_cards() {
                column = column.push(result).padding(10).spacing(10);
            }

            column
        }
    }

    fn result_cards(&self) -> Vec<Card<Message, Theme, Renderer>> {
        let mut result_cards = vec![];

        for race in &self.driver_profile.races {
            let footer = format!(
                "Track Name: {}\nSession: {}\nDate: {}",
                race.track_name, race.session_id, race.date,
            )
            .to_string();

            result_cards
                .push(Card::new("Dive Profile", text(race.get_laptimes())).foot(text(footer)));
        }

        result_cards
    }
}
