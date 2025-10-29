use crate::{
    commands::messages::Message,
    models::{
        application::karting_time::KartingTime,
        driver::session_information::race_result::RaceResult,
    },
};
use iced::{
    Element,
    widget::{column, text},
};
use iced_aw::widgets::Card;
use iced_table::Table;

impl KartingTime {
    pub fn overview_race_results_view(&self) -> iced::widget::Column<'_, Message> {
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
                    self.race_results_overview_table(&self.driver_profile.filter.filtered_races),
                ))
                .padding(10)
                .spacing(10);

            column
        }
    }

    fn race_results_overview_table(&self, races: &Vec<RaceResult>) -> Element<'_, Message> {
        let mut table = Table::default();

        table.add_headers(vec![
            "Track Name",
            "Date",
            "Session",
            "Car Used",
            "Race Position",
            "Fastest Lap",
            "Average Lap 5",
            "Average Lap 10",
            "Average Lap 15",
            "Total Lap 5",
            "Total Lap 10",
            "Total Lap 15",
            "Total Time",
        ]);

        for race in races {
            table.add_row(vec![
                &race.race_information.track_name,
                &race.race_information.date.to_string(),
                &race.race_information.session.session_id.to_string(),
                &race.race_metadata.car_used,
                &race.race_information.session.race_position.to_string(),
                &race.race_statistics.fastest_lap,
                &RaceResult::get_time_by_key(&race.race_statistics.average_times_table, 5),
                &RaceResult::get_time_by_key(&race.race_statistics.average_times_table, 10),
                &RaceResult::get_time_by_key(&race.race_statistics.total_times_table, 15),
                &RaceResult::get_time_by_key(&race.race_statistics.total_times_table, 10),
                &RaceResult::get_time_by_key(&race.race_statistics.total_times_table, 15),
                &RaceResult::get_last_time(&race.race_statistics.total_times_table),
            ]);
        }

        Table::build(table, Some(self.theme().palette().text), None, None, None)
    }
}
