use crate::{
    commands::messages::Message,
    controllers::file::file_picker::{
        save_file_location, save_folder_location, select_file_to_load, select_files_to_load,
    },
    models::application::karting_time::KartingTime,
};
use iced::Task;

impl KartingTime {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::MenuBar => Task::none(),
            Message::SelectedTabChanged(tab_identifier) => {
                self.switch_tab(tab_identifier);
                Task::none()
            }
            Message::FileNew => {
                self.file_new();
                Task::none()
            }
            Message::ImportRacesRequested => {
                select_files_to_load().map(Message::ImportRacesCompleted)
            }
            Message::ImportRacesCompleted(file_paths) => {
                if let Some(file_paths) = file_paths {
                    self.import_races(file_paths);
                    self.driver_profile.sort_races();
                    self.driver_profile.update_filtering();
                }
                Task::none()
            }
            Message::ExportRacesRequested => {
                save_folder_location().map(Message::ExportRacesCompleted)
            }
            Message::ExportRacesCompleted(folder_location) => {
                if let Some(folder_location) = folder_location {
                    self.export_races(&folder_location)
                }
                Task::none()
            }
            Message::ExportHtmlRacesRequested => {
                save_folder_location().map(Message::ExportHtmlRacesCompleted)
            }
            Message::ExportHtmlRacesCompleted(folder_location) => {
                match folder_location {
                    Some(folder_location) => self.export_html_races(&folder_location),
                    None => todo!(),
                }
                Task::none()
            }
            Message::SaveApplicationRequested => {
                save_file_location().map(Message::SaveApplicationCompleted)
            }
            Message::SaveApplicationCompleted(file_path) => {
                if let Some(file_path) = file_path {
                    self.save_application(&file_path);
                }
                Task::none()
            }
            Message::LoadApplicationRequested => {
                select_file_to_load().map(Message::LoadApplicationCompleted)
            }
            Message::LoadApplicationCompleted(file_path) => {
                if let Some(file_path) = file_path {
                    self.load_application(&file_path);
                    self.driver_profile.sort_races();
                    self.driver_profile.update_filtering();
                }
                Task::none()
            }
            Message::ViewToggleTheme => {
                self.switch_theme();
                Task::none()
            }
            Message::ViewToggleFilter => {
                self.toggle_filter();
                self.driver_profile.update_filtering();
                Task::none()
            }
            Message::DriverNameChanged(name) => {
                self.driver_profile.name = name;
                Task::none()
            }
            Message::TrackNameChanged(track_name) => {
                self.driver_profile.new_race.race_information.track_name = track_name;

                self.driver_profile
                    .new_race
                    .race_information
                    .update_unique_identifier();
                Task::none()
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
                Task::none()
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
                Task::none()
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
                Task::none()
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
                Task::none()
            }
            Message::SessionTypeChanged(session_type) => {
                self.driver_profile.new_race.race_metadata.session_type = session_type;
                Task::none()
            }
            Message::TrackConditionsChanged(track_conditions) => {
                self.driver_profile.new_race.race_metadata.track_conditions = track_conditions;
                Task::none()
            }
            Message::RacePositionChanged(race_position) => {
                self.driver_profile
                    .new_race
                    .race_information
                    .session
                    .set_race_position(race_position);
                Task::none()
            }
            Message::CarUsedChanged(car_used) => {
                self.driver_profile.new_race.race_metadata.car_used = car_used;
                Task::none()
            }
            Message::ChampionshipChanged(championship) => {
                self.driver_profile.new_race.race_metadata.championship = championship;
                Task::none()
            }
            Message::NotesChanged(notes) => {
                self.driver_profile.new_race.race_metadata.notes = notes;
                Task::none()
            }
            Message::LaptimeEditor(action) => {
                self.application_state
                    .race_editor
                    .text_editor
                    .perform(action);
                Task::none()
            }
            Message::TrackFilterChanged(track_query) => {
                self.driver_profile.filter.track_query = track_query;

                self.driver_profile.update_filtering();
                Task::none()
            }
            Message::DateFilterChanged(date_query) => {
                self.driver_profile.filter.date_query = date_query;

                self.driver_profile.update_filtering();
                Task::none()
            }
            Message::CarUsedFilterChanged(car_used_query) => {
                self.driver_profile.filter.car_used_query = car_used_query;

                self.driver_profile.update_filtering();
                Task::none()
            }
            Message::ChampionshipFilterChanged(championship_query) => {
                self.driver_profile.filter.championship_query = championship_query;

                self.driver_profile.update_filtering();
                Task::none()
            }
            Message::SessionTypeFilterChanged(session_type_query) => {
                self.driver_profile.filter.session_type_query = session_type_query;

                self.driver_profile.update_filtering();
                Task::none()
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
                Task::none()
            }
            Message::ClearRaceEditorPressed => {
                self.application_state.race_editor.clear_text_editor();
                Task::none()
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
                Task::none()
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
                Task::none()
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
                Task::none()
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
                Task::none()
            }
        }
    }
}
