use crate::{
    commands::messages::Message,
    models::{
        application::karting_time::KartingTime,
        driver::session_information::race_result::RaceResult,
    },
};
use iced::{
    Element, Renderer, Theme,
    widget::{button, column, row, text},
};
use iced_aw::widgets::Card;
use iced_table::Table;

const TABLE_WIDTH: f32 = 500.0;

impl KartingTime {
    pub fn race_results_view(&self) -> iced::widget::Column<'_, Message> {
        if self.driver_profile.name.is_empty() {
            column!()
        } else {
            let mut column = column!()
                .push(text("Results").size(24))
                .padding(10)
                .spacing(10);

            for result in self.read_only_result_cards() {
                column = column.push(result).padding(10).spacing(10);
            }

            column
        }
    }

    fn read_only_result_cards(&self) -> Vec<Card<'_, Message, Theme, Renderer>> {
        let mut result_cards = vec![];

        for race in self.driver_profile.filter.filtered_races.iter() {
            result_cards.push(self.create_result_card(race));
        }

        result_cards
    }

    fn create_result_card<'a>(&'a self, race: &'a RaceResult) -> Card<'a, Message> {
        let header = format!(
            "{} Session: {} Date: {}",
            race.race_information.track_name,
            race.race_information.session.session_id,
            race.race_information.date
        );

        if self.is_metadata_table_empty(race) {
            Card::new(
                text(header),
                column!(
                    text!("Laptimes"),
                    self.race_result_table(race),
                    text!("Total Times"),
                    self.total_time_table(race),
                    text!("Average Times"),
                    self.average_time_table(race),
                    text!("Race Summary"),
                    self.race_summary_table(race),
                ),
            )
            .foot(create_footer(race))
        } else {
            Card::new(
                text(header),
                column!(
                    text!("Laptimes"),
                    self.race_result_table(race),
                    text!("Total Times"),
                    self.total_time_table(race),
                    text!("Average Times"),
                    self.average_time_table(race),
                    text!("Race Summary"),
                    self.race_summary_table(race),
                    text!("Metadata"),
                    self.metadata_table(race),
                ),
            )
            .foot(create_footer(race))
        }
    }

    fn race_result_table(&self, race: &RaceResult) -> Element<'_, Message> {
        let mut table = Table::default();

        table.add_headers(vec!["Lap", "Time (s)"]);

        let mut lap_number = 1;

        for time in &race.race_statistics.formatted_laps {
            table.add_row(vec![&lap_number.to_string(), &time.to_string()]);

            lap_number += 1;
        }

        Table::build(
            table,
            Some(self.theme().palette().text),
            Some(TABLE_WIDTH),
            None,
            None,
        )
    }

    fn race_summary_table(&self, race: &RaceResult) -> Element<'_, Message> {
        let mut table = Table::default();

        table.add_headers(vec!["Race Summary", "Race Statistic"]);

        table.add_rows(vec![
            vec![
                "Race Position",
                &race.race_information.session.race_position.to_string(),
            ],
            vec![
                "Number of Laps",
                &race.race_statistics.number_of_laps.to_string(),
            ],
            vec!["Fastest Lap", &race.race_statistics.fastest_lap],
            vec!["Average Lap", &race.race_statistics.average_105_lap],
        ]);

        Table::build(
            table,
            Some(self.theme().palette().text),
            Some(TABLE_WIDTH),
            None,
            None,
        )
    }

    fn total_time_table(&self, race: &RaceResult) -> Element<'_, Message> {
        let mut table = Table::default();

        table.add_headers(vec!["Lap", "Time (s)"]);

        for (total_time_key, total_time_value) in &race.race_statistics.total_times_table {
            table.add_row(vec![total_time_key.to_string(), total_time_value.clone()]);
        }

        Table::build(
            table,
            Some(self.theme().palette().text),
            Some(TABLE_WIDTH),
            None,
            None,
        )
    }

    fn average_time_table(&self, race: &RaceResult) -> Element<'_, Message> {
        let mut table = Table::default();

        table.add_headers(vec!["Lap", "Time (s)"]);

        for (average_time_key, average_time_value) in &race.race_statistics.average_times_table {
            table.add_row(vec![
                average_time_key.to_string(),
                average_time_value.clone(),
            ]);
        }

        Table::build(
            table,
            Some(self.theme().palette().text),
            Some(TABLE_WIDTH),
            None,
            None,
        )
    }

    fn metadata_table(&self, race: &RaceResult) -> Element<'_, Message> {
        let mut table = Table::default();

        table.add_headers(vec!["Metadata", "Value"]);

        if !race.race_metadata.session_type.is_empty() {
            table.add_row(vec!["Session Type", &race.race_metadata.session_type]);
        }
        if !race.race_metadata.track_conditions.is_empty() {
            table.add_row(vec![
                "Track Conditions",
                &race.race_metadata.track_conditions,
            ]);
        }
        if !race.race_metadata.car_used.is_empty() {
            table.add_row(vec!["Car Used", &race.race_metadata.car_used]);
        }
        if !race.race_metadata.championship.is_empty() {
            table.add_row(vec!["Championship", &race.race_metadata.championship]);
        }

        Table::build(
            table,
            Some(self.theme().palette().text),
            Some(TABLE_WIDTH),
            None,
            None,
        )
    }

    fn is_metadata_table_empty(&self, race: &RaceResult) -> bool {
        race.race_metadata.notes.is_empty()
            && race.race_metadata.championship.is_empty()
            && race.race_metadata.car_used.is_empty()
            && race.race_metadata.session_type.is_empty()
            && race.race_metadata.track_conditions.is_empty()
    }
}

fn create_footer(race: &RaceResult) -> iced::widget::Column<'_, Message> {
    let notes = if !race.race_metadata.notes.is_empty() {
        format!("Notes: {}", race.race_metadata.notes)
    } else {
        String::from("")
    };

    match race.is_deleting {
        true => column!().push(text(notes)).spacing(10).padding(10).push(
            row!()
                .push(button("Confirm").on_press(Message::DeleteConfirmedPressed(
                    race.race_information.unique_race_identifier.clone(),
                )))
                .spacing(10)
                .padding(10)
                .push(button("Cancel").on_press(Message::DeleteCancelledPressed(
                    race.race_information.unique_race_identifier.clone(),
                )))
                .spacing(10)
                .padding(10),
        ),
        false => column!().push(text(notes)).spacing(10).padding(10).push(
            row!()
                .push(button("Replace").on_press(Message::ReplacePressed(
                    race.race_information.unique_race_identifier.clone(),
                )))
                .spacing(10)
                .padding(10)
                .push(button("Delete").on_press(Message::DeletePressed(
                    race.race_information.unique_race_identifier.clone(),
                )))
                .spacing(10)
                .padding(10),
        ),
    }
}
