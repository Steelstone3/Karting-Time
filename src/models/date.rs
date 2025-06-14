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

impl Default for Date {
    fn default() -> Self {
        Self {
            day: 1,
            month: 1,
            year: 2000,
        }
    }
}

impl Display for Date {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}-{}", self.year, self.month, self.day)
    }
}

impl Ord for Date {
    fn cmp(&self, other: &Self) -> Ordering {
        let year_ordering = self.year.cmp(&other.year);
        if year_ordering != Ordering::Equal {
            return year_ordering;
        }

        let month_ordering = self.month.cmp(&other.month);
        if month_ordering != Ordering::Equal {
            return month_ordering;
        }

        self.day.cmp(&other.day)
    }
}

impl PartialOrd for Date {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Date {
    pub fn set_day(&mut self, day: String) {
        let day = parse_input_u32(day, u32::MIN, u32::MAX);

        self.set_date(day, self.month, self.year);
    }

    pub fn set_month(&mut self, month: String) {
        let month = parse_input_u32(month, u32::MIN, u32::MAX);

        self.set_date(self.day, month, self.year);
    }

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

#[cfg(test)]
mod date_should {
    use super::Date;
    use rstest::rstest;
    use std::cmp::Ordering;

    #[test]
    fn create_date() {
        // Given
        let expected_date = Date {
            day: 1,
            month: 1,
            year: 2000,
        };

        // Then
        assert_eq!(expected_date, Date::default());
    }

    #[rstest]
    #[case(Date{ day: 21, month: 11, year: 2025 }, Ordering::Equal)]
    #[case(Date{ day: 20, month: 11, year: 2025 }, Ordering::Greater)]
    #[case(Date{ day: 21, month: 10, year: 2025 }, Ordering::Greater)]
    #[case(Date{ day: 21, month: 11, year: 2024 }, Ordering::Greater)]
    #[case(Date{ day: 22, month: 11, year: 2025 }, Ordering::Less)]
    #[case(Date{ day: 21, month: 12, year: 2025 }, Ordering::Less)]
    #[case(Date{ day: 21, month: 11, year: 2026 }, Ordering::Less)]
    fn ordering(#[case] comparison_date: Date, #[case] expected_ordering: Ordering) {
        // Given
        let date = Date {
            day: 21,
            month: 11,
            year: 2025,
        };

        // When
        let ordering = date.cmp(&comparison_date);

        // Then
        assert_eq!(expected_ordering, ordering)
    }

    #[rstest]
    #[case(Date{ day: 21, month: 11, year: 2025 }, Ordering::Equal)]
    #[case(Date{ day: 20, month: 11, year: 2025 }, Ordering::Greater)]
    #[case(Date{ day: 21, month: 10, year: 2025 }, Ordering::Greater)]
    #[case(Date{ day: 21, month: 11, year: 2024 }, Ordering::Greater)]
    #[case(Date{ day: 22, month: 11, year: 2025 }, Ordering::Less)]
    #[case(Date{ day: 21, month: 12, year: 2025 }, Ordering::Less)]
    #[case(Date{ day: 21, month: 11, year: 2026 }, Ordering::Less)]
    fn partial_ordering(#[case] comparison_date: Date, #[case] expected_ordering: Ordering) {
        // Given
        let date = Date {
            day: 21,
            month: 11,
            year: 2025,
        };

        // When
        let ordering = date.partial_cmp(&comparison_date);

        // Then
        assert_eq!(expected_ordering, ordering.unwrap())
    }

    #[test]
    fn display() {
        // Given
        let date = Date {
            day: 15,
            month: 12,
            year: 2000,
        };

        // When
        let date_string = date.to_string();

        // Then
        assert_eq!("2000-12-15", date_string)
    }

    #[test]
    fn set_day() {
        // Given
        let mut date = Date {
            day: 1,
            month: 1,
            year: 2000,
        };

        // When
        date.set_day("15".to_string());

        // Then
        assert_eq!(15, date.day)
    }

    #[test]
    fn set_month() {
        // Given
        let mut date = Date {
            day: 1,
            month: 1,
            year: 2000,
        };

        // When
        date.set_month("11".to_string());

        // Then
        assert_eq!(11, date.month)
    }

    #[test]
    fn set_year() {
        // Given
        let mut date = Date {
            day: 1,
            month: 1,
            year: 2000,
        };

        // When
        date.set_year("2020".to_string());

        // Then
        assert_eq!(2020, date.year)
    }
}
