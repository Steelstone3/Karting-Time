use crate::{
    commands::messages::Message,
    models::{application::karting_time::KartingTime, driver::race_result::Race},
};
use iced::{
    Element, Renderer, Theme,
    widget::{Canvas, Column, Row, column, container, row, scrollable, text},
};
use iced_aw::widgets::Card;
use iced_table::{Table, table};

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
                    KartingTime::race_results_overview_table(
                        &self.application_state.filtered_races,
                    ),
                ))
                .padding(10)
                .spacing(10);

            column
        }
    }

    fn race_results_overview_table(races: &Vec<Race>) -> Element<Message> {
        column!(
            table(
                scrollable::Id::unique(),
                scrollable::Id::unique(),
                &vec![column!()],
                &vec![row!()],
                Message::SyncHeader,
            )
            // .style(Default::default())
            .into(),
        )
        .into()
    }
}
