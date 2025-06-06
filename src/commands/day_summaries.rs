use chrono::{DateTime, Duration, Local, NaiveDate};
use std::process::exit;

use crate::units::aggregate_day::AggregateDay;
use crate::units::day::{read_day_from_date_str, Day};
use crate::user_interaction::convert_input::convert_input_to_seconds;
use crate::user_interaction::render_list_for_user::render_list_for_user;
use crate::utils::config::{get_config, Config};
use crate::utils::dates_and_times::{get_local_now, DateRange};

pub fn summarise_week(args: Vec<String>) {
    let config: Config = get_config();
    let show_times_in_hours: bool = config.show_times_in_hours_or_default();
    match parse_args_for_summarise_week(args) {
        Ok((start_date, end_date, initial_time_behind_opt)) => {
            summarise_date_range(
                start_date,
                end_date,
                initial_time_behind_opt,
                show_times_in_hours,
            );
        }
        Err(msg) => {
            eprintln!("{}", msg);
            exit(1);
        }
    }
}

fn parse_args_for_summarise_week(
    args: Vec<String>,
) -> Result<(NaiveDate, NaiveDate, Option<i64>), String> {
    if args.len() > 2 {
        return Err("Too many args found for summarise_week".to_owned());
    }

    let current_date = if args.len() == 0 {
        get_local_now().date_naive()
    } else {
        let parse_date_result = NaiveDate::parse_from_str(&args[0], "%Y-%m-%d");
        if let Err(_) = parse_date_result {
            return Err(format!("First argument for summarise-week must be a date of the form 'YYYY-mm-dd'. Got: '{}'", args[0]));
        }
        parse_date_result.expect("Error already handled")
    };

    let week_before: NaiveDate = current_date - Duration::days(6);

    let intial_time_behind_opt = if args.len() == 2 {
        let parse_result: Result<i64, String> = convert_input_to_seconds(&args[1]);
        if let Err(err_msg) = parse_result {
            return Err(err_msg);
        }
        Some(parse_result.expect("Error from parsing i64 already handled!"))
    } else {
        None
    };

    return Ok((week_before, current_date, intial_time_behind_opt));
}

pub fn summarise_days(args: Vec<String>) {
    let config: Config = get_config();
    let show_times_in_hours: bool = config.show_times_in_hours_or_default();
    match parse_args_for_summarise_days(args) {
        Ok((start_date, end_date, initial_time_behind_opt)) => summarise_date_range(
            start_date,
            end_date,
            initial_time_behind_opt,
            show_times_in_hours,
        ),
        Err(msg) => {
            eprintln!("{}", msg);
            exit(1);
        }
    }
}

fn parse_args_for_summarise_days(
    args: Vec<String>,
) -> Result<(NaiveDate, NaiveDate, Option<i64>), String> {
    if args.len() == 0 {
        return Err("summarise-days must have at least one argument.".to_string());
    }
    if args.len() > 3 {
        return Err("summarise-days must have at most three arguments.".to_string());
    }
    let naive_date_result: Result<NaiveDate, chrono::ParseError> =
        NaiveDate::parse_from_str(&args[0], "%Y-%m-%d");
    if let Err(_) = naive_date_result {
        return Err(format!(
            "First argument for summarise-days must be a date of the form 'YYYY-mm-dd'. Got: '{}'",
            args[0]
        ));
    }
    let naive_start_date = naive_date_result.expect("Error for this has already been handled!");
    let naive_end_date: NaiveDate = if args.len() >= 2 {
        let naive_end_date_result: Result<NaiveDate, chrono::ParseError> =
            NaiveDate::parse_from_str(&args[1], "%Y-%m-%d");
        if let Err(_) = naive_end_date_result {
            return Err(format!("Second argument for summarise-days must be a date of the form 'YYYY-mm-dd'. Got: '{}'", args[1]));
        }
        naive_end_date_result.expect("Error for this has already been handled!")
    } else {
        naive_start_date
    };
    let initial_time_behind_opt = if args.len() == 3 {
        let parse_result: Result<i64, String> = convert_input_to_seconds(&args[2]);
        if let Err(err_msg) = parse_result {
            return Err(err_msg);
        }
        Some(parse_result.expect("Error for this has already been handled!"))
    } else {
        None
    };

    return Ok((naive_start_date, naive_end_date, initial_time_behind_opt));
}

