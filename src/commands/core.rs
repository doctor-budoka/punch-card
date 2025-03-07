use std::process::exit;
use chrono::prelude::{DateTime, Local};
use crate::commands::day_summaries::print_day_summary;
use crate::utils::file_io::SafeFileEdit;

use crate::units::day::{
    Day,
    read_day,
    read_day_from_date_str,
    write_day};

use crate::utils::config::{Config, get_config, update_config};

pub fn punch_in(now: &DateTime<Local>, other_args: Vec<String>) {
    if let Ok(_) = read_day(now) {
        println!("You've already clocked in for the day!");
    }
    else{
        let parsed_args: (String, u64) = get_other_args_for_punch_in(other_args);
        let new_day: Day = Day::new(&now, parsed_args.0, parsed_args.1, None);
        println!("Clocking in for the day at '{}'", &new_day.get_day_start_as_str());
        write_day(&new_day);
    }
}

fn get_other_args_for_punch_in(other_args: Vec<String>) -> (String, u64) {
    let default_time_to_do: u64 = get_default_day_in_minutes();
    println!("Using the default time to do for the day: {} minutes", default_time_to_do);
    let punch_in_task: String; 
    if other_args.len() == 0 {
        punch_in_task = get_default_punch_in_task();
        println!(
            "No start task for the day provided. Using the default value: '{}'", 
            punch_in_task);
    }
    else {
        punch_in_task = other_args[0].to_owned();
    }
    println!("Remember: You can use `punch edit` to change anything about the day.");
    return (punch_in_task, default_time_to_do)

}

fn get_default_day_in_minutes() -> u64 {
    return get_config().day_in_minutes() as u64;
}

fn get_default_punch_in_task() -> String {
    return get_config().get_default_punch_in_task().to_owned();
}

pub fn punch_out(now: &DateTime<Local>, mut day: Day) {
    if let Ok(_) = day.end_day_at(&now) {
        println!("Punching out for the day at '{}'", &day.get_day_end_as_str().unwrap().trim());        
        let summary_result = print_day_summary(&day, true);
        if let Err(err_msg) = summary_result {
            eprintln!("{}", err_msg);
            exit(1);
        }
        write_day(&day);
        match update_time_behind(day) {
            Ok(()) => (),
            Err(err_msg) => {
                eprintln!("{}", err_msg);
                exit(1);
            }
        }
    }
    else {
        println!("Can't punch out: Already punched out for the day!");
    }
}

pub fn take_break(now: &DateTime<Local>, other_args: Vec<String>, mut day: Day) {
    let resolved_break_name: Result<String, &str> = get_name_for_break(other_args);
    if let Err(msg) = resolved_break_name {
        eprintln!("{}", msg);
        exit(1);
    }
    let break_result: Result<(), &str> = day.start_break_at(
        resolved_break_name.expect("break_name error should already have been handled"), &now
    );
    if let Ok(_) = break_result {
        println!("Taking a break at '{}'", &now);
        write_day(&day);

        if !day.has_ended() {day.end_day_at(&now).expect("We should be able to end the day");}

        let summary_result = print_day_summary(&day, true);
        if let Err(err_msg) = summary_result {
            eprintln!("{}", err_msg);
            exit(1);
        }
    }
    else {
        let msg = break_result.unwrap_err();
        eprintln!("{}", msg);
        exit(1);
    }
}

pub fn get_name_for_break(other_args: Vec<String>) -> Result<String, &'static str> {
    let default_break_name = get_config().get_default_break_task().to_owned();
    return match other_args.len() {
        0 => Ok(default_break_name),
        1 => Ok(other_args[0].to_owned()),
        _ => Err("'punch pause' should have at most one argument!"),
    };
}

