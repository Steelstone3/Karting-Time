use crate::{
    commands::messages::Message,
    controllers::driver_profile::time_parser::format_laptime,
    models::{application::karting_time::KartingTime, driver::race_result::Race},
    table::Table,
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
                    self.race_results_overview_table(&self.application_state.filtered_races),
                ))
                .padding(10)
                .spacing(10);

            column
        }
    }

    fn race_results_overview_table(&self, races: &Vec<Race>) -> Element<Message> {
        let mut table = Table::default();

        table.add_headers(vec![
            "Track Name".to_string(),
            "Date".to_string(),
            "Session".to_string(),
            "Car Used".to_string(),
            "Race Position".to_string(),
            "Fastest Lap".to_string(),
            "Average Lap 5".to_string(),
            "Average Lap 10".to_string(),
            "Average Lap 15".to_string(),
            "Total Lap 5".to_string(),
            "Total Lap 10".to_string(),
            "Total Lap 15".to_string(),
            "Total Time".to_string(),
        ]);

        for race in races {
            let total_times = race.calculate_total_times();
            let average_times = race.calculate_average_total_times(&total_times);

            table.add_row(vec![
                race.race_information.track_name.to_string(),
                race.race_information.date.to_string(),
                race.race_information.session_id.to_string(),
                race.race_information.car_used.to_string(),
                race.race_information.race_position.to_string(),
                format_laptime(race.get_fastest_lap()),
                Race::get_average_time(&average_times, &5),
                Race::get_average_time(&average_times, &10),
                Race::get_average_time(&average_times, &15),
                Race::get_total_time(&total_times, &5),
                Race::get_total_time(&total_times, &10),
                Race::get_total_time(&total_times, &15),
                Race::get_total_time(&total_times, &race.laptimes.len()),
            ]);
        }

        Table::build(table, self.theme().palette().text, None)
    }
}
