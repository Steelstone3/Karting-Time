use crate::{
    commands::messages::Message,
    models::{application::karting_time::KartingTime, driver::race_result::Race},
};
use iced::widget::{column, text};
use iced_aw::widgets::Card;

impl KartingTime {
    pub fn overview_race_results_view(&self) -> iced::widget::Column<Message> {
        if self.driver_profile.name.is_empty() || self.driver_profile.races.is_empty() {
            column!()
        } else {
            let mut column = column!()
                .push(text("Results Overview").size(24))
                .padding(10)
                .spacing(10);

            column = column
                .push(Card::new(
                    text("Results Overview"),
                    text(Race::display_race_results_overview(
                        &self.application_state.filtered_races,
                    )),
                ))
                .padding(10)
                .spacing(10);

            column
        }
    }
}
