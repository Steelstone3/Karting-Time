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
                            ( &race.track_name ) " - " ( race.day ) "/" ( race.month ) "/" ( race.year ) 
                        }
                        table {
                        caption { "Session " ( race.session_id ) " position " ( race.race_position ) }
                            thead {
                                tr { th { "Lap" } th { "Time" } }
                            }
                            tbody {
                                @for (i, lt) in race.laptimes.iter().enumerate() {
                                    tr {
                                        td data-label="Lap" { ( i+1 ) }
                                        td data-label="Time" { ( lt ) }
                                    }
                                }
                            }
                        }
                    }
                    @if let Some(notes) = &race.notes {
                        p { strong { "Notes" } " " ( notes ) }
                    }
                }
            }
        }
    }
}