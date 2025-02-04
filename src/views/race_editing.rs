use crate::{commands::messages::Message, models::application::karting_time::KartingTime};
use iced::{
    widget::{button, column, text, text_editor, text_input},
    Renderer, Theme,
};
use iced_aw::Card;

impl KartingTime {
    pub fn race_editing_view(&self) -> iced::widget::Column<Message> {
        let mut column = column!()
            .push(button("Save").on_press(Message::SaveRacesPressed))
            .padding(10)
            .spacing(10);

        column = column
            .push(text("Editing").size(24))
            .padding(10)
            .spacing(10);

        for edit in self.edit_result_cards() {
            column = column.push(edit).padding(10).spacing(10);
        }

        column
    }

    fn edit_result_cards(&self) -> Vec<Card<Message, Theme, Renderer>> {
        let mut result_cards = vec![];

        for race in &self.driver_profile.races {
            let header = format!(
                "Editing {} Session: {} Date: {}",
                race.track_name, race.session_id, race.date
            );

            let edit_race_contents = column!()
                .push(text("Track Name"))
                .spacing(10)
                .padding(10)
                .push(
                    text_input("Track Name", &race.track_name).on_input(Message::TrackNameChanged),
                )
                .push(text("Day (dd):"))
                .spacing(10)
                .padding(10)
                .push(
                    text_input("Day of the Month", &race.date.day.to_string())
                        .on_input(Message::DayChanged),
                )
                .spacing(10)
                .padding(10)
                .push(text("Month (mm):"))
                .spacing(10)
                .padding(10)
                .push(
                    text_input("Month of the Year", &race.date.month.to_string())
                        .on_input(Message::MonthChanged),
                )
                .spacing(10)
                .padding(10)
                .push(text("Year (YYYY):"))
                .spacing(10)
                .padding(10)
                .push(
                    text_input("Year", &race.date.year.to_string()).on_input(Message::YearChanged),
                )
                .spacing(10)
                .padding(10)
                .push(text("Session Number:"))
                .spacing(10)
                .padding(10)
                .push(
                    text_input("Session Number", &race.session_id.to_string())
                        .on_input(Message::SessionIdChanged),
                )
                .push(text("Race Position:"))
                .spacing(10)
                .padding(10)
                .push(
                    text_input("Race Position", &race.race_position.to_string())
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

            result_cards.push(Card::new(text(header), edit_race_contents));
        }

        result_cards
    }
}
