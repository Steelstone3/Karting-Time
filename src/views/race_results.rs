use crate::{commands::messages::Message, models::application::karting_time::KartingTime};
use iced::{
    widget::{button, column, text},
    Renderer, Theme,
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

        // TODO sort by track name ascending, session ID ascending then date descending
        // i.e 2025-02-10 will be higher in the order than 2004-03-15
        // i.e Brands Hatch will be higher in the order than Oulton Park
        // i.e Brands Hatch Session 1 will be higher in the order than Brands Hatch Session 2
        // i.e Brands Hatch Session 1 and 2 2025-02-10 will be higher in the order than Donnington Park Session 1 2024-02-10

        let mut index = 0;

        for race in &self.driver_profile.races {
            let header = format!(
                "{} Session: {} Date: {}",
                race.track_name, race.session_id, race.date
            );

            let race_summary = format!(
                "Number of laps: {}\nFastest lap: {:.2}\nAverage lap (105%): {:.2}\n\nRace Pace:\n{}\n{}",
                race.get_number_of_laps(),
                race.get_fastest_lap(),
                race.get_average_lap(),
                race.convert_total_times_to_string(),
                race.convert_average_total_times_to_string()
            )
            .to_string();

            let footer = column!()
                .push(text(race_summary))
                .spacing(10)
                .padding(10)
                .push(button("Replace").on_press(Message::ReplacePressed(index)));

            result_cards.push(Card::new(text(header), text(race.to_string())).foot(footer));

            index += 1;
        }

        result_cards
    }
}
