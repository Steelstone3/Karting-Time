use crate::{
    data_models::driver_profile_file::DriverProfileFile,
    models::driver::session_information::race_result::RaceResult,
};
use maud::{DOCTYPE, Markup, html};

pub fn convert_to_html(driver_profile: &DriverProfileFile) -> Markup {
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

                h2 { ( &driver_profile.name ) " Profile Summary" }

                // Driver Profile Summary
                table {
                    thead {
                        tr { th { "Profile Summary" } th { "Driver Statistic" } }
                    }
                    tbody {
                        tr {
                            td data-label="Profile Summary" { "Races" }
                            td data-label="Driver Statistic" { ( driver_profile.profile_statistics.races ) }
                        }
                        tr {
                            td data-label="Profile Summary" { "Wins" }
                            td data-label="Driver Statistic" { ( driver_profile.profile_statistics.wins ) }
                        }
                        tr {
                            td data-label="Profile Summary" { "Podiums" }
                            td data-label="Driver Statistic" { ( driver_profile.profile_statistics.podiums ) }
                        }
                        tr {
                            td data-label="Profile Summary" { "Top Fives" }
                            td data-label="Driver Statistic" { ( driver_profile.profile_statistics.top_5 ) }
                        }
                        tr {
                            td data-label="Profile Summary" { "Top Tens" }
                            td data-label="Driver Statistic" { ( driver_profile.profile_statistics.top_10 ) }
                        }
                        tr {
                            td data-label="Profile Summary" { "Unique Tracks" }
                            td data-label="Driver Statistic" { ( driver_profile.profile_statistics.unique_tracks ) }
                        }
                        tr {
                            td data-label="Profile Summary" { "Unique Cars" }
                            td data-label="Driver Statistic" { ( driver_profile.profile_statistics.unique_cars ) }
                        }
                    }
                }

                // Race Summary
                h2 { ( &driver_profile.name ) " Race Summary" }
                table {
                    thead {
                        tr {
                            th { "Track Name" }
                            th { "Date" }
                            th { "Session" }
                            th { "Car Used" }
                            th { "Race Position" }
                            th { "Fastest Lap" }
                            th { "Average Lap 5" }
                            th { "Average Lap 10" }
                            th { "Average Lap 15" }
                            th { "Total Lap 5" }
                            th { "Total Lap 10" }
                            th { "Total Lap 15" }
                            th { "Total Time" }
                        }
                    }
                        tbody {
                            @for race in &driver_profile.races {
                            tr {
                                td data-label="Track Name" { ( &race.track_name ) }
                                td data-label="Date" { ( &race.day ) "/" ( &race.month ) "/" ( &race.year ) }
                                td data-label="Session" { ( &race.session_id ) }
                                @if let Some(car_used) = &race.car_used {
                                    td data-label="Car Used" { ( car_used ) }
                                }
                                td data-label="Race Position" { ( &race.race_position ) }
                                td data-label="Fastest Lap" { ( race.race_statistics.fastest_lap ) }

                                td data-label="Average Lap 5" { ( RaceResult::get_time_by_key( &race.race_statistics.average_times_table, 5 ) ) }
                                td data-label="Average Lap 10" { ( RaceResult::get_time_by_key( &race.race_statistics.average_times_table, 10 ) ) }
                                td data-label="Average Lap 15" { ( RaceResult::get_time_by_key( &race.race_statistics.average_times_table, 15 ) ) }

                                td data-label="Total Lap 5" { ( RaceResult::get_time_by_key( &race.race_statistics.total_times_table, 5 ) ) }
                                td data-label="Total Lap 10" { ( RaceResult::get_time_by_key( &race.race_statistics.total_times_table, 10 ) ) }
                                td data-label="Total Lap 15" { ( RaceResult::get_time_by_key( &race.race_statistics.total_times_table, 15 ) ) }
                                td data-label="Total Time" { ( RaceResult::get_last_time( &race.race_statistics.total_times_table) ) }
                            }
                        }
                    }
                }

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
                            @for (lap_number, lap_time) in race.race_statistics.formatted_laps.iter().enumerate() {
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
                                td data-label="Race Statistic" { ( race.race_statistics.number_of_laps ) }
                            }
                            tr {
                                td data-label="Race Summary" { "Fastest lap" }
                                td data-label="Race Statistic" { ( race.race_statistics.fastest_lap ) }
                            }
                            tr {
                                td data-label="Race Summary" { "Average lap (105%)" }
                                td data-label="Race Statistic" { ( race.race_statistics.average_105_lap ) }
                            }
                        }
                    }
                    // Pace
                    h3 { "Race Pace" }
                    h4 { "Total Times" }
                    table {
                        thead {
                            tr { th { "Lap" } th { "Pace" } }
                        }
                        tbody {
                            @for (total_time_key, total_time_value) in &race.race_statistics.total_times_table {
                                tr {
                                    td data-label="Lap" { "Total Time " (total_time_key) }
                                    td data-label="Pace" { (total_time_value) }
                                }
                            }
                        }
                    }
                    h4 { "Average Times" }
                    table {
                        thead {
                            tr { th { "Lap" } th { "Pace" } }
                        }
                        tbody {
                            @for (average_time_key, average_time_value) in &race.race_statistics.average_times_table {
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
        models::{
            date::RaceDate,
            driver::session_information::{race_metadata::RaceMetadata, session::Session},
        },
    };

    #[test]
    fn convert_profile_summary_table() {
        // Given
        let driver_profile_file = driver_profile_file_test_fixture();

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

        // Profile Summary Table
        assert!(markdown_string.contains(&format!(
            "<h2>{} Profile Summary</h2>",
            &driver_profile_file.name
        )));
        assert!(markdown_string.contains("<th>Profile Summary</th><th>Driver Statistic</th>"));
        assert!(markdown_string.contains("<td data-label=\"Profile Summary\">Races</td>"));
        assert!(markdown_string.contains("<td data-label=\"Driver Statistic\">1</td>"));
        assert!(markdown_string.contains("<td data-label=\"Profile Summary\">Wins</td>"));
        assert!(markdown_string.contains("<td data-label=\"Driver Statistic\">1</td>"));
        assert!(markdown_string.contains("<td data-label=\"Profile Summary\">Podiums</td>"));
        assert!(markdown_string.contains("<td data-label=\"Driver Statistic\">1</td>"));
        assert!(markdown_string.contains("<td data-label=\"Profile Summary\">Top Fives</td>"));
        assert!(markdown_string.contains("<td data-label=\"Driver Statistic\">1</td>"));
        assert!(markdown_string.contains("<td data-label=\"Profile Summary\">Top Tens</td>"));
        assert!(markdown_string.contains("<td data-label=\"Driver Statistic\">1</td>"));
        assert!(markdown_string.contains("<td data-label=\"Profile Summary\">Unique Tracks</td>"));
        assert!(markdown_string.contains("<td data-label=\"Driver Statistic\">1</td>"));
        assert!(markdown_string.contains("<td data-label=\"Profile Summary\">Unique Cars</td>"));
        assert!(markdown_string.contains("<td data-label=\"Driver Statistic\">1</td>"));
    }

    #[test]
    fn convert_profile_races_summary_table() {
        // Given
        let driver_profile_file = driver_profile_file_test_fixture();

        // When
        let markdown = convert_to_html(&driver_profile_file);

        // Then
        let markdown_string = markdown.into_string();

        // Title
        assert!(markdown_string.contains("<title>Race Results</title"));

        // Profile Summary Table
        assert!(markdown_string.contains(&format!(
            "<h2>{} Race Summary</h2>",
            &driver_profile_file.name
        )));

        assert!(markdown_string.contains("<th>Track Name</th><th>Date</th><th>Session</th><th>Car Used</th><th>Race Position</th><th>Fastest Lap</th><th>Average Lap 5</th><th>Average Lap 10</th><th>Average Lap 15</th><th>Total Lap 5</th><th>Total Lap 10</th><th>Total Lap 15</th><th>Total Time</th>"));

        for race in driver_profile_file.races {
            assert!(markdown_string.contains(&format!(
                "<td data-label=\"Track Name\">{}</td>",
                &race.track_name
            )));
            assert!(markdown_string.contains(&format!(
                "<td data-label=\"Date\">{}/{}/{}</td>",
                &race.day, &race.month, &race.year
            )));
            assert!(markdown_string.contains(&format!(
                "<td data-label=\"Session\">{}</td>",
                &race.session_id
            )));
            assert!(markdown_string.contains(&format!(
                "<td data-label=\"Car Used\">{}</td>",
                &race.car_used.unwrap_or_default()
            )));
            assert!(markdown_string.contains(&format!(
                "<td data-label=\"Race Position\">{}</td>",
                &race.race_position
            )));
            assert!(markdown_string.contains("<td data-label=\"Fastest Lap\">5.00</td>",));
            assert!(markdown_string.contains("<td data-label=\"Average Lap 5\">15.00</td>",));
            assert!(markdown_string.contains("<td data-label=\"Average Lap 10\">N/A</td>",));
            assert!(markdown_string.contains("<td data-label=\"Average Lap 15\">N/A</td>",));
            assert!(markdown_string.contains("<td data-label=\"Total Lap 5\">1:15.00</td>",));
            assert!(markdown_string.contains("<td data-label=\"Total Lap 10\">N/A</td>",));
            assert!(markdown_string.contains("<td data-label=\"Total Lap 15\">N/A</td>",));
            assert!(markdown_string.contains("<td data-label=\"Total Time\">1:45.00</td>",));
        }
    }

    #[test]
    fn convert_race_summary_table() {
        // Given
        let driver_profile_file = driver_profile_file_test_fixture();

        // When
        let markdown = convert_to_html(&driver_profile_file);

        // Then
        let markdown_string = markdown.into_string();

        for _ in driver_profile_file.races {
            // Race Summary Table
            assert!(markdown_string.contains("<h3>Race Summary</h3>"));
            assert!(markdown_string.contains("<th>Race Summary</th><th>Race Statistic</th>"));
            assert!(markdown_string.contains("<td data-label=\"Race Summary\">Race position</td>"));
            assert!(markdown_string.contains("<td data-label=\"Race Statistic\">1</td>"));

            assert!(
                markdown_string.contains("<td data-label=\"Race Summary\">Number of laps</td>")
            );
            assert!(markdown_string.contains("<td data-label=\"Race Statistic\">6</td>"));

            assert!(markdown_string.contains("<td data-label=\"Race Summary\">Fastest lap</td>"));
            assert!(markdown_string.contains("<td data-label=\"Race Statistic\">5.00</td>"));

            assert!(
                markdown_string.contains("<td data-label=\"Race Summary\">Average lap (105%)</td>")
            );
            assert!(markdown_string.contains("<td data-label=\"Race Statistic\">5.00</td>"));
        }
    }

    #[test]
    fn convert_race_pace_total_times_table() {
        // Given
        let driver_profile_file = driver_profile_file_test_fixture();

        // When
        let markdown = convert_to_html(&driver_profile_file);

        // Then
        let markdown_string = markdown.into_string();

        for _ in driver_profile_file.races {
            // Race Pace Table
            assert!(markdown_string.contains("<h3>Race Pace</h3>"));

            assert!(markdown_string.contains("<h4>Total Times</h4>"));
            assert!(markdown_string.contains("<th>Lap</th><th>Pace</th>"));
            assert!(markdown_string.contains("<td data-label=\"Lap\">Total Time 5</td>"));
            assert!(markdown_string.contains("<td data-label=\"Lap\">Total Time 6</td>"));

            assert!(markdown_string.contains("<td data-label=\"Pace\">1:15.00</td>"));
            assert!(markdown_string.contains("<td data-label=\"Pace\">1:45.00</td>"));
        }
    }

    #[test]
    fn convert_race_pace_average_times_table() {
        // Given
        let driver_profile_file = driver_profile_file_test_fixture();

        // When
        let markdown = convert_to_html(&driver_profile_file);

        // Then
        let markdown_string = markdown.into_string();

        for _ in driver_profile_file.races {
            // Race Pace Table
            assert!(markdown_string.contains("<h3>Race Pace</h3>"));

            assert!(markdown_string.contains("<h4>Average Times</h4>"));
            assert!(markdown_string.contains("<th>Lap</th><th>Pace</th>"));
            assert!(markdown_string.contains("<td data-label=\"Lap\">Average Time 5</td>"));
            assert!(markdown_string.contains("<td data-label=\"Lap\">Average Time 6</td>"));

            assert!(markdown_string.contains("<td data-label=\"Pace\">15.00</td>"));
            assert!(markdown_string.contains("<td data-label=\"Pace\">17.50</td>"));
        }
    }

    #[test]
    fn convert_laptime_table() {
        // Given
        let driver_profile_file = driver_profile_file_test_fixture();

        // When
        let markdown = convert_to_html(&driver_profile_file);

        // Then
        let markdown_string = markdown.into_string();

        for race in driver_profile_file.races {
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
            assert!(markdown_string.contains("<th>Lap</th><th>Time</th>"));
            assert!(markdown_string.contains("<td data-label=\"Lap\">1</td>"));
            assert!(markdown_string.contains("<td data-label=\"Lap\">2</td>"));
            assert!(markdown_string.contains("<td data-label=\"Lap\">3</td>"));
            assert!(markdown_string.contains("<td data-label=\"Lap\">4</td>"));
            assert!(markdown_string.contains("<td data-label=\"Lap\">5</td>"));
            assert!(markdown_string.contains("<td data-label=\"Lap\">6</td>"));
            assert!(markdown_string.contains(&format!(
                "<td data-label=\"Time\">{}</td>",
                race.race_statistics.formatted_laps[0].clone()
            )));
            assert!(markdown_string.contains(&format!(
                "<td data-label=\"Time\">{}</td>",
                race.race_statistics.formatted_laps[1].clone()
            )));
            assert!(markdown_string.contains(&format!(
                "<td data-label=\"Time\">{}</td>",
                race.race_statistics.formatted_laps[2].clone()
            )));
            assert!(markdown_string.contains(&format!(
                "<td data-label=\"Time\">{}</td>",
                race.race_statistics.formatted_laps[3].clone()
            )));
            assert!(markdown_string.contains(&format!(
                "<td data-label=\"Time\">{}</td>",
                race.race_statistics.formatted_laps[4].clone()
            )));
            assert!(markdown_string.contains(&format!(
                "<td data-label=\"Time\">{}</td>",
                race.race_statistics.formatted_laps[5].clone()
            )));
        }
    }

    #[test]
    fn convert_metadata_table() {
        // Given
        let driver_profile_file = driver_profile_file_test_fixture();

        // When
        let markdown = convert_to_html(&driver_profile_file);

        // Then
        let markdown_string = markdown.into_string();

        for race in driver_profile_file.races {
            // Race Metadata Table
            assert!(markdown_string.contains("<h3>Metadata</h3>"));
            assert!(markdown_string.contains("<th>Metadata</th><th>Value</th>"));
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

    fn driver_profile_file_test_fixture() -> DriverProfileFile {
        DriverProfileFile::new(
            "Derek",
            vec![RaceFile::new(
                "Three Brothers",
                vec![
                    "5.0".to_string(),
                    "10.0".to_string(),
                    "15.0".to_string(),
                    "20.0".to_string(),
                    "25.0".to_string(),
                    "30.0".to_string(),
                ],
                RaceMetadata::new(
                    "Race",
                    "Dry",
                    "Mercedes GT3",
                    "GT World Challenge",
                    "No comment",
                ),
                Session::new(1, 1),
                RaceDate::new(24, 12, 2025),
            )],
        )
    }
}
