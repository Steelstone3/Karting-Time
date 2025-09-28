use crate::{
    data_models::driver_profile_file::DriverProfileFile,
    models::{
        application::{application_state::ApplicationState, karting_time::KartingTime},
        driver::race_result::Race,
    },
};
use maud::{DOCTYPE, Markup, html};

pub fn convert_to_html(driver_profile: &DriverProfileFile) -> Markup {
    let karting_time = KartingTime {
        application_state: ApplicationState {
            filtered_races: driver_profile.convert_to_driver_profile().races,
            ..Default::default()
        },
        ..Default::default()
    };

    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                link rel="stylesheet" href="https://cdn.simplecss.org/simple.css";
                meta charset="utf-8";
                meta name="viewport" content="width=device-width,initial-scale=1";
                title { "Race Results" }
            }
            body {
                h1 { ( &driver_profile.name ) }

                hr {}

                h2 { ( &driver_profile.name ) " Driver Profile Summary" }

                // Driver Profile Summary
                table {
                    thead {
                        tr { th { "Profile Summary" } th { "Driver Statistic" } }
                    }
                    tbody {
                        tr {
                            td data-label="Profile Summary" { "Races" }
                            td data-label="Driver Statistic" { ( karting_time.get_filtered_number_of_races() ) }
                        }
                        tr {
                            td data-label="Profile Summary" { "Wins" }
                            td data-label="Driver Statistic" { ( karting_time.get_filtered_number_of_wins() ) }
                        }
                        tr {
                            td data-label="Profile Summary" { "Podiums" }
                            td data-label="Driver Statistic" { ( karting_time.get_filtered_number_of_podiums() ) }
                        }
                        tr {
                            td data-label="Profile Summary" { "Top Fives" }
                            td data-label="Driver Statistic" { ( karting_time.get_filtered_number_of_top_fives() ) }
                        }
                        tr {
                            td data-label="Profile Summary" { "Top Tens" }
                            td data-label="Driver Statistic" { ( karting_time.get_filtered_number_of_top_tens() ) }
                        }
                        tr {
                            td data-label="Profile Summary" { "Unique Tracks" }
                            td data-label="Driver Statistic" { ( karting_time.get_filtered_number_of_unique_tracks() ) }
                        }
                        tr {
                            td data-label="Profile Summary" { "Unique Cars" }
                            td data-label="Driver Statistic" { ( karting_time.get_filtered_number_of_unique_cars() ) }
                        }
                    }
                }

                // Race Summary
                table {}

                @for race in &driver_profile.races {
                    // Races
                    h2 {
                        ( &race.track_name ) " Session: " ( &race.session_id ) " Date: " ( race.day ) "/" ( race.month ) "/" ( race.year )
                    }
                    h3 { "Race" }
                    table {
                        thead {
                            tr { th { "Lap" } th { "Time" } }
                        }
                        tbody {
                            @for (lap_number, lap_time) in race.laptimes.iter().enumerate() {
                                tr {
                                    td data-label="Lap" { ( lap_number + 1 ) }
                                    td data-label="Time" { ( lap_time ) }
                                }
                            }
                        }
                    }
                    // Summary
                    h3 { "Race Summary" }
                    table {
                        thead {
                            tr { th { "Race Summary" } th { "Race Statistic" } }
                        }
                        tbody {
                            tr {
                                td data-label="Race Summary" { "Race position" }
                                td data-label="Race Statistic" { ( race.race_position ) }
                            }
                            tr {
                                td data-label="Race Summary" { "Number of laps" }
                                td data-label="Race Statistic" { ( race.convert_to_race().get_number_of_laps() ) }
                            }
                            tr {
                                td data-label="Race Summary" { "Fastest lap" }
                                td data-label="Race Statistic" { ( race.convert_to_race().get_fastest_lap() ) }
                            }
                            tr {
                                td data-label="Race Summary" { "Average lap (105%)" }
                                td data-label="Race Statistic" { ( race.convert_to_race().get_average_lap() ) }
                            }
                        }
                    }
                    // Pace
                    h3 { "Race Pace" }
                    table {
                        thead {
                            tr { th { "Lap" } th { "Pace" } }
                        }
                        tbody {
                            @for (total_time_key, total_time_value) in Race::convert_hash_map(race.convert_to_race().calculate_total_times()) {
                                tr {
                                    td data-label="Lap" { "Total Time " (total_time_key) }
                                    td data-label="Pace" { (total_time_value) }
                                }
                            }
                            @for (average_time_key, average_time_value) in Race::convert_hash_map(race.convert_to_race().calculate_average_total_times(&race.convert_to_race().calculate_total_times())) {
                                tr {
                                    td data-label="Lap" { "Average Time " (average_time_key) }
                                    td data-label="Pace" { (average_time_value) }
                                }
                            }
                        }
                    }
                    // Metadata
                    h3 { "Metadata" }
                    table {
                        thead {
                            tr { th { "Metadata" } th { "Value" } }
                        }
                        tbody {
                            tr {
                                @if let Some(session_type) = &race.session_type {
                                    td data-label="Metadata" { "Session type" }
                                    td data-label="Value" { ( session_type ) }
                                }
                            }
                            tr {
                                @if let Some(track_conditions) = &race.track_conditions {
                                     td data-label="Metadata" { "Track condition" }
                                     td data-label="Value" { ( track_conditions ) }
                                }
                            }
                            tr {
                                @if let Some(car_used) = &race.car_used {
                                    td data-label="Metadata" { "Car used" }
                                    td data-label="Value" { ( car_used ) }
                                }
                            }
                            tr {
                                @if let Some(championship) = &race.championship {
                                    td data-label="Metadata" { "Championship" }
                                    td data-label="Value" { ( championship ) }
                                }
                            }
                        }
                    }
                    @if let Some(notes) = &race.notes {
                            p { strong { "Notes: " } ( notes ) }
                    }

                    hr {}
                }
            }
        }
    }
}

