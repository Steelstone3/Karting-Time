use crate::{commands::messages::Message, models::application::karting_time::KartingTime};
use iced::widget::{button, column, text, text_editor, text_input};
use iced_aw::Card;

impl KartingTime {
    pub fn race_editing_view(&self) -> iced::widget::Column<Message> {
        let edit_race_contents = column!()
            .push(text("Track Name"))
            .spacing(10)
            .padding(10)
            .push(
                text_input("Track Name", &self.new_race.track_name)
                    .on_input(Message::TrackNameChanged),
            )
            .push(text("Day (dd):"))
            .spacing(10)
            .padding(10)
            .push(
                text_input("Day of the Month", &self.new_race.date.day.to_string())
                    .on_input(Message::DayChanged),
            )
            .spacing(10)
            .padding(10)
            .push(text("Month (mm):"))
            .spacing(10)
            .padding(10)
            .push(
                text_input("Month of the Year", &self.new_race.date.month.to_string())
                    .on_input(Message::MonthChanged),
            )
            .spacing(10)
            .padding(10)
            .push(text("Year (YYYY):"))
            .spacing(10)
            .padding(10)
            .push(
                text_input("Year", &self.new_race.date.year.to_string())
                    .on_input(Message::YearChanged),
            )
            .spacing(10)
            .padding(10)
            .push(text("Session Number:"))
            .spacing(10)
            .padding(10)
            .push(
                text_input("Session Number", &self.new_race.session_id.to_string())
                    .on_input(Message::SessionIdChanged),
            )
            .push(text("Race Position:"))
            .spacing(10)
            .padding(10)
            .push(
                text_input("Race Position", &self.new_race.race_position.to_string())
                    .on_input(Message::RacePositionChanged),
            )
            .push(text("Laps"))
            .spacing(10)
            .padding(10)
            .push(
                text_editor(&self.application_state.text_editor)
                    .placeholder("Add laptimes here...")
                    .on_action(Message::LaptimeEditor),
            )
            .spacing(10)
            .padding(10)
            .push(button("Add Race").on_press(Message::AddRacePressed))
            .spacing(10)
            .padding(10);

        column!()
            .push(button("Save").on_press(Message::SaveRacesPressed))
            .padding(10)
            .spacing(10)
            .push(Card::new("Edit Race", edit_race_contents))
            .spacing(10)
            .padding(10)
    }

    // pub fn race_results_view(&self) -> iced::widget::Column<Message> {
    //     if self.driver_profile.name.is_empty() {
    //         column!()
    //     } else {
    //         let mut column = column![];

    //         column = column
    //             .push(text("Results").size(24))
    //             .padding(10)
    //             .spacing(10);

    //         for result in self.read_only_result_cards() {
    //             column = column.push(result).padding(10).spacing(10);
    //         }

    //         column
    //     }
    // }

    // fn read_only_result_cards(&self) -> Vec<Card<Message, Theme, Renderer>> {
    //     let mut result_cards = vec![];

    //     for race in &self.driver_profile.races {
    //         let header = format!(
    //             "{} Session: {} Date: {}",
    //             race.track_name, race.session_id, race.date
    //         );

    //         let footer = format!(
    //             "Number of laps: {}\nFastest lap: {:.2}\nAverage lap (105%): {:.2}\n\nRace Pace:\n{}\n{}",
    //             race.get_number_of_laps(),
    //             race.get_fastest_lap(),
    //             race.get_average_lap(),
    //             race.convert_total_times_to_string(),
    //             race.convert_average_total_times_to_string()
    //         )
    //         .to_string();

    //         result_cards.push(Card::new(text(header), text(race.to_string())).foot(text(footer)));
    //     }

    //     result_cards
    // }
}
