use crate::{
    commands::messages::Message,
    models::application::karting_time::KartingTime,
    views::application::file_picker::{
        save_folder_location, save_toml_file_location, select_file_to_load,
        select_json_file_to_load, select_toml_file_to_load, select_toml_files_to_load,
    },
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
            Message::SaveApplicationRequested => {
                save_toml_file_location().map(Message::SaveApplicationCompleted)
            }
            Message::SaveApplicationCompleted(file_path) => {
                if let Some(file_path) = file_path {
                    self.save_application(&file_path);
                }
                Task::none()
            }
            Message::LoadApplicationRequested => {
                select_toml_file_to_load().map(Message::LoadApplicationCompleted)
            }
            Message::LoadApplicationCompleted(file_path) => {
                if let Some(file_path) = file_path {
                    self.load_application(&file_path);
                    self.driver_profile.sort_races();
                    self.driver_profile.update_filtering();
                    self.driver_profile.filter.update_pagination();
                }
                Task::none()
            }
            Message::ImportRacesRequested => {
                select_toml_files_to_load().map(Message::ImportRacesCompleted)
            }
            Message::ImportRacesCompleted(file_paths) => {
                if let Some(file_paths) = file_paths {
                    self.import_races(file_paths);
                    self.driver_profile.sort_races();
                    self.driver_profile.update_filtering();
                    self.driver_profile.filter.update_pagination();
                }
                Task::none()
            }
            Message::ImportAccLaptimesFileRequested => {
                select_json_file_to_load().map(Message::ImportAccLaptimesFileCompleted)
            }
            Message::ImportAccLaptimesFileCompleted(file_path) => {
                if let Some(file_path) = file_path {
                    self.import_acc_laptimes(&file_path);
                    self.driver_profile.sort_races();
                    self.driver_profile.update_filtering();
                    self.driver_profile.filter.update_pagination();
                }
                Task::none()
            }
            Message::ImportLaptimesFileRequested => {
                select_file_to_load().map(Message::ImportLaptimesFileCompleted)
            }
            Message::ImportLaptimesFileCompleted(file_path) => {
                if let Some(file_path) = file_path {
                    self.import_laptimes(&file_path);
                    self.driver_profile.sort_races();
                    self.driver_profile.update_filtering();
                    self.driver_profile.filter.update_pagination();
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
                if let Some(folder_location) = folder_location {
                    self.export_html_races(&folder_location)
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
                self.driver_profile.filter.update_pagination();
                Task::none()
            }
            Message::PaginationNext => {
                self.driver_profile.filter.next_page();
                Task::none()
            }
            Message::PaginationPrevious => {
                self.driver_profile.filter.previous_page();
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
                self.driver_profile.filter.update_pagination();
                Task::none()
            }
            Message::DateFilterChanged(date_query) => {
                self.driver_profile.filter.date_query = date_query;

                self.driver_profile.update_filtering();
                self.driver_profile.filter.update_pagination();
                Task::none()
            }
            Message::CarUsedFilterChanged(car_used_query) => {
                self.driver_profile.filter.car_used_query = car_used_query;

                self.driver_profile.update_filtering();
                self.driver_profile.filter.update_pagination();
                Task::none()
            }
            Message::ChampionshipFilterChanged(championship_query) => {
                self.driver_profile.filter.championship_query = championship_query;

                self.driver_profile.update_filtering();
                self.driver_profile.filter.update_pagination();
                Task::none()
            }
            Message::SessionTypeFilterChanged(session_type_query) => {
                self.driver_profile.filter.session_type_query = session_type_query;

                self.driver_profile.update_filtering();
                self.driver_profile.filter.update_pagination();
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
                self.driver_profile.filter.update_pagination();
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
                    self.driver_profile.filter.update_pagination();
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
                    self.driver_profile.filter.update_pagination();
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
                    self.driver_profile.filter.update_pagination();
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
                    self.driver_profile.filter.update_pagination();
                }
                Task::none()
            }
        }
    }
}

#[cfg(test)]
mod karting_time_messages_should {
    use super::*;
    use crate::{
        commands::tab_identifiers::TabIdentifier::{self},
        controllers::file::test_file_guard::TestFileGuard,
        models::{
            date::RaceDate,
            driver::{
                driver_profile::DriverProfile,
                session_information::{
                    lap::Lap, race_information::RaceInformation, race_metadata::RaceMetadata,
                    race_result::RaceResult, session::Session,
                },
            },
        },
    };
    use iced::widget::text_editor::{Action, Edit};

