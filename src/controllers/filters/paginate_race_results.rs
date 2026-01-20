use crate::models::filters::filter::Filter;

impl Filter {
    pub fn page_of_total_pages(&self) -> String {
        format!(
            "Page {} of {}",
            self.pagination.current_page + 1,
            self.pagination.total_pages + 1
        )
    }

    pub fn next_page(&mut self) {
        self.update_total_page_count();

        if self.pagination.current_page < self.pagination.total_pages {
            self.pagination.current_page += 1;
        }

        self.update_page();
    }

    pub fn previous_page(&mut self) {
        self.update_total_page_count();

        if self.pagination.current_page > 0 {
            self.pagination.current_page -= 1;
        }

        self.update_page();
    }

    fn update_total_page_count(&mut self) {
        self.pagination.total_pages = self.filtered_races.len() / self.pagination.page_size;
    }

    fn update_page(&mut self) {
        let race_count = self.filtered_races.len();

        self.pagination.start_index = self.pagination.current_page * self.pagination.page_size;

        if self.pagination.start_index + self.pagination.page_size > race_count {
            self.pagination.end_index = race_count;
        } else {
            self.pagination.end_index = self.pagination.start_index + self.pagination.page_size;
        }

        self.pagination.paginated_races =
            self.filtered_races[self.pagination.start_index..self.pagination.end_index].to_vec();
    }
}

#[cfg(test)]
mod paginate_race_results_should {
    use super::*;
    use crate::models::{
        date::RaceDate,
        driver::session_information::{
            lap::Lap, race_information::RaceInformation, race_metadata::RaceMetadata,
            race_result::RaceResult, race_statistics::RaceStatistics, session::Session,
        },
        filters::pagination::Pagination,
    };

    #[test]
    fn display_page_message_on_previous_page() {
        // Given
        let mut filter = Filter {
            pagination: Pagination::default(),
            filtered_races: vec![
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
            ],
            ..Default::default()
        };

        // When
        filter.previous_page();
        let current_page = filter.page_of_total_pages();

        // Then
        pretty_assertions::assert_eq!("Page 1 of 5".to_string(), current_page)
    }

    #[test]
    fn next_page() {
        // Given
        let mut race_result = RaceResult {
            race_information: RaceInformation::new(
                "Bob",
                RaceDate::new(24, 12, 2026),
                Session::new(1, 1),
            ),
            race_metadata: RaceMetadata::new(
                "Race",
                "Wet",
                "A Fast One",
                "One With Really Good Drivers In It",
                "No Notes",
            ),
            race_statistics: Default::default(),
            laptimes: vec![Lap::new(1, 56.5)],
            is_deleting: false,
        };
        race_result.race_statistics = RaceStatistics::new(&race_result);

        let mut filter = Filter {
            pagination: Pagination::default(),
            filtered_races: vec![
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
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
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
            ],
            ..Default::default()
        };

        // When
        filter.next_page();
        let current_page = filter.page_of_total_pages();

        // Then
        pretty_assertions::assert_eq!("Page 2 of 5".to_string(), current_page);
        pretty_assertions::assert_eq!(10, filter.pagination.paginated_races.len());
        pretty_assertions::assert_eq!(
            filter.filtered_races[10],
            filter.pagination.paginated_races[0]
        );
        pretty_assertions::assert_eq!(
            filter.filtered_races[11],
            filter.pagination.paginated_races[1]
        );
        pretty_assertions::assert_eq!(
            filter.filtered_races[12],
            filter.pagination.paginated_races[2]
        );
        pretty_assertions::assert_eq!(
            filter.filtered_races[13],
            filter.pagination.paginated_races[3]
        );
        pretty_assertions::assert_eq!(
            filter.filtered_races[14],
            filter.pagination.paginated_races[4]
        );
        pretty_assertions::assert_eq!(
            filter.filtered_races[15],
            filter.pagination.paginated_races[5]
        );
        pretty_assertions::assert_eq!(
            filter.filtered_races[16],
            filter.pagination.paginated_races[6]
        );
        pretty_assertions::assert_eq!(
            filter.filtered_races[17],
            filter.pagination.paginated_races[7]
        );
        pretty_assertions::assert_eq!(
            filter.filtered_races[18],
            filter.pagination.paginated_races[8]
        );
        pretty_assertions::assert_eq!(
            filter.filtered_races[19],
            filter.pagination.paginated_races[9]
        );
    }

