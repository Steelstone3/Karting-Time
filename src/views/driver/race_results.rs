use crate::{
    commands::messages::Message,
    controllers::driver_profile::time_parser::format_laptime,
    models::{
        application::karting_time::KartingTime,
        driver::{race_information::RaceInformation, race_result::Race},
    },
    table::Table,
};
use iced::{
    Element, Renderer, Theme,
    widget::{button, column, row, text},
};
use iced_aw::widgets::Card;

impl KartingTime {
    pub fn race_results_view(&self) -> iced::widget::Column<Message> {
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

    fn read_only_result_cards(&self) -> Vec<Card<Message, Theme, Renderer>> {
        let mut result_cards = vec![];

        for race in self.application_state.filtered_races.iter() {
            let header = format!(
                "{} Session: {} Date: {}",
                race.race_information.track_name,
                race.race_information.session_id,
                race.race_information.date
            );

            let race_summary = format!(
                "Car Used: {}\n\nRace position: {}\nNumber of laps: {}\nFastest lap: {}\nAverage lap (105%): {}\n\nRace Pace:\n{}\n{}",
                race.race_information.car_used,
                race.race_information.race_position,
                race.get_number_of_laps(),
                format_laptime(race.get_fastest_lap()),
                format_laptime(race.get_average_lap()),
                race.convert_total_times_to_string(),
                race.convert_average_total_times_to_string()
            )
            .to_string();

            let footer = match race.is_deleting {
                true => column!()
                    .push(text(race_summary))
                    .spacing(10)
                    .padding(10)
                    .push(
                        row!()
                            // .push(button("Replace").on_press(Message::ReplacePressed(
                            //     RaceInformation::get_unique_race_identifier(&race.race_information),
                            // )))
                            // .spacing(10)
                            // .padding(10)
                            .push(button("Confirm").on_press(Message::DeleteConfirmedPressed(
                                RaceInformation::get_unique_race_identifier(&race.race_information),
                            )))
                            .spacing(10)
                            .padding(10)
                            .push(button("Cancel").on_press(Message::DeleteCancelledPressed(
                                RaceInformation::get_unique_race_identifier(&race.race_information),
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
                                RaceInformation::get_unique_race_identifier(&race.race_information),
                            )))
                            .spacing(10)
                            .padding(10)
                            .push(button("Delete").on_press(Message::DeletePressed(
                                RaceInformation::get_unique_race_identifier(&race.race_information),
                            )))
                            .spacing(10)
                            .padding(10),
                    ),
            };

            result_cards.push(Card::new(text(header), self.race_result_table(race)).foot(footer));
        }

        result_cards
    }

    fn race_result_table(&self, race: &Race) -> Element<Message> {
        let mut table = Table::default();

        table.add_headers(vec!["Lap".to_string(), "Time (s)".to_string()]);

        for laptime in &race.laptimes {
            table.add_row(vec![
                laptime.lap_number.to_string(),
                format_laptime(laptime.time),
            ]);
        }

        Table::build(table, self.theme().palette().text, Some(200.0))
    }
}
