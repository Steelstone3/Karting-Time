use crate::{commands::messages::Message, models::application::karting_time::KartingTime};
use iced::widget::{button, column, text, text_editor, text_input, Column};
use iced_aw::Card;

impl KartingTime {
    pub fn add_race_view(&self) -> Column<Message> {
        let add_race_contents = column!()
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
            .new_race
            .is_unique_identifer(&self.driver_profile.races)
        {
            add_race_button
                .push(button("Add Race").on_press(Message::UpdateRacesPressed))
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
