use crate::{
    commands::messages::Message,
    controllers::file::file_picker::{
        save_file_location, save_folder_location, select_file_to_load, select_files_to_load,
    },
    models::application::karting_time::KartingTime,
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
                self.apply_filters();
            }
            Message::ExportRaces => self.export_races(&save_folder_location()),
            Message::SaveApplication => self.save_application(&save_file_location()),
            Message::LoadApplication => {
                self.load_application(&select_file_to_load());
                self.driver_profile.sort_races();
                self.apply_filters();
            }
            Message::ViewToggleTheme => self.switch_theme(),
            Message::ViewToggleFilter => {
                self.toggle_filter();
                self.apply_filters();
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
                    .set_session_id(session_id);
            }
            Message::RacePositionChanged(race_position) => {
                self.application_state
                    .new_race
                    .race_information
                    .set_race_position(race_position);
            }
            Message::LaptimeEditor(action) => self
                .application_state
                .race_editor
                .text_editor
                .perform(action),
            Message::TrackFilterChanged(track_query) => {
                self.application_state.track_query = track_query;

                self.apply_filters();
            }
            Message::DateFilterChanged(date_query) => {
                self.application_state.date_query = date_query;

                self.apply_filters();
            }
            Message::UpdateRacesPressed => {
                self.application_state.new_race.convert_to_laps(
                    self.application_state
                        .race_editor
                        .get_text_from_text_editor(),
                );
                if self
                    .application_state
                    .new_race
                    .is_unique_identifer(&self.driver_profile.races)
                {
                    self.driver_profile
                        .races
                        .push(self.application_state.new_race.clone());
                } else {
                    self.driver_profile.races = self
                        .application_state
                        .new_race
                        .replace_existing_race(&self.driver_profile.races);
                }

                self.driver_profile.sort_races();
                self.apply_filters();
            }
            Message::ClearRaceEditorPressed => {
                self.application_state.race_editor.clear_text_editor();
            }
            Message::ReplacePressed(index) => {
                if let Some(race) = self.driver_profile.races.get(index) {
                    self.application_state.new_race = race.clone();
                    self.application_state.race_editor.clear_text_editor();
                    self.application_state.race_editor.paste_laptimes(race);
                    self.apply_filters();
                }
            }
        }
    }
}
