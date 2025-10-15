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
            let header = format!(
                "{} Session: {} Date: {}",
                race.race_information.track_name,
                race.race_information.session.session_id,
                race.race_information.date
            );

            let notes_line = if !race.race_metadata.notes.is_empty() {
                format!("\n\nNotes: {}", race.race_metadata.notes)
            } else {
                String::from("")
            };

            let championship_line = if !race.race_metadata.championship.is_empty() {
                format!("\nChampionship: {}", race.race_metadata.championship)
            } else {
                String::from("")
            };

            let race_summary = format!(
                "Session Type: {}\nTrack Conditions: {}\nCar Used: {}{}\n\nRace position: {}\nNumber of laps: {}\nFastest lap: {}\nAverage lap (105%): {}\n\nRace Pace:\n{}\n{}{}",
                race.race_metadata.session_type,
                race.race_metadata.track_conditions,
                race.race_metadata.car_used,
                championship_line,
                race.race_information.session.race_position,
                race.race_statistics.number_of_laps,
                race.race_statistics.fastest_lap,
                race.race_statistics.average_105_lap,
                race.race_statistics.total_times_summary,
                race.race_statistics.average_times_summary,
                notes_line
            )
            .to_string();

            let footer = match race.is_deleting {
                true => column!()
                    .push(text(race_summary))
                    .spacing(10)
                    .padding(10)
                    .push(
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
                false => column!()
                    .push(text(race_summary))
                    .spacing(10)
                    .padding(10)
                    .push(
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
            };

            result_cards.push(Card::new(text(header), self.race_result_table(race)).foot(footer));
        }

        result_cards
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
            Some(200.0),
            None,
            None,
        )
    }
}
