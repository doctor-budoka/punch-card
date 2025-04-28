use chrono::{DateTime, Duration, Local, NaiveDate};
use std::process::exit;
use prettytable::{Table, row};

use crate::units::day::{read_day_from_date_str, Day};
use crate::utils::config::{get_config, Config};
use crate::utils::dates_and_times::{get_local_now, DateRange};
use crate::user_interaction::human_readable::render_seconds_human_readable;
use crate::user_interaction::render_list_for_user::render_list_for_user;

pub fn week_in_tasks(args: Vec<String>) {
    let config: Config = get_config();
    let show_times_in_hours: bool = config.show_times_in_hours_or_default();
    match parse_args_for_week_in_tasks(args) {
        Ok((start_date, end_date)) => {
            print_daily_task_summary_for_date_range(
                start_date,
                end_date,
                show_times_in_hours,
            );
        }
        Err(msg) => {
            eprintln!("{}", msg);
            exit(1);
        }
    }
}

fn parse_args_for_week_in_tasks(
    args: Vec<String>,
) -> Result<(NaiveDate, NaiveDate), String> {
    if args.len() > 1 {
        return Err("Too many args found for week-in-tasks".to_owned());
    }

    let current_date = if args.len() == 0 {
        get_local_now().date_naive()
    } else {
        let parse_date_result = NaiveDate::parse_from_str(&args[0], "%Y-%m-%d");
        if let Err(_) = parse_date_result {
            return Err(format!("First argument for week-in-tasks must be a date of the form 'YYYY-mm-dd'. Got: '{}'", args[0]));
        }
        parse_date_result.expect("Error already handled")
    };

    let week_before: NaiveDate = current_date - Duration::days(6);
    return Ok((week_before, current_date));
}

pub fn daily_tasks(args: Vec<String>) {
    let config: Config = get_config();
    let show_times_in_hours: bool = config.show_times_in_hours_or_default();
    match parse_args_for_daily_tasks(args) {
        Ok((start_date, end_date)) => print_daily_task_summary_for_date_range(
            start_date,
            end_date,
            show_times_in_hours,
        ),
        Err(msg) => {
            eprintln!("{}", msg);
            exit(1);
        }
    }
}

fn parse_args_for_daily_tasks(
    args: Vec<String>,
) -> Result<(NaiveDate, NaiveDate), String> {
    if args.len() == 0 {
        return Err("daily-tasks must have at least one argument.".to_string());
    }
    if args.len() > 2 {
        return Err("daily-tasks must have at most two arguments.".to_string());
    }
    let naive_date_result: Result<NaiveDate, chrono::ParseError> =
        NaiveDate::parse_from_str(&args[0], "%Y-%m-%d");
    if let Err(_) = naive_date_result {
        return Err(format!(
            "First argument for daily-tasks must be a date of the form 'YYYY-mm-dd'. Got: '{}'",
            args[0]
        ));
    }
    let naive_start_date = naive_date_result.expect("Error for this has already been handled!");
    let naive_end_date: NaiveDate = if args.len() >= 2 {
        let naive_end_date_result: Result<NaiveDate, chrono::ParseError> =
            NaiveDate::parse_from_str(&args[1], "%Y-%m-%d");
        if let Err(_) = naive_end_date_result {
            return Err(format!("Second argument for daily-tasks must be a date of the form 'YYYY-mm-dd'. Got: '{}'", args[1]));
        }
        naive_end_date_result.expect("Error for this has already been handled!")
    } else {
        naive_start_date
    };
    return Ok((naive_start_date, naive_end_date));
}

pub fn print_daily_task_summary_for_date_range(
    start_date: NaiveDate,
    end_date: NaiveDate,
    show_times_in_hours: bool,
) {
    let local_now: DateTime<Local> = get_local_now();
    let todays_date: NaiveDate = local_now.date_naive();
    let mut days_done: Vec<String> = Vec::new();
    let mut days_not_there: Vec<String> = Vec::new();
    let mut days_not_ended: Vec<String> = Vec::new();

    let mut table = Table::new();
    table.set_titles(row!["Date", "Task", "Time", "Blocks"]);
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
        let mut first_for_date: bool = true;
        let task_summaries = this_day.get_task_times_secs_and_num_blocks();
        for task_name in this_day.get_tasks_in_chronological_order() {
            let (time, blocks) = task_summaries.get(&task_name).unwrap();
            let date_col = if first_for_date {this_date_str.clone()} else {"".to_owned()};
            table.add_row(row![&date_col, &task_name, render_seconds_human_readable(*time, show_times_in_hours), &blocks]);
            first_for_date = false;
        }

        days_done.push(this_date_str.clone());
    }
    println!(
        "Days included: {}",
        render_list_for_user(&days_done, None)
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
    table.printstd();
}
