use crate::{
    controllers::driver_profile::time_parser::format_laptime, models::driver::race_result::Race,
};
use comfy_table::{presets::ASCII_MARKDOWN, Cell, Table};
use std::{collections::HashMap, fmt::Display};

impl Display for Race {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut table = Table::new();

        table
            // .load_preset(UTF8_FULL) // Use UTF8_FULL preset for solid lines
            .load_preset(ASCII_MARKDOWN)
            .set_header(vec![
                Cell::new("Lap"),      //.with_style(Attr::Bold),
                Cell::new("Time (s)"), //.with_style(Attr::Bold),
            ]);

        for laptime in &self.laptimes {
            table.add_row(vec![
                Cell::new(laptime.lap_number.to_string()),
                Cell::new(format_laptime(laptime.time)),
            ]);
        }

        write!(f, "{}", table)
    }
}

impl Race {
    pub fn display_race_results_overview(races: &Vec<Race>) -> String {
        let mut table = Race::create_table();

        for race in races {
            let total_times = race.calculate_total_times();
            let average_times = race.calculate_average_total_times(&total_times);

            table.add_row(vec![
                Cell::new(race.race_information.track_name.to_string()),
                Cell::new(race.race_information.date.to_string()),
                Cell::new(race.race_information.session_id.to_string()),
                Cell::new(race.race_information.race_position.to_string()),
                Cell::new(format_laptime(race.get_fastest_lap())),
                Race::get_average_time_cell(&average_times, &5),
                Race::get_average_time_cell(&average_times, &10),
                Race::get_average_time_cell(&average_times, &15),
                Race::get_total_time_cell(&total_times, &5),
                Race::get_total_time_cell(&total_times, &10),
                Race::get_total_time_cell(&total_times, &15),
            ]);
        }

        format!("{}", table)
    }

    fn create_table() -> Table {
        let mut table = Table::new();

        table
            // .load_preset(UTF8_FULL) // Use UTF8_FULL preset for solid lines
            .load_preset(ASCII_MARKDOWN)
            .set_header(vec![
                Cell::new("Track Name"),
                Cell::new("Date"),
                Cell::new("Session"),
                Cell::new("Race Position"),
                Cell::new("Fastest Lap"),
                Cell::new("Average Lap 5"),
                Cell::new("Average Lap 10"),
                Cell::new("Average Lap 15"),
                Cell::new("Total Lap 5"),
                Cell::new("Total Lap 10"),
                Cell::new("Total Lap 15"),
            ]);
        table
    }

    fn get_average_time_cell(average_times: &HashMap<usize, f32>, key: &usize) -> Cell {
        let not_applicable_cell = Cell::new("N/A".to_string());

        let average_time_cell = match average_times.get(key) {
            Some(average_time) => Cell::new(format_laptime(*average_time)),
            None => not_applicable_cell.clone(),
        };

        average_time_cell
    }

    fn get_total_time_cell(total_times: &HashMap<usize, f32>, key: &usize) -> Cell {
        let not_applicable_cell = Cell::new("N/A".to_string());

        let total_time_cell = match total_times.get(key) {
            Some(total_time) => Cell::new(format_laptime(*total_time)),
            None => not_applicable_cell.clone(),
        };

        total_time_cell
    }
}

#[cfg(test)]
mod display_race_results_should {
    use crate::models::{
        date::Date,
        driver::{lap::Lap, race_information::RaceInformation, race_result::Race},
    };

    #[test]
    fn display() {
        // Given
        let expected_display =
            "| Lap | Time (s) |\n|-----|----------|\n| 1   | 12.20    |\n| 2   | 12.40    |"
                .to_string();
        let race_result = Race {
            race_information: RaceInformation {
                track_name: "Brands Hatch".to_string(),
                date: Date {
                    day: 12,
                    month: 12,
                    year: 2025,
                },
                session_id: 1,
                race_position: 1,
            },
            laptimes: vec![
                Lap {
                    lap_number: 1,
                    time: 12.2,
                },
                Lap {
                    lap_number: 2,
                    time: 12.4,
                },
            ],
        };

        // When
        let display = race_result.to_string();

        // Then
        assert_eq!(expected_display, display);
    }

    #[test]
    #[ignore]
    fn display_race_results_overview() {}
}
