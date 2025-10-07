use crate::{
    commands::messages::Message,
    controllers::file::file_picker::{
        save_file_location, save_folder_location, select_file_to_load, select_files_to_load,
    },
    models::{application::karting_time::KartingTime, driver::driver_profile::DriverProfile},
};

impl KartingTime {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::MenuBar => {}
            Message::SelectedTabChanged(tab_identifier) => self.switch_tab(tab_identifier),
            Message::FileNew => self.file_new(),
            Message::ImportRaces => {
                self.import_races(select_files_to_load());
                DriverProfile::new_from_self(self.driver_profile.clone());
                self.driver_profile.sort_races();
                self.driver_profile.update_filtering();
            }
            Message::ExportRaces => self.export_races(&save_folder_location()),
            Message::ExportHtmlRaces => self.export_html_races(&save_folder_location()),
            Message::SaveApplication => self.save_application(&save_file_location()),
            Message::LoadApplication => {
                self.load_application(&select_file_to_load());
                DriverProfile::new_from_self(self.driver_profile.clone());
                self.driver_profile.sort_races();
                self.driver_profile.update_filtering();
            }
            Message::ViewToggleTheme => self.switch_theme(),
            Message::ViewToggleFilter => {
                self.toggle_filter();
                self.driver_profile.update_filtering();
            }
            Message::DriverNameChanged(name) => self.driver_profile.name = name,
            Message::TrackNameChanged(track_name) => {
                self.driver_profile.new_race.race_information.track_name = track_name;

                self.driver_profile
                    .new_race
                    .race_information
                    .update_unique_identifier();
            }
            Message::DayChanged(day) => {
                self.driver_profile
                    .new_race
                    .race_information
                    .date
                    .set_day(day);

                self.driver_profile
                    .new_race
                    .race_information
                    .update_unique_identifier();
            }
            Message::MonthChanged(month) => {
                self.driver_profile
                    .new_race
                    .race_information
                    .date
                    .set_month(month);

                self.driver_profile
                    .new_race
                    .race_information
                    .update_unique_identifier();
            }
            Message::YearChanged(year) => {
                self.driver_profile
                    .new_race
                    .race_information
                    .date
                    .set_year(year);

                self.driver_profile
                    .new_race
                    .race_information
                    .update_unique_identifier();
            }
            Message::SessionIdChanged(session_id) => {
                self.driver_profile
                    .new_race
                    .race_information
                    .session
                    .set_session_id(session_id);

                self.driver_profile
                    .new_race
                    .race_information
                    .update_unique_identifier();
            }
            Message::SessionTypeChanged(session_type) => {
                self.driver_profile.new_race.race_metadata.session_type = session_type;
            }
            Message::TrackConditionsChanged(track_conditions) => {
                self.driver_profile.new_race.race_metadata.track_conditions = track_conditions;
            }
            Message::RacePositionChanged(race_position) => {
                self.driver_profile
                    .new_race
                    .race_information
                    .session
                    .set_race_position(race_position);
            }
            Message::CarUsedChanged(car_used) => {
                self.driver_profile.new_race.race_metadata.car_used = car_used;
            }
            Message::ChampionshipChanged(championship) => {
                self.driver_profile.new_race.race_metadata.championship = championship;
            }
            Message::NotesChanged(notes) => {
                self.driver_profile.new_race.race_metadata.notes = notes;
            }
            Message::LaptimeEditor(action) => self
                .application_state
                .race_editor
                .text_editor
                .perform(action),
            Message::TrackFilterChanged(track_query) => {
                self.driver_profile.filter.track_query = track_query;

                self.driver_profile.update_filtering();
            }
            Message::DateFilterChanged(date_query) => {
                self.driver_profile.filter.date_query = date_query;

                self.driver_profile.update_filtering();
            }
            Message::CarUsedFilterChanged(car_used_query) => {
                self.driver_profile.filter.car_used_query = car_used_query;

                self.driver_profile.update_filtering();
            }
            Message::ChampionshipFilterChanged(championship_query) => {
                self.driver_profile.filter.championship_query = championship_query;

                self.driver_profile.update_filtering();
            }
            Message::SessionTypeFilterChanged(session_type_query) => {
                self.driver_profile.filter.session_type_query = session_type_query;

                self.driver_profile.update_filtering();
            }
            Message::UpdateRacesPressed => {
                self.driver_profile.new_race.convert_to_laps(
                    self.application_state
                        .race_editor
                        .get_text_from_text_editor(),
                );

                self.driver_profile.upsert_race();
                self.driver_profile.sort_races();
                self.driver_profile.update_filtering();
            }
            Message::ClearRaceEditorPressed => {
                self.application_state.race_editor.clear_text_editor();
            }
            Message::ReplacePressed(identifier) => {
                if let Some(race) = self
                    .driver_profile
                    .races
                    .iter_mut()
                    .find(|race| race.race_information.unique_race_identifier == identifier)
                {
                    self.driver_profile.new_race = race.clone();
                    self.application_state.race_editor.clear_text_editor();
                    self.application_state.race_editor.paste_laptimes(race);
                    self.driver_profile.update_filtering();
                }
            }
            Message::DeletePressed(identifier) => {
                if let Some(race) = self
                    .driver_profile
                    .races
                    .iter_mut()
                    .find(|race| race.race_information.unique_race_identifier == identifier)
                {
                    race.is_deleting = true;
                    self.driver_profile.update_filtering();
                }
            }
            Message::DeleteConfirmedPressed(identifier) => {
                if let Some(index) = self
                    .driver_profile
                    .races
                    .iter()
                    .position(|race| race.race_information.unique_race_identifier == identifier)
                {
                    self.driver_profile.races.remove(index);
                    self.driver_profile.update_filtering();
                }
            }
            Message::DeleteCancelledPressed(identifier) => {
                if let Some(race) = self
                    .driver_profile
                    .races
                    .iter_mut()
                    .find(|race| race.race_information.unique_race_identifier == identifier)
                {
                    race.is_deleting = false;
                    self.driver_profile.update_filtering();
                }
            }
        }
    }
}