#[cfg(test)]
mod html_converter_should {
    use crate::{
        controllers::file::html_converter::convert_to_html,
        data_models::{driver_profile_file::DriverProfileFile, race_file::RaceFile},
    };

    #[test]
    fn convert() {
        // Given
        let driver_profile_file = DriverProfileFile {
            name: "Derek".to_string(),
            races: vec![RaceFile {
                laptimes: vec![
                    "5.0".to_string(),
                    "10.0".to_string(),
                    "15.0".to_string(),
                    "20.0".to_string(),
                    "25.0".to_string(),
                    "30.0".to_string(),
                ],
                day: 24,
                month: 12,
                year: 2025,
                track_name: "Three Brothers".to_string(),
                session_id: 1,
                race_position: 1,
                session_type: Some("Race".to_string()),
                track_conditions: Some("Dry".to_string()),
                car_used: Some("Mercedes GT3".to_string()),
                championship: Some("GT World Challenge".to_string()),
                notes: Some("No comment".to_string()),
            }],
        };

        // When
        let markdown = convert_to_html(&driver_profile_file);

        // Then
        let markdown_string = markdown.into_string();

        // Title
        assert!(markdown_string.contains("<title>Race Results</title"));

        // Driver Profile Heading
        assert!(
            markdown_string.contains(&format!("<h1>{}</h1>", &driver_profile_file.name.clone()))
        );

        // Driver Profile Summary Table
        assert!(markdown_string.contains(&format!(
            "<h2>{} Driver Profile Summary</h2>",
            &driver_profile_file.name
        )));
        assert!(markdown_string.contains(&format!(
            "<th>Profile Summary</th><th>Driver Statistic</th>"
        )));
        assert!(
            markdown_string.contains(&format!("<td data-label=\"Profile Summary\">Races</td>"))
        );
        assert!(markdown_string.contains(&format!("<td data-label=\"Driver Statistic\">1</td>")));
        assert!(markdown_string.contains(&format!("<td data-label=\"Profile Summary\">Wins</td>")));
        assert!(markdown_string.contains(&format!("<td data-label=\"Driver Statistic\">1</td>")));
        assert!(
            markdown_string.contains(&format!("<td data-label=\"Profile Summary\">Podiums</td>"))
        );
        assert!(markdown_string.contains(&format!("<td data-label=\"Driver Statistic\">1</td>")));
        assert!(markdown_string.contains(&format!(
            "<td data-label=\"Profile Summary\">Top Fives</td>"
        )));
        assert!(markdown_string.contains(&format!("<td data-label=\"Driver Statistic\">1</td>")));
        assert!(
            markdown_string.contains(&format!("<td data-label=\"Profile Summary\">Top Tens</td>"))
        );
        assert!(markdown_string.contains(&format!("<td data-label=\"Driver Statistic\">1</td>")));
        assert!(markdown_string.contains(&format!(
            "<td data-label=\"Profile Summary\">Unique Tracks</td>"
        )));
        assert!(markdown_string.contains(&format!("<td data-label=\"Driver Statistic\">1</td>")));
        assert!(markdown_string.contains(&format!(
            "<td data-label=\"Profile Summary\">Unique Cars</td>"
        )));
        assert!(markdown_string.contains(&format!("<td data-label=\"Driver Statistic\">1</td>")));

        for race in driver_profile_file.races {
            // Race Summary Table
            assert!(markdown_string.contains("<h3>Race Summary</h3>"));
            assert!(
                markdown_string.contains(&format!("<th>Race Summary</th><th>Race Statistic</th>"))
            );
            assert!(markdown_string.contains("<td data-label=\"Race Summary\">Race position</td>"));
            assert!(markdown_string.contains("<td data-label=\"Race Statistic\">1</td>"));

            assert!(
                markdown_string.contains("<td data-label=\"Race Summary\">Number of laps</td>")
            );
            assert!(markdown_string.contains("<td data-label=\"Race Statistic\">6</td>"));

            assert!(markdown_string.contains("<td data-label=\"Race Summary\">Fastest lap</td>"));
            assert!(markdown_string.contains("<td data-label=\"Race Statistic\">5</td>"));

            assert!(
                markdown_string.contains("<td data-label=\"Race Summary\">Average lap (105%)</td>")
            );
            assert!(markdown_string.contains("<td data-label=\"Race Statistic\">5</td>"));

            // Race Pace Table
            assert!(markdown_string.contains("<h3>Race Pace</h3>"));
            assert!(markdown_string.contains(&format!("<th>Lap</th><th>Pace</th>")));
            assert!(markdown_string.contains("<td data-label=\"Lap\">Total Time 5</td>"));
            assert!(markdown_string.contains("<td data-label=\"Lap\">Total Time 6</td>"));

            assert!(markdown_string.contains("<td data-label=\"Pace\">75</td>"));
            assert!(markdown_string.contains("<td data-label=\"Pace\">105</td>"));

            assert!(
                markdown_string.contains(&format!("<td data-label=\"Lap\">Average Time 5</td>"))
            );
            assert!(
                markdown_string.contains(&format!("<td data-label=\"Lap\">Average Time 6</td>"))
            );

            assert!(markdown_string.contains(&format!("<td data-label=\"Pace\">15</td>")));
            assert!(markdown_string.contains(&format!("<td data-label=\"Pace\">17.5</td>")));

            // Laptime Table
            assert!(
                markdown_string.contains(
                    &format!(
                        "<h2>{} Session: {} Date: {}/{}/{}</h2>",
                        race.track_name, race.session_id, race.day, race.month, race.year
                    )
                    .clone(),
                )
            );
            assert!(markdown_string.contains(&format!("<th>Lap</th><th>Time</th>")));
            assert!(markdown_string.contains("<td data-label=\"Lap\">1</td>"));
            assert!(markdown_string.contains("<td data-label=\"Lap\">2</td>"));
            assert!(markdown_string.contains("<td data-label=\"Lap\">3</td>"));
            assert!(markdown_string.contains("<td data-label=\"Lap\">4</td>"));
            assert!(markdown_string.contains("<td data-label=\"Lap\">5</td>"));
            assert!(markdown_string.contains("<td data-label=\"Lap\">6</td>"));
            assert!(markdown_string.contains(&format!(
                "<td data-label=\"Time\">{}</td>",
                race.laptimes[0].clone()
            )));
            assert!(markdown_string.contains(&format!(
                "<td data-label=\"Time\">{}</td>",
                race.laptimes[1].clone()
            )));
            assert!(markdown_string.contains(&format!(
                "<td data-label=\"Time\">{}</td>",
                race.laptimes[2].clone()
            )));
            assert!(markdown_string.contains(&format!(
                "<td data-label=\"Time\">{}</td>",
                race.laptimes[3].clone()
            )));
            assert!(markdown_string.contains(&format!(
                "<td data-label=\"Time\">{}</td>",
                race.laptimes[4].clone()
            )));
            assert!(markdown_string.contains(&format!(
                "<td data-label=\"Time\">{}</td>",
                race.laptimes[5].clone()
            )));

            // Race Metadata Table
            assert!(markdown_string.contains("<h3>Metadata</h3>"));
            assert!(markdown_string.contains(&format!("<th>Metadata</th><th>Value</th>")));
            assert!(markdown_string.contains("<td data-label=\"Metadata\">Session type</td>"));
            assert!(markdown_string.contains(&format!(
                "<td data-label=\"Value\">{}</td>",
                &race.session_type.unwrap_or_default()
            )));
            assert!(markdown_string.contains("<td data-label=\"Metadata\">Track condition</td>"));
            assert!(markdown_string.contains(&format!(
                "<td data-label=\"Value\">{}</td>",
                &race.track_conditions.unwrap_or_default()
            )));

            assert!(markdown_string.contains("<td data-label=\"Metadata\">Car used</td>"));
            assert!(markdown_string.contains(&format!(
                "<td data-label=\"Value\">{}</td>",
                &race.car_used.unwrap_or_default()
            )));

            assert!(markdown_string.contains("<td data-label=\"Metadata\">Championship</td>"));
            assert!(markdown_string.contains(&format!(
                "<td data-label=\"Value\">{}</td>",
                &race.championship.unwrap_or_default()
            )));

            assert!(markdown_string.contains(&format!(
                "<strong>Notes: </strong>{}",
                &race.notes.unwrap_or_default()
            )));
        }
    }
}
