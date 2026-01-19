use crate::models::filters::filter::Filter;

impl Filter {
    pub fn next_page(&mut self) {
        let total_pages = self
            .filtered_races
            .len()
            .div_ceil(self.pagination.page_size);

        if self.pagination.current_page < total_pages {
            self.pagination.current_page += 1;
        }

        self.update_page();
    }

    pub fn previous_page(&mut self) {
        if self.pagination.current_page > 0 {
            self.pagination.current_page -= 1;
        }

        self.update_page();
    }

    fn update_page(&mut self) {
        let race_count_index = self.filtered_races.len() - 1;
        let current_page_index = self.pagination.current_page - 1;

        self.pagination.start_index = current_page_index * self.pagination.page_size;

        if self.pagination.end_index + self.pagination.page_size > race_count_index {
            self.pagination.end_index = race_count_index;
        } else {
            self.pagination.end_index = self.pagination.start_index + self.pagination.page_size;
        }

        self.pagination.paginated_races =
            self.filtered_races[self.pagination.start_index..self.pagination.end_index].to_vec();
    }
}

#[cfg(test)]
mod paginate_race_results_should {
    #[test]
    #[ignore]
    fn next_page() {}

    #[test]
    #[ignore]
    fn next_page_bound() {}

    #[test]
    #[ignore]
    fn next_page_exceeds_bound() {}

    #[test]
    #[ignore]
    fn previous_page() {}

    #[test]
    #[ignore]
    fn previous_page_bound() {}

    #[test]
    #[ignore]
    fn previous_page_exceeds_bound() {}
}