    #[test]
    fn next_page_bound() {
        // Given
        let mut race_result = RaceResult {
            race_information: RaceInformation::new(
                "Bob",
                RaceDate::new(24, 12, 2026),
                Session::new(1, 1),
            ),
            race_metadata: RaceMetadata::new(
                "Race",
                "Wet",
                "A Fast One",
                "One With Really Good Drivers In It",
                "No Notes",
            ),
            race_statistics: Default::default(),
            laptimes: vec![Lap::new(1, 56.5)],
            is_deleting: false,
        };
        race_result.race_statistics = RaceStatistics::new(&race_result);

        let mut filter = Filter {
            pagination: Pagination::default(),
            filtered_races: vec![
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                race_result.clone(),
                race_result.clone(),
                race_result.clone(),
                race_result.clone(),
                race_result.clone(),
                race_result.clone(),
                race_result.clone(),
            ],
            ..Default::default()
        };

        // When
        filter.next_page();
        filter.next_page();
        filter.next_page();
        filter.next_page();
        let current_page = filter.page_of_total_pages();

        // Then
        pretty_assertions::assert_eq!("Page 5 of 5".to_string(), current_page);
        pretty_assertions::assert_eq!(8, filter.pagination.paginated_races.len());
        pretty_assertions::assert_eq!(
            filter.filtered_races[40],
            filter.pagination.paginated_races[0]
        );
        pretty_assertions::assert_eq!(
            filter.filtered_races[41],
            filter.pagination.paginated_races[1]
        );
        pretty_assertions::assert_eq!(
            filter.filtered_races[42],
            filter.pagination.paginated_races[2]
        );
        pretty_assertions::assert_eq!(
            filter.filtered_races[43],
            filter.pagination.paginated_races[3]
        );
        pretty_assertions::assert_eq!(
            filter.filtered_races[44],
            filter.pagination.paginated_races[4]
        );
        pretty_assertions::assert_eq!(
            filter.filtered_races[45],
            filter.pagination.paginated_races[5]
        );
        pretty_assertions::assert_eq!(
            filter.filtered_races[46],
            filter.pagination.paginated_races[6]
        );
        pretty_assertions::assert_eq!(
            filter.filtered_races[47],
            filter.pagination.paginated_races[7]
        );
    }

    #[test]
    fn next_page_exceeds_bound() {
        // Given
        let mut race_result = RaceResult {
            race_information: RaceInformation::new(
                "Bob",
                RaceDate::new(24, 12, 2026),
                Session::new(1, 1),
            ),
            race_metadata: RaceMetadata::new(
                "Race",
                "Wet",
                "A Fast One",
                "One With Really Good Drivers In It",
                "No Notes",
            ),
            race_statistics: Default::default(),
            laptimes: vec![Lap::new(1, 56.5)],
            is_deleting: false,
        };
        race_result.race_statistics = RaceStatistics::new(&race_result);

        let mut filter = Filter {
            pagination: Pagination::default(),
            filtered_races: vec![
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                race_result.clone(),
                race_result.clone(),
                race_result.clone(),
                race_result.clone(),
                race_result.clone(),
                race_result.clone(),
                race_result.clone(),
            ],
            ..Default::default()
        };

        // When
        filter.next_page();
        filter.next_page();
        filter.next_page();
        filter.next_page();
        filter.next_page();
        filter.next_page();
        filter.next_page();
        filter.next_page();
        let current_page = filter.page_of_total_pages();

        // Then
        pretty_assertions::assert_eq!("Page 5 of 5".to_string(), current_page);
        pretty_assertions::assert_eq!(8, filter.pagination.paginated_races.len());
        pretty_assertions::assert_eq!(
            filter.filtered_races[40],
            filter.pagination.paginated_races[0]
        );
        pretty_assertions::assert_eq!(
            filter.filtered_races[41],
            filter.pagination.paginated_races[1]
        );
        pretty_assertions::assert_eq!(
            filter.filtered_races[42],
            filter.pagination.paginated_races[2]
        );
        pretty_assertions::assert_eq!(
            filter.filtered_races[43],
            filter.pagination.paginated_races[3]
        );
        pretty_assertions::assert_eq!(
            filter.filtered_races[44],
            filter.pagination.paginated_races[4]
        );
        pretty_assertions::assert_eq!(
            filter.filtered_races[45],
            filter.pagination.paginated_races[5]
        );
        pretty_assertions::assert_eq!(
            filter.filtered_races[46],
            filter.pagination.paginated_races[6]
        );
        pretty_assertions::assert_eq!(
            filter.filtered_races[47],
            filter.pagination.paginated_races[7]
        );
    }

