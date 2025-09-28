use crate::data_models::driver_profile_file::DriverProfileFile;
use maud::{DOCTYPE, Markup, html};

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
                @for race in &driver_profile.races {
                    section {
                        h2 {
                            ( &race.track_name ) " Session: " ( &race.session_id ) " Date: " ( race.day ) "/" ( race.month ) "/" ( race.year )
                        }
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
                                tr { th { "Race Statistic" } th { "Pace" } }
                            }
                            tbody {
                                // tr { td data-label="Race position" { ( race.race_position ) } }
                                // tr { td data-label="Number of laps" { ( race.convert_to_race().get_number_of_laps() ) } }
                                // tr { td data-label="Fastest lap" { ( race.convert_to_race().get_fastest_lap() ) } }
                                // tr { td data-label="Average lap (105%)" { ( race.convert_to_race().get_average_lap() ) } }
                            }
                        }
                    }

                    h3 { "Metadata" }
                    section {
                        @if let Some(session_type) = &race.session_type {
                            p { strong { "Session type: " } ( session_type ) }
                        }

                        @if let Some(track_conditions) = &race.track_conditions {
                            p { strong { "Track condition: " } ( track_conditions ) }
                        }

                        @if let Some(car_used) = &race.car_used {
                            p { strong { "Car used: " } ( car_used ) }
                        }

                        @if let Some(championship) = &race.championship {
                            p { strong { "Championship: " } ( championship ) }
                        }

                        @if let Some(notes) = &race.notes {
                            p { strong { "Notes: " } ( notes ) }
                        }
                    }
                }
            }
        }
    }
}
