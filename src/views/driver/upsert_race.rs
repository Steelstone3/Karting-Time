use crate::{commands::messages::Message, models::application::karting_time::KartingTime};
use iced::widget::{Column, button, column, text, text_editor, text_input};
use iced_aw::Card;

impl KartingTime {
    pub fn upsert_race_view(&self) -> Column<'_, Message> {
        let add_race_contents = column!()
            .push(text("Track Name:"))
            .spacing(10)
            .padding(10)
            .push(
                text_input(
                    "Track Name",
                    &self.application_state.new_race.race_information.track_name,
                )
                .on_input(Message::TrackNameChanged),
            )
            .push(text("Day (dd):"))
            .spacing(10)
            .padding(10)
            .push(
                text_input(
                    "Day of the Month",
                    &self
                        .application_state
                        .new_race
                        .race_information
                        .date
                        .day
                        .to_string(),
                )
                .on_input(Message::DayChanged),
            )
            .spacing(10)
            .padding(10)
            .push(text("Month (mm):"))
            .spacing(10)
            .padding(10)
            .push(
                text_input(
                    "Month of the Year",
                    &self
                        .application_state
                        .new_race
                        .race_information
                        .date
                        .month
                        .to_string(),
                )
                .on_input(Message::MonthChanged),
            )
            .spacing(10)
            .padding(10)
            .push(text("Year (YYYY):"))
            .spacing(10)
            .padding(10)
            .push(
                text_input(
                    "Year",
                    &self
                        .application_state
                        .new_race
                        .race_information
                        .date
                        .year
                        .to_string(),
                )
                .on_input(Message::YearChanged),
            )
            .spacing(10)
            .padding(10)
            .push(text("Session Number:"))
            .spacing(10)
            .padding(10)
            .push(
                text_input(
                    "Session Number",
                    &self
                        .application_state
                        .new_race
                        .race_information
                        .session_id
                        .to_string(),
                )
                .on_input(Message::SessionIdChanged),
            )
            .push(text("Race Position:"))
            .spacing(10)
            .padding(10)
            .push(
                text_input(
                    "Race Position",
                    &self
                        .application_state
                        .new_race
                        .race_information
                        .race_position
                        .to_string(),
                )
                .on_input(Message::RacePositionChanged),
            )
            .push(text("Car:"))
            .spacing(10)
            .padding(10)
            .push(
                text_input(
                    "Car Used",
                    &self
                        .application_state
                        .new_race
                        .race_information
                        .car_used
                        .to_string(),
                )
                .on_input(Message::CarUsedChanged),
            )
            .push(text("Laps:"))
            .spacing(10)
            .padding(10)
            .push(
                text_editor(&self.application_state.race_editor.text_editor)
                    .placeholder("Add laptimes here...")
                    .on_action(Message::LaptimeEditor),
            )
            .spacing(10)
            .padding(10);

        let body = column!()
            .push(add_race_contents)
            .push(self.determine_add_race_button());

        column!()
            .push(Card::new("Add Race", body))
            .spacing(10)
            .padding(10)
    }

    fn determine_add_race_button(&self) -> Column<'_, Message> {
        let add_race_button = column!();

        if self
            .application_state
            .new_race
            .is_unique_identifer(&self.driver_profile.races)
            && !self
                .application_state
                .new_race
                .race_information
                .track_name
                .is_empty()
        {
            add_race_button
                .push(button("Add Race").on_press(Message::UpdateRacesPressed))
                .spacing(10)
                .padding(10)
                .push(button("Clear Laps").on_press(Message::ClearRaceEditorPressed))
                .spacing(10)
                .padding(10)
        } else if self
            .application_state
            .new_race
            .race_information
            .track_name
            .is_empty()
        {
            add_race_button
                .push(button("Clear Laps").on_press(Message::ClearRaceEditorPressed))
                .spacing(10)
                .padding(10)
        } else {
            add_race_button
                .push(button("Update Race").on_press(Message::UpdateRacesPressed))
                .spacing(10)
                .padding(10)
        }
    }
}