      #[test]
    fn previous_page() {
        // Given
        let mut race_result = RaceResult {
            race_information: RaceInformation::new(
                "Bob",
                RaceDate::new(24, 12, 2026),
                Session::new(1, 1),
            ),
            race_metadata: RaceMetadata::new(
                "Race",
                "Wet",
                "A Fast One",
                "One With Really Good Drivers In It",
                "No Notes",
            ),
            race_statistics: Default::default(),
            laptimes: vec![Lap::new(1, 56.5)],
            is_deleting: false,
        };
        race_result.race_statistics = RaceStatistics::new(&race_result);

        let mut filter = Filter {
            pagination: Pagination::default(),
            filtered_races: vec![
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
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
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
            ],
            ..Default::default()
        };

        // When
        filter.next_page();
        filter.next_page();
        filter.previous_page();
        let current_page = filter.page_of_total_pages();

        // Then
        pretty_assertions::assert_eq!("Page 2 of 5".to_string(), current_page);
        pretty_assertions::assert_eq!(10, filter.pagination.paginated_races.len());
        pretty_assertions::assert_eq!(
            filter.filtered_races[10],
            filter.pagination.paginated_races[0]
        );
        pretty_assertions::assert_eq!(
            filter.filtered_races[11],
            filter.pagination.paginated_races[1]
        );
        pretty_assertions::assert_eq!(
            filter.filtered_races[12],
            filter.pagination.paginated_races[2]
        );
        pretty_assertions::assert_eq!(
            filter.filtered_races[13],
            filter.pagination.paginated_races[3]
        );
        pretty_assertions::assert_eq!(
            filter.filtered_races[14],
            filter.pagination.paginated_races[4]
        );
        pretty_assertions::assert_eq!(
            filter.filtered_races[15],
            filter.pagination.paginated_races[5]
        );
        pretty_assertions::assert_eq!(
            filter.filtered_races[16],
            filter.pagination.paginated_races[6]
        );
        pretty_assertions::assert_eq!(
            filter.filtered_races[17],
            filter.pagination.paginated_races[7]
        );
        pretty_assertions::assert_eq!(
            filter.filtered_races[18],
            filter.pagination.paginated_races[8]
        );
        pretty_assertions::assert_eq!(
            filter.filtered_races[19],
            filter.pagination.paginated_races[9]
        );
    }

