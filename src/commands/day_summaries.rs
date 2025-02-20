use chrono::{NaiveDate, Duration};
use chrono::prelude::{DateTime, Local};
use std::process::exit;
use std::mem;


use crate::units::day::{Day,read_day_from_date_str};
use crate::units::aggregate_day::AggregateDay
use crate::utils::config::{Config, get_config};

struct DateRange(NaiveDate, NaiveDate);

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

pub fn summarise_week(args: Vec<String>) {}


pub fn summarise_days(args: Vec<String>) {}

pub fn summarise_date_range(start_date_str: String, end_date_str: String, initial_time_behind_opt: Option<i64>) {
    let start_date = NaiveDate::parse_from_str(&start_date_str, "%Y-%m-%d").unwrap();
    let end_date = NaiveDate::parse_from_str(&end_date_str, "%Y-%m-%d").unwrap();

    let seed_time: i64 = initial_time_behind_opt.unwrap_or(0);
    let mut aggregated: AggregateDay = AggregateDay::new(seed_time);
    for local_date in DateRange(start_date, end_date) {
        let this_date_str: String = local_date.format("%Y-%m-%d");
        let this_day: Day = read_day_from_date_str(date_str);
        aggregated.add_day(this_day);
    }

    let print_result: Result<(), String> = print_aggregated_day_summary(&aggregated, initial_time_behind_opt.is_some());
    if let Err(err_msg) = print_result {
        eprintln!("{}", err_msg);
        exit(1);
    }
}

pub fn print_aggregated_day_summary(aggregate_day: &AggregateDay, include_overall_time_behind: bool) -> Result<(), String> {
    let summary_result: Result<String, String> = aggregate_day.render_human_readable_summary(include_overall_time_behind);    
    return match summary_result {
        Ok(summary_str) => {
            println!("{}", summary_str);
            Ok(())
        },
        Err(err_msg) => Err(err_msg),
    }
}


pub fn summary_past(date_str: String) {
    let day: Day = read_day_from_date_str(&date_str);
    if let Err(err_msg) = print_day_summary(day, false) {
        eprintln!("{}", err_msg);
        exit(1);
    }
}

pub fn summary(now: &DateTime<Local>, mut day: Day) {
    let end_result: Result<(), &str> = day.end_day_at(&now);
    match end_result {
        Ok(_) => (),
        _ => (),
    }
    if let Err(err_msg) = print_day_summary(day, true) {
        eprintln!("{}", err_msg);
        exit(1);
    }
}


pub fn print_day_summary(day: Day, use_config_for_time_behind: bool) -> Result<(), String> {
    let time_behind_opt: Option<i64> = match use_config_for_time_behind {
        true => {
            let config: Config = get_config();
            config.minutes_behind() * 60;
        },
        false => None,
    };
    let summary_result: Result<String, String> = day.render_human_readable_summary(time_behind_opt);
    
    return match summary_result {
        Ok(summary_str) => {
            println!("{}", summary_str);
            Ok(())
        },
        Err(err_msg) => Err(err_msg),
    }
}