pub fn resume(now: &DateTime<Local>, other_args: Vec<String>, mut day: Day) {
    let new_block_task_result: Result<String, String> = get_resume_task_from_args(
        other_args, day.clone());
    if let Err(msg) = new_block_task_result {
        eprintln!("{}", msg);
        exit(1);
    }

    let new_block_task: String = new_block_task_result.expect("We've precluded no arguments");
    let resume_result: Result<(), &str> = day.start_new_block(new_block_task, &now);
    if let Ok(_) = resume_result {
        println!("Back to work at '{}'", &now);
        write_day(&day);
        if !day.has_ended() {day.end_day_at(&now).expect("We should be able to end the day");}

        let summary_result = print_day_summary(&day, true);
        if let Err(err_msg) = summary_result {
            eprintln!("{}", err_msg);
            exit(1);
        }
    }
    else {
        let msg = resume_result.unwrap_err();
        eprintln!("{}", msg);
        exit(1);
    }
}

fn get_resume_task_from_args(other_args: Vec<String>, day: Day) -> Result<String, String> {
    return match other_args.len() {
        0 => Ok(day.get_task_name(-2)),
        1 => Ok(other_args[0].to_owned()),
        _ => Err("'punch resume' should have at most one argument!".to_string()),
    }
}

pub fn punch_back_in(now: &DateTime<Local>, other_args: Vec<String>, mut day: Day) {
    let new_block_task_result: Result<String, String> = get_restart_task_from_args(
        other_args, day.clone());
    if let Err(msg) = new_block_task_result {
        eprintln!("{}", msg);
        exit(1);
    }
    let new_block_task: String = new_block_task_result.expect("We've precluded no arguments");

    let default_break_name = get_config().get_default_break_task().to_owned();
    let restart_result: Result<i64, &str> = day.restart_day(default_break_name, new_block_task, &now);
    if let Ok(seconds_left_before) = restart_result {
        println!("Back to work at '{}'", &now);
        write_day(&day);
        if !day.has_ended() {day.end_day_at(&now).expect("We should be able to end the day");}
        let mut config: Config = get_config();
        config.update_minutes_behind(-seconds_left_before / 60);
        update_config(config);

        let summary_result = print_day_summary(&day, true);
        if let Err(err_msg) = summary_result {
            eprintln!("{}", err_msg);
            exit(1);
        }
    }
    else {
        let msg = restart_result.unwrap_err();
        eprintln!("{}", msg);
        exit(1);
    }
}

fn get_restart_task_from_args(other_args: Vec<String>, day: Day) -> Result<String, String> {
    return match other_args.len() {
        0 => Ok(day.get_task_name(-1)),
        1 => Ok(other_args[0].to_owned()),
        _ => Err("'punch back-in' should have at most one argument!".to_string()),
    }
}

pub fn switch_to_new_task(now: &DateTime<Local>, mut day: Day, other_args: Vec<String>) {
    let new_block_task_result: Result<String, String> = get_new_task_block_from_args(other_args);
    if let Err(msg) = new_block_task_result {
        eprintln!("{}", msg);
        exit(1);
    } 

    let new_block_task: String = new_block_task_result.expect("We've handled errors");
    let result: Result<(), &str> = day.start_new_block(new_block_task.to_owned(), &now);
    if let Ok(_) = result {
        println!("Now working on '{}' from '{}'", &new_block_task, &now);
        write_day(&day);
        if !day.has_ended() {day.end_day_at(&now).expect("We should be able to end the day");}

        let summary_result = print_day_summary(&day, true);
        if let Err(err_msg) = summary_result {
            eprintln!("{}", err_msg);
            exit(1);
        }
    }
    else {
        let msg = result.unwrap_err();
        eprintln!("{}", msg);
        exit(1);
    }
}

fn get_new_task_block_from_args(other_args: Vec<String>) -> Result<String, String> {
    return match other_args.len() {
        0 => Err("'punch task' needs a new task name!".to_string()),
        1 => Ok(other_args[0].to_owned()),
        _ => Err("'punch task' should have at most one argument!".to_string()),
    };
}