    #[test]
    fn test_menu_bar() {
        // Given
        let mut karting_time = KartingTime::default();

        // When
        let task = karting_time.update(Message::MenuBar);

        // Then
        pretty_assertions::assert_eq!(0, task.units());
    }

    #[test]
    fn test_selected_tab_changed() {
        // Given
        let mut karting_time = KartingTime::default();

        // When
        let task = karting_time.update(Message::SelectedTabChanged(TabIdentifier::Results));

        // Then
        pretty_assertions::assert_eq!(
            TabIdentifier::Results,
            karting_time.application_state.tab_identifier
        );
        pretty_assertions::assert_eq!(0, task.units());
    }

    #[test]
    fn test_file_new() {
        // Given
        let race_result = RaceResult::new(
            RaceInformation::new(
                "Silverstone",
                RaceDate::new(21, 12, 2022),
                Session::new(1, 6),
            ),
            RaceMetadata::default(),
            vec![
                Lap::new(1, 23.6),
                Lap::new(2, 24.6),
                Lap::new(3, 25.4),
                Lap::new(4, 26.7),
            ],
        );
        let mut karting_time = KartingTime::new(DriverProfile::new(
            "Derek",
            vec![
                race_result.clone(),
                race_result.clone(),
                race_result.clone(),
                race_result.clone(),
                race_result.clone(),
                race_result.clone(),
                race_result.clone(),
                race_result.clone(),
                race_result.clone(),
                race_result.clone(),
                race_result.clone(),
                race_result.clone(),
                race_result.clone(),
                race_result.clone(),
                race_result.clone(),
                race_result.clone(),
                race_result.clone(),
            ],
        ));

        // When
        let task = karting_time.update(Message::FileNew);

        // Then
        pretty_assertions::assert_eq!(KartingTime::default(), karting_time);
        pretty_assertions::assert_eq!(0, task.units());
    }

    #[test]
    fn acceptance_test_save_application() {
        // Given
        let file_name = "karting_time_test_file_3.toml";
        let mut karting_time = KartingTime::default();

        // When
        let task_1 = karting_time.update(Message::SaveApplicationRequested);
        let task_2 = karting_time.update(Message::SaveApplicationCompleted(Some(
            file_name.to_string(),
        )));

        // Then
        assert!(std::path::Path::new(file_name).is_file());
        pretty_assertions::assert_eq!(1, task_1.units());
        pretty_assertions::assert_eq!(0, task_2.units());

        // Cleanup
        let _guard = TestFileGuard::new(file_name);
    }

    #[test]
    fn acceptance_test_load_application() {
        // Given
        let file_name = "./file_io_test_files/karting_time_application_state.toml";
        let driver_profile = DriverProfile::new(
            "Jack Jackson",
            vec![
                RaceResult::new(
                    RaceInformation::new(
                        "Three Sisters",
                        RaceDate::new(12, 12, 2025),
                        Session::new(1, 1),
                    ),
                    RaceMetadata::new(
                        Default::default(),
                        Default::default(),
                        "Kart",
                        "Championship",
                        Default::default(),
                    ),
                    vec![Lap::new(1, 50.4), Lap::new(2, 55.5)],
                ),
                RaceResult::new(
                    RaceInformation::new(
                        "Trafford Park",
                        RaceDate::new(15, 1, 2024),
                        Session::new(2, 3),
                    ),
                    RaceMetadata::new(
                        Default::default(),
                        Default::default(),
                        "Kart",
                        "Championship",
                        Default::default(),
                    ),
                    vec![Lap::new(1, 56.8), Lap::new(2, 58.7)],
                ),
            ],
        );
        let expected = KartingTime::new(driver_profile.clone());
        let mut karting_time = KartingTime::default();

        // When
        let task_1 = karting_time.update(Message::LoadApplicationRequested);
        let task_2 = karting_time.update(Message::LoadApplicationCompleted(Some(
            file_name.to_string(),
        )));

        // Then
        assert!(std::path::Path::new(file_name).is_file());
        pretty_assertions::assert_eq!(expected, karting_time);
        pretty_assertions::assert_eq!(1, task_1.units());
        pretty_assertions::assert_eq!(0, task_2.units());
    }

