use serde_with::{DeserializeFromStr, SerializeDisplay};

use crate::prelude::*;

/// A date relevant for the invoice, e.g. invoice date, due date or a transaction
/// date for an expense.
#[derive(
    Clone,
    Copy,
    Debug,
    Display,
    PartialEq,
    Eq,
    SerializeDisplay,
    DeserializeFromStr,
    TypedBuilder,
    Getters,
)]
#[display("{year:04}-{month:02}-{day:02}")]
pub struct Date {
    /// e.g. 2025
    #[builder(setter(into))]
    #[getset(get = "pub")]
    year: Year,

    /// e.g. 5 for May
    #[builder(setter(into))]
    #[getset(get = "pub")]
    month: Month,

    /// e.g. 31 for the last day of May
    #[builder(setter(into))]
    #[getset(get = "pub")]
    day: Day,
}

/// A date relevant for the invoice, e.g. invoice date, due date or a transaction
/// date for an expense.
#[derive(
    Clone,
    Copy,
    Debug,
    Display,
    PartialEq,
    Eq,
    Hash,
    SerializeDisplay,
    DeserializeFromStr,
    TypedBuilder,
    Getters,
)]
#[display("{year:04}-{month:02}")]
pub struct YearAndMonth {
    /// e.g. 2025
    #[builder(setter(into))]
    #[getset(get = "pub")]
    year: Year,

    /// e.g. 5 for May
    #[builder(setter(into))]
    #[getset(get = "pub")]
    month: Month,
}
impl YearAndMonth {
    pub fn last_day_of_month(&self) -> Day {
        match **self.month() {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => Day::from(31),
            4 | 6 | 9 | 11 => Day::from(30),
            2 => {
                let year = **self.year();
                if (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0) {
                    Day::from(29)
                } else {
                    Day::from(28)
                }
            }
            _ => unreachable!("Invalid month value"),
        }
    }

    pub fn to_date_end_of_month(&self) -> Date {
        Date::builder()
            .year(self.year)
            .month(self.month)
            .day(self.last_day_of_month())
            .build()
    }

    pub fn current() -> Self {
        use chrono::Datelike;
        let today = chrono::Local::now().date_naive();
        Self::builder()
            .year(Year::from(today.year()))
            .month(Month::from(today.month() as i32))
            .build()
    }

    pub fn one_month_earlier(&self) -> Self {
        let mut year = *self.year;
        let mut month = *self.month;

        if month == 1 {
            year -= 1;
            month = 12
        } else {
            month -= 1
        }

        Self::builder()
            .year(Year::from(year))
            .month(Month::from(month))
            .build()
    }

    pub fn last() -> Self {
        Self::current().one_month_earlier()
    }
}

impl PartialOrd for YearAndMonth {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.year > other.year {
            Some(std::cmp::Ordering::Greater)
        } else if self.year < other.year {
            Some(std::cmp::Ordering::Less)
        } else if self.month > other.month {
            Some(std::cmp::Ordering::Greater)
        } else if self.month < other.month {
            Some(std::cmp::Ordering::Less)
        } else {
            Some(std::cmp::Ordering::Equal)
        }
    }
}

impl std::str::FromStr for YearAndMonth {
    type Err = crate::prelude::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('-').collect();
        if parts.len() != 2 {
            return Err(Error::FailedToParseDate {
                underlying: "Invalid Format YearAndMonth".to_owned(),
            });
        }

        let year = Year::from(
            parts[0]
                .parse::<i32>()
                .map_err(|_| Error::FailedToParseDate {
                    underlying: "Invalid year".to_owned(),
                })?,
        );
        let month = Month::from(
            parts[1]
                .parse::<i32>()
                .map_err(|_| Error::FailedToParseDate {
                    underlying: "Invalid month".to_owned(),
                })?,
        );

        Ok(Self::builder().year(year).month(month).build())
    }
}

impl std::str::FromStr for Date {
    type Err = crate::prelude::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('-').collect();
        if parts.len() != 3 {
            return Err(Error::FailedToParseDate {
                underlying: "Invalid Format".to_owned(),
            });
        }

        let year = Year::from(
            parts[0]
                .parse::<i32>()
                .map_err(|_| Error::FailedToParseDate {
                    underlying: "Invalid year".to_owned(),
                })?,
        );
        let month = Month::from(
            parts[1]
                .parse::<i32>()
                .map_err(|_| Error::FailedToParseDate {
                    underlying: "Invalid month".to_owned(),
                })?,
        );
        let day = Day::from(
            parts[2]
                .parse::<i32>()
                .map_err(|_| Error::FailedToParseDate {
                    underlying: "Invalid day".to_owned(),
                })?,
        );

        Ok(Self::builder().year(year).month(month).day(day).build())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compare_year_and_month() {
        let date1 = YearAndMonth::from_str("2025-05").unwrap();
        let date2 = YearAndMonth::from_str("2025-06").unwrap();
        let date3 = YearAndMonth::from_str("2024-12").unwrap();

        assert!(date1 < date2);
        assert!(date2 > date1);
        assert!(date1 > date3);
        assert!(date3 < date1);
    }
}
