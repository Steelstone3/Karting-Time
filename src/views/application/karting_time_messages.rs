use crate::{
    commands::messages::Message,
    controllers::file::file_picker::{
        save_file_location, save_folder_location, select_file_to_load, select_files_to_load,
    },
    models::{application::karting_time::KartingTime, driver::race_information::RaceInformation},
};

impl KartingTime {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::MenuBar => {}
            Message::SelectedTabChanged(tab_identifier) => self.switch_tab(tab_identifier),
            Message::FileNew => self.file_new(),
            Message::ImportRaces => {
                self.import_race(select_files_to_load());
                self.driver_profile.sort_races();
                self.update_filtering();
            }
            Message::ExportRaces => self.export_races(&save_folder_location()),
            Message::SaveApplication => self.save_application(&save_file_location()),
            Message::LoadApplication => {
                self.load_application(&select_file_to_load());
                self.driver_profile.sort_races();
                self.update_filtering();
            }
            Message::ViewToggleTheme => self.switch_theme(),
            Message::ViewToggleFilter => {
                self.toggle_filter();
                self.update_filtering();
            }
            Message::DriverNameChanged(name) => self.driver_profile.name = name,
            Message::TrackNameChanged(track_name) => {
                self.application_state.new_race.race_information.track_name = track_name
            }
            Message::DayChanged(day) => {
                self.application_state
                    .new_race
                    .race_information
                    .date
                    .set_day(day);
            }
            Message::MonthChanged(month) => {
                self.application_state
                    .new_race
                    .race_information
                    .date
                    .set_month(month);
            }
            Message::YearChanged(year) => {
                self.application_state
                    .new_race
                    .race_information
                    .date
                    .set_year(year);
            }
            Message::SessionIdChanged(session_id) => {
                self.application_state
                    .new_race
                    .race_information
                    .session
                    .set_session_id(session_id);
            }
            Message::SessionTypeChanged(session_type) => {
                self.application_state
                    .new_race
                    .race_information
                    .session
                    .session_type = session_type;
            }
            Message::TrackConditionsChanged(session_condition) => {
                self.application_state
                    .new_race
                    .race_information
                    .session
                    .track_condition = session_condition;
            }
            Message::RacePositionChanged(race_position) => {
                self.application_state
                    .new_race
                    .race_information
                    .session
                    .set_race_position(race_position);
            }
            Message::CarUsedChanged(car_used) => {
                self.application_state.new_race.race_information.car_used = car_used;
            }
            Message::NotesChanged(notes) => {
                self.application_state.new_race.race_information.notes = notes;
            }
            Message::LaptimeEditor(action) => self
                .application_state
                .race_editor
                .text_editor
                .perform(action),
            Message::TrackFilterChanged(track_query) => {
                self.application_state.track_query = track_query;

                self.update_filtering();
            }
            Message::DateFilterChanged(date_query) => {
                self.application_state.date_query = date_query;

                self.update_filtering();
            }
            Message::CarUsedFilterChanged(car_used_query) => {
                self.application_state.car_used_query = car_used_query;

                self.update_filtering();
            }
            Message::UpdateRacesPressed => {
                self.application_state.new_race.convert_to_laps(
                    self.application_state
                        .race_editor
                        .get_text_from_text_editor(),
                );

                self.upsert_race();
                self.driver_profile.sort_races();
                self.update_filtering();
            }
            Message::ClearRaceEditorPressed => {
                self.application_state.race_editor.clear_text_editor();
            }
            Message::ReplacePressed(identifier) => {
                if let Some(race) = self.driver_profile.races.iter_mut().find(|race| {
                    RaceInformation::get_unique_race_information_identifier(&race.race_information)
                        == identifier
                }) {
                    self.application_state.new_race = race.clone();
                    self.application_state.race_editor.clear_text_editor();
                    self.application_state.race_editor.paste_laptimes(race);
                    self.update_filtering();
                }
            }
            Message::DeletePressed(identifier) => {
                if let Some(race) = self.driver_profile.races.iter_mut().find(|race| {
                    RaceInformation::get_unique_race_information_identifier(&race.race_information)
                        == identifier
                }) {
                    race.is_deleting = true;
                    self.update_filtering();
                }
            }
            Message::DeleteConfirmedPressed(identifier) => {
                if let Some(index) = self.driver_profile.races.iter().position(|race| {
                    RaceInformation::get_unique_race_information_identifier(&race.race_information)
                        == identifier
                }) {
                    self.driver_profile.races.remove(index);
                    self.update_filtering();
                }
            }
            Message::DeleteCancelledPressed(identifier) => {
                if let Some(race) = self.driver_profile.races.iter_mut().find(|race| {
                    RaceInformation::get_unique_race_information_identifier(&race.race_information)
                        == identifier
                }) {
                    race.is_deleting = false;
                    self.update_filtering();
                }
            }
        }
    }
}