    #[test]
    fn acceptance_test_load_races() {
        // Given
        let expected = KartingTime::new(DriverProfile::new(
            "Racer",
            vec![
                RaceResult::new(
                    RaceInformation::new(
                        "Three Sisters",
                        RaceDate::new(12, 12, 2025),
                        Session::new(1, 1),
                    ),
                    RaceMetadata::new(
                        Default::default(),
                        Default::default(),
                        "Kart",
                        "Championship",
                        "",
                    ),
                    vec![Lap::new(1, 50.4), Lap::new(2, 55.5)],
                ),
                RaceResult::new(
                    RaceInformation::new(
                        "Trafford Park",
                        RaceDate::new(15, 1, 2024),
                        Session::new(2, 3),
                    ),
                    RaceMetadata::new(
                        Default::default(),
                        Default::default(),
                        "Kart",
                        "Championship",
                        Default::default(),
                    ),
                    vec![Lap::new(1, 56.8), Lap::new(2, 58.7)],
                ),
            ],
        ));
        let mut karting_time = KartingTime::default();
        let file_name_1 = "./file_io_test_files/Date_2025-12-12_Track_Three Sisters_Session_1.toml";
        let file_name_2 = "./file_io_test_files/Date_2024-1-15_Track_Trafford Park_Session_2.toml";

        // When
        let task_1 = karting_time.update(Message::ImportRacesRequested);
        let task_2 = karting_time.update(Message::ImportRacesCompleted(Some(vec![
            file_name_1.to_string(),
            file_name_2.to_string(),
        ])));

        // Then
        assert!(std::path::Path::new(file_name_1).is_file());
        assert!(std::path::Path::new(file_name_2).is_file());
        pretty_assertions::assert_eq!(expected, karting_time);
        pretty_assertions::assert_eq!(1, task_1.units());
        pretty_assertions::assert_eq!(0, task_2.units());
    }

    #[test]
    fn acceptance_test_import_acc_laptime_file() {
        // Given
        let expected = KartingTime::new(DriverProfile::new(
            "Racer",
            vec![RaceResult::new(
                RaceInformation::new("silverstone", RaceDate::today(), Session::new(1001, 1)),
                RaceMetadata::new("FP", Default::default(), "N/A", "", "Imported from ACC"),
                vec![
                    Lap::new(1, 122.505),
                    Lap::new(2, 122.147),
                    Lap::new(3, 121.615),
                    Lap::new(4, 121.1),
                    Lap::new(5, 121.935),
                    Lap::new(6, 123.527),
                    Lap::new(7, 122.215),
                    Lap::new(8, 121.702),
                    Lap::new(9, 122.18),
                    Lap::new(10, 121.297),
                    Lap::new(11, 120.785),
                    Lap::new(12, 120.522),
                ],
            )],
        ));
        let mut karting_time = KartingTime::default();
        let file_name = "./file_io_test_files/acc_file_1.json";

        // When
        let task_1 = karting_time.update(Message::ImportAccLaptimesFileRequested);
        let task_2 = karting_time.update(Message::ImportAccLaptimesFileCompleted(Some(
            file_name.to_string(),
        )));

        // Then
        assert!(std::path::Path::new(file_name).is_file());
        pretty_assertions::assert_eq!(expected, karting_time);
        pretty_assertions::assert_eq!(1, task_1.units());
        pretty_assertions::assert_eq!(0, task_2.units());
    }

    #[test]
    fn acceptance_test_import_laptime_files() {
        // Given
        let expected = KartingTime::new(DriverProfile::new(
            "Racer",
            vec![RaceResult::new(
                RaceInformation::new("Default", RaceDate::new(1, 1, 2000), Session::new(1, 1)),
                RaceMetadata::new(
                    Default::default(),
                    Default::default(),
                    Default::default(),
                    "",
                    "",
                ),
                vec![
                    Lap::new(1, 120.6),
                    Lap::new(2, 120.7),
                    Lap::new(3, 120.8),
                    Lap::new(4, 120.9),
                ],
            )],
        ));
        let mut karting_time = KartingTime::default();
        let file_name = "file_io_test_files/laptime_file_test_collection_1.json";

        // When
        let task_1 = karting_time.update(Message::ImportLaptimesFileRequested);
        let task_2 = karting_time.update(Message::ImportLaptimesFileCompleted(Some(
            file_name.to_string(),
        )));

        // Then
        assert!(std::path::Path::new(file_name).is_file());
        pretty_assertions::assert_eq!(expected, karting_time);
        pretty_assertions::assert_eq!(1, task_1.units());
        pretty_assertions::assert_eq!(0, task_2.units());
    }

