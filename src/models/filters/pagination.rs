use crate::models::driver::session_information::race_result::RaceResult;

#[derive(Clone, PartialEq, Debug)]
pub struct Pagination {
    pub paginated_races: Vec<RaceResult>,
    pub current_page: usize,
    pub page_size: usize,
    pub start_index: usize,
    pub end_index: usize,
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            page_size: 10,
            paginated_races: Default::default(),
            current_page: Default::default(),
            start_index: Default::default(),
            end_index: Default::default(),
        }
    }
}
