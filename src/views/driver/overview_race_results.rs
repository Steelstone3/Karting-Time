use crate::{
    commands::messages::Message,
    models::{application::karting_time::KartingTime, driver::race_result::Race},
};
use iced::{
    Element,
    widget::{column, text},
};
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
                    text("Hello")
                    // KartingTime::race_results_overview_table(
                    //     &self.application_state.filtered_races,
                    // ),
                ))
                .padding(10)
                .spacing(10);

            column
        }
    }

    fn race_results_overview_table(_races: &Vec<Race>) -> Element<Message> {
        todo!()
    }
}