pub fn summarise_date_range(
    start_date: NaiveDate,
    end_date: NaiveDate,
    initial_time_behind_opt: Option<i64>,
    show_times_in_hours: bool,
) {
    let seed_time: i64 = initial_time_behind_opt.unwrap_or(0);
    let mut aggregated: AggregateDay = AggregateDay::new(seed_time);

    let local_now: DateTime<Local> = get_local_now();
    let todays_date: NaiveDate = local_now.date_naive();
    let mut days_aggregated: Vec<String> = Vec::new();
    let mut days_not_there: Vec<String> = Vec::new();
    let mut days_not_ended: Vec<String> = Vec::new();
    for local_date in DateRange(start_date, end_date) {
        let this_date_str: String = local_date.format("%Y-%m-%d").to_string();
        let this_day_result = read_day_from_date_str(&this_date_str);

        if let Err(_) = this_day_result {
            days_not_there.push(this_date_str.clone());
            continue;
        }
        let mut this_day: Day = this_day_result.expect("Already handled error!");
        if !this_day.has_ended() && (local_date == todays_date) {
            match this_day.end_day_at(&local_now, false) {
                Ok(()) => (),
                Err(err_msg) => {
                    eprintln!("Failed to end today: {err_msg}");
                    exit(1);
                }
            }
        } else if !this_day.has_ended() {
            days_not_ended.push(this_date_str.clone());
            continue;
        }
        if let Err(err_msg) = aggregated.add_day(this_day) {
            eprintln!("{}", err_msg);
            exit(1);
        };
        days_aggregated.push(this_date_str.clone());
    }
    println!(
        "Days aggregated: {}",
        render_list_for_user(&days_aggregated, None)
    );
    if days_not_there.len() > 0 {
        println!(
            "Days not there: {}",
            render_list_for_user(&days_not_there, None)
        );
    }
    if days_not_ended.len() > 0 {
        println!(
            "Days not ended: {}",
            render_list_for_user(&days_not_ended, None)
        );
    }
    let print_result: Result<(), String> = print_aggregated_day_summary(
        &aggregated,
        initial_time_behind_opt.is_some(),
        show_times_in_hours,
    );
    if let Err(err_msg) = print_result {
        eprintln!("{}", err_msg);
        exit(1);
    }
}

pub fn print_aggregated_day_summary(
    aggregate_day: &AggregateDay,
    include_overall_time_behind: bool,
    show_times_in_hours: bool,
) -> Result<(), String> {
    let summary_result: Result<String, String> = aggregate_day
        .render_human_readable_summary(include_overall_time_behind, show_times_in_hours);
    return match summary_result {
        Ok(summary_str) => {
            println!("{}", summary_str);
            Ok(())
        }
        Err(err_msg) => Err(err_msg),
    };
}

pub fn summary_past(args: Vec<String>) {
    let parse_result: Result<NaiveDate, String> = parse_args_for_summary_past(args);
    if let Err(err_msg) = parse_result {
        eprintln!("{}", err_msg);
        exit(1);
    }
    let date: NaiveDate = parse_result.expect("Error should have already been handled.");
    let date_str: String = date.format("%Y-%m-%d").to_string();
    let day_result = read_day_from_date_str(&date_str);
    if let Err(_) = day_result {
        eprintln!("'{}' either doesn't exist or is malformed!", date_str);
        exit(1);
    }
    let day: Day = day_result.expect("Already handled the error case!");
    if let Err(err_msg) = print_day_summary(&day, false) {
        eprintln!("{}", err_msg);
        exit(1);
    }
}

fn parse_args_for_summary_past(args: Vec<String>) -> Result<NaiveDate, String> {
    return match args.len() {
        0 => Err("'punch summary-past' takes a single argument. None were given.".to_string()),
        1 => {
            let parse_result: Result<NaiveDate, chrono::ParseError> =
                NaiveDate::parse_from_str(&args[0], "%Y-%m-%d");
            if let Err(err_contents) = parse_result {
                return Err(err_contents.to_string());
            }
            Ok(parse_result.expect("Error for this has already been processed."))
        }
        a => Err(format!(
            "'punch summary-past' takes a single argument. {} were given.",
            a
        )),
    };
}

pub fn summary(now: &DateTime<Local>, mut day: Day) {
    if !day.has_ended() {
        let end_result: Result<(), &str> = day.end_day_at(&now, false);
        match end_result {
            Ok(_) => (),
            Err(err_msg) => {
                eprintln!("Couldn't end day: {}", err_msg);
                exit(1);
            }
        }
    }
    if let Err(err_msg) = print_day_summary(&day, true) {
        eprintln!("{}", err_msg);
        exit(1);
    }
}

pub fn print_day_summary(day: &Day, use_config_for_time_behind: bool) -> Result<(), String> {
    let config: Config = get_config();
    let show_times_in_hours = config.show_times_in_hours_or_default();
    let time_behind_opt: Option<i64> = match use_config_for_time_behind {
        true => Some(config.get_seconds_behind()),
        false => None,
    };
    let summary_result: Result<String, String> =
        day.render_human_readable_summary(time_behind_opt, show_times_in_hours);

    return match summary_result {
        Ok(summary_str) => {
            println!("{}", summary_str);
            Ok(())
        }
        Err(err_msg) => Err(err_msg),
    };
}