pub fn view_day(day: Day) {
    println!("Here's the day so far: \n");
    println!("{}", day.as_string());
}

pub fn view_past(other_args: Vec<String>) {
    let arg_result: Result<String, String> = parse_args_for_view_past(other_args);
    
    if let Err(msg) = arg_result {
        eprintln!("{}", msg);
        exit(1);
    }
    else if let Ok(date_str) = arg_result {
        if let Ok(day) = read_day_from_date_str(&date_str) {
            println!("Here is {}:\n", date_str);
            println!("{}", day.as_string());
        }
        else {
            eprintln!("'{}' does not have a day associated with it!", date_str);
        }
    }
    
}

fn parse_args_for_view_past(other_args: Vec<String>) -> Result<String, String> {
    return match other_args.len() {
        1 => Ok(other_args[0].to_owned()),
        _ => Err("'punch view-past' should have exactly one argument!".to_string()),
    };
}

pub fn edit_day(day: Day) {
    day.safe_edit_from_file();
}

pub fn add_summary_to_today(mut day: Day, other_args: Vec<String>) {
    if other_args.len() != 4 {
        println!("'punch add-summary' takes exactly 4 arguments: category, project, task and summary.")
    }
    else {
        let (category, project, task, summary) = (
            other_args[0].to_string(), other_args[1].to_string(), other_args[2].to_string(), other_args[3].to_string()
        );
        day.add_summary(category, project, task, summary);
        write_day(&day);
    }
}

pub fn view_config() {
    println!("Here's the current config: \n");
    let config: Config = get_config();
    println!("{}", config.as_string());
}

pub fn edit_config() {
    let config = get_config();
    config.safe_edit_from_file();
}

pub fn add_note_to_today(now: &DateTime<Local>, mut day: Day, other_args: Vec<String>) {
    if other_args.len() == 0 {
        eprintln!("'punch note' requires a msg argument!");
        exit(1);
    }
    else if other_args.len() > 1 {
        eprintln!("'punch note' takes a single argument. Consider wrapping your message in quotes.");
        exit(1);
    }
    else {
        let msg: String = (&other_args[0]).to_string();
        day.add_note(now, &msg);
        write_day(&day);
        println!("New note '{}' added to today at '{}'.", msg, now);
    }
}

pub fn update_current_task_name(now: &DateTime<Local>, mut day: Day, other_args: Vec<String>) {
    let task_name_result: Result<String, String> = get_new_task_name_from_args(other_args);
    if let Err(msg) = task_name_result {
        eprintln!("{}", msg);
        exit(1);
    }
    let task_name = task_name_result.expect("Error already handled!");
    let change_task_result: Result<(), &str> = day.update_current_task_name(task_name.clone());
    
    if let Ok(_) = change_task_result {
        println!("Updated the current task to '{}'", &task_name);
        write_day(&day);
        if !day.has_ended() {day.end_day_at(&now).expect("We should be able to end the day");}

        let summary_result = print_day_summary(&day, true);
        if let Err(err_msg) = summary_result {
            eprintln!("{}", err_msg);
            exit(1);
        }
    }
    else {
        let msg = change_task_result.unwrap_err();
        eprintln!("{}", msg);
        exit(1);
    }
}

fn get_new_task_name_from_args(other_args: Vec<String>) -> Result<String, String> {
    return match other_args.len() {
        0 => Err("'punch update-task' needs a new task name!".to_string()),
        1 => Ok(other_args[0].to_owned()),
        _ => Err("'punch update-task' should have at most one argument!".to_string()),
    };
}


fn update_time_behind(day: Day) -> Result<(), String> {
    if day.has_ended() {
        let mut config: Config = get_config();
        let time_left: i64 = day.get_time_left_secs().expect("Day is over so we should have a time left!");
        config.update_minutes_behind(time_left / 60);
        update_config(config);
        return Ok(());
    }
    else {
        return Err("Can't update time behind: The day isn't over yet".to_string());
    }
}
