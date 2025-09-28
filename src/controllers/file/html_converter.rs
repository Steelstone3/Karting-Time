use crate::{
    data_models::driver_profile_file::DriverProfileFile, models::driver::race_result::Race,
};
use maud::{DOCTYPE, Markup, html};

// TODO Test
pub fn convert_to_html(driver_profile: &DriverProfileFile) -> Markup {
    html! {
        (DOCTYPE)
        html lang="en" {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width,initial-scale=1";
                title { "Race results" }
                style { r#"
                    body{font-family:system-ui,Segoe UI,Roboto,Arial}
                    table{width:100%;border-collapse:collapse}
                    th,td{padding:.5rem;border:1px solid #ddd;text-align:left}
                    @media(max-width:640px){
                        table,thead,tbody,tr{display:block}
                        th{display:none}
                        td{display:flex;justify-content:space-between;border:none;border-bottom:1px solid #eee}
                        td::before{content:attr(data-label);font-weight:600}
                        }
                        "# 
                }
            }
            body {
                h1 { ( &driver_profile.name ) }

                hr {}

                @for race in &driver_profile.races {
                    section {
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
                        h3 { "Summary" }
                        table {
                            thead {
                                tr { th { "Summary" } th { "Value" } }
                            }
                            tbody {
                                tr {
                                    td data-label="Summary" { "Race position" }
                                    td data-label="Value" { ( race.race_position ) }
                                }
                                tr {
                                    td data-label="Summary" { "Number of laps" }
                                    td data-label="Value" { ( race.convert_to_race().get_number_of_laps() ) }
                                }
                                tr {
                                    td data-label="Summary" { "Fastest lap" }
                                    td data-label="Value" { ( race.convert_to_race().get_fastest_lap() ) }
                                }
                                tr {
                                    td data-label="Summary" { "Average lap (105%)" }
                                    td data-label="Value" { ( race.convert_to_race().get_average_lap() ) }
                                }
                            }
                        }
                        h3 { "Race Pace" }
                        table {
                            thead {
                                tr { th { "Race Pace" } th { "Value" } }
                            }
                            tbody {
                                @for (total_time_key, total_time_value) in Race::convert_hash_map(race.convert_to_race().calculate_total_times()) {
                                    tr {
                                        td data-label="Race Pace" { "Total Time " (total_time_key) }
                                        td data-label="Value" { (total_time_value) }
                                    }
                                }

                                @for (average_time_key, average_time_value) in Race::convert_hash_map(race.convert_to_race().calculate_average_total_times(&race.convert_to_race().calculate_total_times())) {
                                    tr {
                                        td data-label="Race Pace" { "Average Time " (average_time_key) }
                                        td data-label="Value" { (average_time_value) }
                                    }
                                }
                            }
                        }
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
}
