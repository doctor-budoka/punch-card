use chrono::{DateTime, Duration, Local, NaiveDate};
use std::mem;

pub struct DateRange(pub NaiveDate, pub NaiveDate);

impl Iterator for DateRange {
    type Item = NaiveDate;
    fn next(&mut self) -> Option<Self::Item> {
        if self.0 <= self.1 {
            let next = self.0 + Duration::days(1);
            Some(mem::replace(&mut self.0, next))
        } else {
            None
        }
    }
}

pub fn get_local_now() -> DateTime<Local> {
    return Local::now();
}

pub fn convert_date_to_date_str(date: NaiveDate) -> String {
    return date.format("%Y-%m-%d").to_string();
}

#[allow(dead_code)]
pub fn get_todays_date_str() -> String {
    return convert_date_to_date_str(get_local_now().date_naive());
}