    #[test]
    fn acceptance_test_export_races() {
        // Given
        let driver_profile = DriverProfile::new(
            "Jack Jackson",
            vec![
                RaceResult::new(
                    RaceInformation::new(
                        "Three Sisters",
                        RaceDate::new(12, 12, 2025),
                        Session::new(1, 1),
                    ),
                    RaceMetadata::new(
                        Default::default(),
                        Default::default(),
                        "Kart",
                        "Championship",
                        Default::default(),
                    ),
                    vec![Lap::new(1, 50.4), Lap::new(2, 55.5)],
                ),
                RaceResult::new(
                    RaceInformation::new(
                        "Trafford Park",
                        RaceDate::new(15, 1, 2024),
                        Session::new(2, 3),
                    ),
                    RaceMetadata::new(
                        Default::default(),
                        Default::default(),
                        "Kart",
                        "Championship",
                        Default::default(),
                    ),
                    vec![Lap::new(1, 56.8), Lap::new(2, 58.7)],
                ),
            ],
        );
        let mut karting_time = KartingTime::new(driver_profile);
        let temp_dir = match tempfile::tempdir() {
            Ok(temp_dir) => temp_dir,
            Err(_) => {
                unreachable!();
            }
        };
        let file_location = temp_dir.path();
        let file_path_1 = file_location.join(
            karting_time.driver_profile.races[0]
                .race_information
                .unique_race_identifier
                .clone()
                + ".toml",
        );

        let file_path_2 = file_location.join(
            karting_time.driver_profile.races[1]
                .race_information
                .unique_race_identifier
                .clone()
                + ".toml",
        );

        // When
        let task_1 = karting_time.update(Message::ExportRacesRequested);
        let task_2 = karting_time.update(Message::ExportRacesCompleted(Some(
            file_location.to_string_lossy().into_owned(),
        )));

        // Then
        assert!(std::path::Path::new(&file_path_1).is_file());
        assert!(std::path::Path::new(&file_path_2).is_file());
        pretty_assertions::assert_eq!(1, task_1.units());
        pretty_assertions::assert_eq!(0, task_2.units());

        // Cleanup
        let _guard = TestFileGuard::new(&file_path_1.to_string_lossy());
        let _guard = TestFileGuard::new(&file_path_2.to_string_lossy());
    }

    #[test]
    fn acceptance_test_export_html_races() {
        // Given
        let driver_profile = DriverProfile::new(
            "Jack Jackson",
            vec![
                RaceResult::new(
                    RaceInformation::new(
                        "Seven Sisters",
                        RaceDate::new(12, 12, 2025),
                        Session::new(1, 1),
                    ),
                    RaceMetadata::new(
                        Default::default(),
                        Default::default(),
                        "Kart",
                        "Championship",
                        Default::default(),
                    ),
                    vec![Lap::new(1, 50.4), Lap::new(2, 55.5)],
                ),
                RaceResult::new(
                    RaceInformation::new(
                        "Trafford Lake",
                        RaceDate::new(15, 1, 2024),
                        Session::new(2, 3),
                    ),
                    RaceMetadata::new(
                        Default::default(),
                        Default::default(),
                        "Kart",
                        "Championship",
                        Default::default(),
                    ),
                    vec![Lap::new(1, 56.8), Lap::new(2, 58.7)],
                ),
            ],
        );
        let mut karting_time = KartingTime::new(driver_profile);
        let temp_dir = match tempfile::tempdir() {
            Ok(temp_dir) => temp_dir,
            Err(_) => {
                unreachable!();
            }
        };
        let file_location = temp_dir.path();
        let file_name = file_location.join("Jack Jackson.html");

        // When
        let task_1 = karting_time.update(Message::ExportHtmlRacesRequested);
        let task_2 = karting_time.update(Message::ExportHtmlRacesCompleted(Some(
            file_location.to_string_lossy().into_owned(),
        )));

        // Then
        assert!(std::path::Path::new(&file_name).is_file());
        pretty_assertions::assert_eq!(1, task_1.units());
        pretty_assertions::assert_eq!(0, task_2.units());

        // Cleanup
        let _guard = TestFileGuard::new(&file_name.to_string_lossy());
    }

    #[test]
    fn test_toggle_theme() {
        // Given
        let mut karting_time = KartingTime::default();

        // When
        let task = karting_time.update(Message::ViewToggleTheme);

        // Then
        pretty_assertions::assert_eq!(true, karting_time.application_state.is_light_theme);
        pretty_assertions::assert_eq!(0, task.units());
    }

    #[test]
    fn test_toggle_filter() {
        // Given
        let mut karting_time = KartingTime::default();

        // When
        let task = karting_time.update(Message::ViewToggleFilter);

        // Then
        pretty_assertions::assert_eq!(true, karting_time.driver_profile.filter.is_filter_visible);
        pretty_assertions::assert_eq!(0, task.units());
    }