    #[test]
    fn previous_page_bound() {
        // Given
        let mut race_result = RaceResult {
            race_information: RaceInformation::new(
                "Bob",
                RaceDate::new(24, 12, 2026),
                Session::new(1, 1),
            ),
            race_metadata: RaceMetadata::new(
                "Race",
                "Wet",
                "A Fast One",
                "One With Really Good Drivers In It",
                "No Notes",
            ),
            race_statistics: Default::default(),
            laptimes: vec![Lap::new(1, 56.5)],
            is_deleting: false,
        };
        race_result.race_statistics = RaceStatistics::new(&race_result);

        let mut filter = Filter {
            pagination: Pagination::default(),
            filtered_races: vec![
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
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
            ],
            ..Default::default()
        };

        // When
        filter.previous_page();
        let current_page = filter.page_of_total_pages();

        // Then
        pretty_assertions::assert_eq!("Page 1 of 5".to_string(), current_page);
        pretty_assertions::assert_eq!(10, filter.pagination.paginated_races.len());
        pretty_assertions::assert_eq!(
            filter.filtered_races[0],
            filter.pagination.paginated_races[0]
        );
        pretty_assertions::assert_eq!(
            filter.filtered_races[1],
            filter.pagination.paginated_races[1]
        );
        pretty_assertions::assert_eq!(
            filter.filtered_races[2],
            filter.pagination.paginated_races[2]
        );
        pretty_assertions::assert_eq!(
            filter.filtered_races[3],
            filter.pagination.paginated_races[3]
        );
        pretty_assertions::assert_eq!(
            filter.filtered_races[4],
            filter.pagination.paginated_races[4]
        );
        pretty_assertions::assert_eq!(
            filter.filtered_races[5],
            filter.pagination.paginated_races[5]
        );
        pretty_assertions::assert_eq!(
            filter.filtered_races[6],
            filter.pagination.paginated_races[6]
        );
        pretty_assertions::assert_eq!(
            filter.filtered_races[7],
            filter.pagination.paginated_races[7]
        );
        pretty_assertions::assert_eq!(
            filter.filtered_races[8],
            filter.pagination.paginated_races[8]
        );
        pretty_assertions::assert_eq!(
            filter.filtered_races[9],
            filter.pagination.paginated_races[9]
        );
    }

 #[test]
    fn previous_page_exceeds_bound() {
        // Given
        let mut race_result = RaceResult {
            race_information: RaceInformation::new(
                "Bob",
                RaceDate::new(24, 12, 2026),
                Session::new(1, 1),
            ),
            race_metadata: RaceMetadata::new(
                "Race",
                "Wet",
                "A Fast One",
                "One With Really Good Drivers In It",
                "No Notes",
            ),
            race_statistics: Default::default(),
            laptimes: vec![Lap::new(1, 56.5)],
            is_deleting: false,
        };
        race_result.race_statistics = RaceStatistics::new(&race_result);

        let mut filter = Filter {
            pagination: Pagination::default(),
            filtered_races: vec![
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
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
                RaceResult::default(),
            ],
            ..Default::default()
        };

        // When
        filter.next_page();
        filter.previous_page();
        filter.previous_page();
        filter.previous_page();
        filter.previous_page();
        let current_page = filter.page_of_total_pages();

        // Then
        pretty_assertions::assert_eq!("Page 1 of 5".to_string(), current_page);
        pretty_assertions::assert_eq!(10, filter.pagination.paginated_races.len());
        pretty_assertions::assert_eq!(
            filter.filtered_races[0],
            filter.pagination.paginated_races[0]
        );
        pretty_assertions::assert_eq!(
            filter.filtered_races[1],
            filter.pagination.paginated_races[1]
        );
        pretty_assertions::assert_eq!(
            filter.filtered_races[2],
            filter.pagination.paginated_races[2]
        );
        pretty_assertions::assert_eq!(
            filter.filtered_races[3],
            filter.pagination.paginated_races[3]
        );
        pretty_assertions::assert_eq!(
            filter.filtered_races[4],
            filter.pagination.paginated_races[4]
        );
        pretty_assertions::assert_eq!(
            filter.filtered_races[5],
            filter.pagination.paginated_races[5]
        );
        pretty_assertions::assert_eq!(
            filter.filtered_races[6],
            filter.pagination.paginated_races[6]
        );
        pretty_assertions::assert_eq!(
            filter.filtered_races[7],
            filter.pagination.paginated_races[7]
        );
        pretty_assertions::assert_eq!(
            filter.filtered_races[8],
            filter.pagination.paginated_races[8]
        );
        pretty_assertions::assert_eq!(
            filter.filtered_races[9],
            filter.pagination.paginated_races[9]
        );
    }
}
