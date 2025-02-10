use std::{cmp::Ordering, fmt::Display};

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::views::application::input_parser::parse_input_u32;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Eq)]
pub struct Date {
    pub day: u32,
    pub month: u32,
    pub year: i32,
}

impl Ord for Date {
    fn cmp(&self, other: &Self) -> Ordering {
        // First, compare years
        let year_ordering = self.year.cmp(&other.year);
        if year_ordering != Ordering::Equal {
            return year_ordering;
        }

        // If years are equal, compare months
        let month_ordering = self.month.cmp(&other.month);
        if month_ordering != Ordering::Equal {
            return month_ordering;
        }

        // If years and months are equal, compare days
        self.day.cmp(&other.day)
    }
}

impl PartialOrd for Date {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Date {
    // TODO Test
    pub fn set_day(&mut self, day: String) {
        let day = parse_input_u32(day, u32::MIN, u32::MAX);

        self.set_date(day, self.month, self.year);
    }

    // TODO Test
    pub fn set_month(&mut self, month: String) {
        let month = parse_input_u32(month, u32::MIN, u32::MAX);

        self.set_date(self.day, month, self.year);
    }

    // TODO Test
    pub fn set_year(&mut self, year: String) {
        let year = parse_input_u32(year, u32::MIN, u32::MAX);

        self.set_date(self.day, self.month, year as i32);
    }

    fn set_date(&mut self, day: u32, month: u32, year: i32) {
        if NaiveDate::from_ymd_opt(year, month, day).is_some() {
            self.day = day;
            self.month = month;
            self.year = year;
        }
    }
}

// TODO Test
impl Default for Date {
    fn default() -> Self {
        Self {
            day: 1,
            month: 1,
            year: 2000,
        }
    }
}

// TODO Test
impl Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}-{}", self.day, self.month, self.year)
    }
}