    #[test]
    fn test_pagination_next() {
        // Given
        let race_result = RaceResult::new(
            RaceInformation::new(
                "Silverstone",
                RaceDate::new(21, 12, 2022),
                Session::new(1, 6),
            ),
            RaceMetadata::default(),
            vec![
                Lap::new(1, 23.6),
                Lap::new(2, 24.6),
                Lap::new(3, 25.4),
                Lap::new(4, 26.7),
            ],
        );
        let mut karting_time = KartingTime::new(DriverProfile::new(
            "Derek",
            vec![
                race_result.clone(),
                race_result.clone(),
                race_result.clone(),
                race_result.clone(),
                race_result.clone(),
                race_result.clone(),
                race_result.clone(),
                race_result.clone(),
                race_result.clone(),
                race_result.clone(),
                race_result.clone(),
                race_result.clone(),
                race_result.clone(),
                race_result.clone(),
                race_result.clone(),
                race_result.clone(),
                race_result.clone(),
            ],
        ));

        // When
        let task = karting_time.update(Message::PaginationNext);

        // Then
        pretty_assertions::assert_eq!(
            1,
            karting_time.driver_profile.filter.pagination.current_page
        );
        pretty_assertions::assert_eq!(0, task.units());
    }

    #[test]
    fn test_pagination_previous() {
        // Given
        let race_result = RaceResult::new(
            RaceInformation::new(
                "Silverstone",
                RaceDate::new(21, 12, 2022),
                Session::new(1, 6),
            ),
            RaceMetadata::default(),
            vec![
                Lap::new(1, 23.6),
                Lap::new(2, 24.6),
                Lap::new(3, 25.4),
                Lap::new(4, 26.7),
            ],
        );
        let mut karting_time = KartingTime::new(DriverProfile::new(
            "Derek",
            vec![
                race_result.clone(),
                race_result.clone(),
                race_result.clone(),
                race_result.clone(),
                race_result.clone(),
                race_result.clone(),
                race_result.clone(),
                race_result.clone(),
                race_result.clone(),
                race_result.clone(),
                race_result.clone(),
                race_result.clone(),
                race_result.clone(),
                race_result.clone(),
                race_result.clone(),
                race_result.clone(),
                race_result.clone(),
            ],
        ));

        // When
        let task = karting_time.update(Message::PaginationPrevious);

        // Then
        pretty_assertions::assert_eq!(
            0,
            karting_time.driver_profile.filter.pagination.current_page
        );
        pretty_assertions::assert_eq!(0, task.units());
    }

    #[test]
    fn test_driver_name_changed() {
        // Given
        let mut karting_time = KartingTime::default();

        // When
        let task = karting_time.update(Message::DriverNameChanged("Lewis Hamilton".to_string()));

        // Then
        pretty_assertions::assert_eq!("Lewis Hamilton", karting_time.driver_profile.name);
        pretty_assertions::assert_eq!(0, task.units());
    }

    #[test]
    fn test_track_name_changed() {
        // Given
        let mut karting_time = KartingTime::default();

        // When
        let task = karting_time.update(Message::TrackNameChanged("Silverstone".to_string()));

        // Then
        pretty_assertions::assert_eq!(
            "Silverstone",
            karting_time
                .driver_profile
                .new_race
                .race_information
                .track_name
        );
        pretty_assertions::assert_eq!(0, task.units());
    }

    #[test]
    fn test_day_changed() {
        // Given
        let mut karting_time = KartingTime::default();

        // When
        let task = karting_time.update(Message::DayChanged("12".to_string()));

        // Then
        pretty_assertions::assert_eq!(
            12,
            karting_time
                .driver_profile
                .new_race
                .race_information
                .date
                .day
        );
        pretty_assertions::assert_eq!(0, task.units());
    }

    #[test]
    fn test_month_changed() {
        // Given
        let mut karting_time = KartingTime::default();

        // When
        let task = karting_time.update(Message::MonthChanged("6".to_string()));

        // Then
        pretty_assertions::assert_eq!(
            6,
            karting_time
                .driver_profile
                .new_race
                .race_information
                .date
                .month
        );
        pretty_assertions::assert_eq!(0, task.units());
    }

    #[test]
    fn test_year_changed() {
        // Given
        let mut karting_time = KartingTime::default();

        // When
        let task = karting_time.update(Message::YearChanged("2022".to_string()));

        // Then
        pretty_assertions::assert_eq!(
            2022,
            karting_time
                .driver_profile
                .new_race
                .race_information
                .date
                .year
        );
        pretty_assertions::assert_eq!(0, task.units());
    }

    #[test]
    fn test_session_id_changed() {
        // Given
        let mut karting_time = KartingTime::default();

        // When
        let task = karting_time.update(Message::SessionIdChanged("4".to_string()));

        // Then
        pretty_assertions::assert_eq!(
            4,
            karting_time
                .driver_profile
                .new_race
                .race_information
                .session
                .session_id
        );
        pretty_assertions::assert_eq!(0, task.units());
    }

    #[test]
    fn test_session_type_changed() {
        // Given
        let mut karting_time = KartingTime::default();

        // When
        let task = karting_time.update(Message::SessionTypeChanged("Race".to_string()));

        // Then
        pretty_assertions::assert_eq!(
            "Race",
            karting_time
                .driver_profile
                .new_race
                .race_metadata
                .session_type
        );
        pretty_assertions::assert_eq!(0, task.units());
    }

    #[test]
    fn test_track_conditions_changed() {
        // Given
        let mut karting_time = KartingTime::default();

        // When
        let task = karting_time.update(Message::TrackConditionsChanged("Wet".to_string()));

        // Then
        pretty_assertions::assert_eq!(
            "Wet",
            karting_time
                .driver_profile
                .new_race
                .race_metadata
                .track_conditions
        );
        pretty_assertions::assert_eq!(0, task.units());
    }

    #[test]
    fn test_race_position_changed() {
        // Given
        let mut karting_time = KartingTime::default();

        // When
        let task = karting_time.update(Message::RacePositionChanged("6".to_string()));

        // Then
        pretty_assertions::assert_eq!(
            6,
            karting_time
                .driver_profile
                .new_race
                .race_information
                .session
                .race_position
        );
        pretty_assertions::assert_eq!(0, task.units());
    }

    #[test]
    fn test_car_used_changed() {
        // Given
        let mut karting_time = KartingTime::default();

        // When
        let task = karting_time.update(Message::CarUsedChanged("Ferrari".to_string()));

        // Then
        pretty_assertions::assert_eq!(
            "Ferrari",
            karting_time.driver_profile.new_race.race_metadata.car_used
        );
        pretty_assertions::assert_eq!(0, task.units());
    }

    #[test]
    fn test_championship_changed() {
        // Given
        let mut karting_time = KartingTime::default();

        // When
        let task = karting_time.update(Message::ChampionshipChanged("GT3".to_string()));

        // Then
        pretty_assertions::assert_eq!(
            "GT3",
            karting_time
                .driver_profile
                .new_race
                .race_metadata
                .championship
        );
        pretty_assertions::assert_eq!(0, task.units());
    }

    #[test]
    fn test_notes_changed() {
        // Given
        let mut karting_time = KartingTime::default();

        // When
        let task = karting_time.update(Message::NotesChanged("This is a note".to_string()));

        // Then
        pretty_assertions::assert_eq!(
            "This is a note",
            karting_time.driver_profile.new_race.race_metadata.notes
        );
        pretty_assertions::assert_eq!(0, task.units());
    }

    #[test]
    fn test_laptime_editor() {
        // Given
        let mut karting_time = KartingTime::default();

        // When
        let task = karting_time.update(Message::LaptimeEditor(Action::Edit(Edit::Paste(
            "Hello there".to_string().into(),
        ))));

        // Then
        pretty_assertions::assert_eq!(
            "Hello there",
            karting_time
                .application_state
                .race_editor
                .text_editor
                .text()
        );
        pretty_assertions::assert_eq!(0, task.units());
    }

    #[test]
    fn test_track_filter_changed() {
        // Given
        let mut karting_time = KartingTime::default();

        // When
        let task = karting_time.update(Message::TrackFilterChanged("Silverstone".to_string()));

        // Then
        pretty_assertions::assert_eq!(
            "Silverstone",
            karting_time.driver_profile.filter.track_query
        );
        pretty_assertions::assert_eq!(0, task.units());
    }

    #[test]
    fn test_date_filter_changed() {
        // Given
        let mut karting_time = KartingTime::default();

        // When
        let task = karting_time.update(Message::DateFilterChanged("22-12-2022".to_string()));

        // Then
        pretty_assertions::assert_eq!("22-12-2022", karting_time.driver_profile.filter.date_query);
        pretty_assertions::assert_eq!(0, task.units());
    }

    #[test]
    fn test_car_used_filter_changed() {
        // Given
        let mut karting_time = KartingTime::default();

        // When
        let task = karting_time.update(Message::CarUsedFilterChanged("Ferrari".to_string()));

        // Then
        pretty_assertions::assert_eq!("Ferrari", karting_time.driver_profile.filter.car_used_query);
        pretty_assertions::assert_eq!(0, task.units());
    }

    #[test]
    fn test_championship_filter_changed() {
        // Given
        let mut karting_time = KartingTime::default();

        // When
        let task = karting_time.update(Message::ChampionshipFilterChanged(
            "Ferrari Cup".to_string(),
        ));

        // Then
        pretty_assertions::assert_eq!(
            "Ferrari Cup",
            karting_time.driver_profile.filter.championship_query
        );
        pretty_assertions::assert_eq!(0, task.units());
    }

    #[test]
    fn test_session_type_filter_changed() {
        // Given
        let mut karting_time = KartingTime::default();

        // When
        let task = karting_time.update(Message::SessionTypeFilterChanged("Race".to_string()));

        // Then
        pretty_assertions::assert_eq!(
            "Race",
            karting_time.driver_profile.filter.session_type_query
        );
        pretty_assertions::assert_eq!(0, task.units());
    }

    #[test]
    fn test_update_races_pressed() {
        // Given
        let race_result = RaceResult::new(
            RaceInformation::new(
                "Silverstone",
                RaceDate::new(21, 12, 2022),
                Session::new(1, 6),
            ),
            RaceMetadata::default(),
            vec![
                Lap::new(1, 23.6),
                Lap::new(2, 24.6),
                Lap::new(3, 25.4),
                Lap::new(4, 26.7),
            ],
        );
        let mut karting_time = KartingTime::new(DriverProfile::new(
            "Derek",
            vec![
                race_result.clone(),
                race_result.clone(),
                race_result.clone(),
            ],
        ));
        karting_time.driver_profile.new_race = RaceResult::new(
            RaceInformation::new(
                "Donnington",
                RaceDate::new(22, 12, 2022),
                Session::new(1, 6),
            ),
            RaceMetadata::default(),
            vec![
                Lap::new(1, 23.6),
                Lap::new(2, 24.6),
                Lap::new(3, 25.4),
                Lap::new(4, 26.7),
            ],
        );

        // When
        let task = karting_time.update(Message::UpdateRacesPressed);

        // Then
        pretty_assertions::assert_eq!(4, karting_time.driver_profile.races.len());
        pretty_assertions::assert_eq!(0, task.units());
    }

    #[test]
    fn test_clear_race_editor_pressed() {
        // Given
        let mut karting_time = KartingTime::default();

        // When
        let task_1 = karting_time.update(Message::LaptimeEditor(Action::Edit(Edit::Paste(
            "Hello there".to_string().into(),
        ))));
        let task_2 = karting_time.update(Message::ClearRaceEditorPressed);

        // Then
        pretty_assertions::assert_eq!(
            "",
            karting_time
                .application_state
                .race_editor
                .text_editor
                .text()
        );
        pretty_assertions::assert_eq!(0, task_1.units());
        pretty_assertions::assert_eq!(0, task_2.units());
    }

    #[test]
    fn test_replaced_pressed() {
        // Given
        let race_result = RaceResult::new(
            RaceInformation::new(
                "Silverstone",
                RaceDate::new(21, 12, 2022),
                Session::new(1, 6),
            ),
            RaceMetadata::default(),
            vec![
                Lap::new(1, 23.6),
                Lap::new(2, 24.6),
                Lap::new(3, 25.4),
                Lap::new(4, 26.7),
            ],
        );
        let new_race_result = RaceResult::new(
            RaceInformation::new(
                "Donnington",
                RaceDate::new(22, 12, 2022),
                Session::new(1, 6),
            ),
            RaceMetadata::default(),
            vec![
                Lap::new(1, 23.6),
                Lap::new(2, 24.6),
                Lap::new(3, 25.4),
                Lap::new(4, 26.7),
            ],
        );
        let mut karting_time = KartingTime::new(DriverProfile::new(
            "Derek",
            vec![
                race_result.clone(),
                race_result.clone(),
                race_result.clone(),
                new_race_result.clone(),
            ],
        ));

        // When
        let task = karting_time.update(Message::ReplacePressed(
            new_race_result
                .race_information
                .unique_race_identifier
                .clone(),
        ));

        // Then
        pretty_assertions::assert_eq!(
            new_race_result.clone(),
            karting_time.driver_profile.new_race
        );
        pretty_assertions::assert_eq!(0, task.units());
    }

    #[test]
    fn test_delete_pressed() {
        // Given
        let race_result = RaceResult::new(
            RaceInformation::new(
                "Silverstone",
                RaceDate::new(21, 12, 2022),
                Session::new(1, 6),
            ),
            RaceMetadata::default(),
            vec![
                Lap::new(1, 23.6),
                Lap::new(2, 24.6),
                Lap::new(3, 25.4),
                Lap::new(4, 26.7),
            ],
        );
        let new_race_result = RaceResult::new(
            RaceInformation::new(
                "Donnington",
                RaceDate::new(22, 12, 2022),
                Session::new(1, 6),
            ),
            RaceMetadata::default(),
            vec![
                Lap::new(1, 23.6),
                Lap::new(2, 24.6),
                Lap::new(3, 25.4),
                Lap::new(4, 26.7),
            ],
        );
        let mut karting_time = KartingTime::new(DriverProfile::new(
            "Derek",
            vec![
                race_result.clone(),
                race_result.clone(),
                race_result.clone(),
                new_race_result.clone(),
            ],
        ));

        // When
        let task = karting_time.update(Message::DeletePressed(
            new_race_result
                .race_information
                .unique_race_identifier
                .clone(),
        ));

        // Then
        pretty_assertions::assert_eq!(true, karting_time.driver_profile.races[3].is_deleting);
        pretty_assertions::assert_eq!(0, task.units());
    }

    #[test]
    fn acceptance_test_delete_confirmed_pressed() {
        // Given
        let race_result = RaceResult::new(
            RaceInformation::new(
                "Silverstone",
                RaceDate::new(21, 12, 2022),
                Session::new(1, 6),
            ),
            RaceMetadata::default(),
            vec![
                Lap::new(1, 23.6),
                Lap::new(2, 24.6),
                Lap::new(3, 25.4),
                Lap::new(4, 26.7),
            ],
        );
        let new_race_result = RaceResult::new(
            RaceInformation::new(
                "Donnington",
                RaceDate::new(22, 12, 2022),
                Session::new(1, 6),
            ),
            RaceMetadata::default(),
            vec![
                Lap::new(1, 23.6),
                Lap::new(2, 24.6),
                Lap::new(3, 25.4),
                Lap::new(4, 26.7),
            ],
        );
        let mut karting_time = KartingTime::new(DriverProfile::new(
            "Derek",
            vec![
                race_result.clone(),
                race_result.clone(),
                race_result.clone(),
                new_race_result.clone(),
            ],
        ));

        // When
        let task_1 = karting_time.update(Message::DeletePressed(
            new_race_result
                .race_information
                .unique_race_identifier
                .clone(),
        ));
        let task_2 = karting_time.update(Message::DeleteConfirmedPressed(
            new_race_result
                .race_information
                .unique_race_identifier
                .clone(),
        ));

        // Then
        pretty_assertions::assert_eq!(3, karting_time.driver_profile.races.len());
        pretty_assertions::assert_eq!(0, task_1.units());
        pretty_assertions::assert_eq!(0, task_2.units());
    }

    #[test]
    fn acceptance_test_delete_cancelled_pressed() {
        // Given
        let race_result = RaceResult::new(
            RaceInformation::new(
                "Silverstone",
                RaceDate::new(21, 12, 2022),
                Session::new(1, 6),
            ),
            RaceMetadata::default(),
            vec![
                Lap::new(1, 23.6),
                Lap::new(2, 24.6),
                Lap::new(3, 25.4),
                Lap::new(4, 26.7),
            ],
        );
        let new_race_result = RaceResult::new(
            RaceInformation::new(
                "Donnington",
                RaceDate::new(22, 12, 2022),
                Session::new(1, 6),
            ),
            RaceMetadata::default(),
            vec![
                Lap::new(1, 23.6),
                Lap::new(2, 24.6),
                Lap::new(3, 25.4),
                Lap::new(4, 26.7),
            ],
        );
        let mut karting_time = KartingTime::new(DriverProfile::new(
            "Derek",
            vec![
                race_result.clone(),
                race_result.clone(),
                race_result.clone(),
                new_race_result.clone(),
            ],
        ));

        // When
        let task_1 = karting_time.update(Message::DeletePressed(
            new_race_result
                .race_information
                .unique_race_identifier
                .clone(),
        ));
        let task_2 = karting_time.update(Message::DeleteCancelledPressed(
            new_race_result
                .race_information
                .unique_race_identifier
                .clone(),
        ));

        // Then
        pretty_assertions::assert_eq!(false, karting_time.driver_profile.races[3].is_deleting);
        pretty_assertions::assert_eq!(4, karting_time.driver_profile.races.len());
        pretty_assertions::assert_eq!(0, task_1.units());
        pretty_assertions::assert_eq!(0, task_2.units());
    }
}
