use std::collections::HashMap;
use chrono::prelude::{DateTime, Local};
use crate::utils::file_io::SafeFileEdit;

use crate::units::day::{
    Day,
    read_day,
    write_day};

use crate::utils::config::{Config, get_config, update_config};

pub fn punch_in(now: &DateTime<Local>, other_args: Vec<String>) {
    if let Ok(_) = read_day(now) {
        println!("You've already clocked in for the day!");
    }
    else{
        let parsed_args: (String, u64) = get_other_args_for_punch_in(other_args);
        let new_day: Day = Day::new(&now, parsed_args.0, parsed_args.1);
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
        write_day(&day);
        update_time_behind(day);
    }
    else {
        println!("Can't punch out: Already punched out for the day!");
    }
}

pub fn take_break(now: &DateTime<Local>, other_args: Vec<String>, mut day: Day) {
    let resolved_break_name: Result<String, &str> = get_name_for_break(other_args);
    if let Err(msg) = resolved_break_name {
        println!("{}", msg);
        return
    }
    let break_result: Result<(), &str> = day.start_break_at(
        resolved_break_name.expect("break_name error should already have been handled"), &now
    );
    if let Ok(_) = break_result {
        println!("Taking a break at '{}'", &now);
        write_day(&day);

        if !day.has_ended() {day.end_day_at(&now).expect("We should be able to end the day");}
        let mut config: Config = get_config();
        summarise_time(&day, &mut config);
    }
    else {
        let msg = break_result.unwrap_err();
        println!("{}", msg);
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
        println!("{}", msg);
        return
    }

    let new_block_task: String = new_block_task_result.expect("We've precluded no arguments");
    let resume_result: Result<(), &str> = day.start_new_block(new_block_task, &now);
    if let Ok(_) = resume_result {
        println!("Back to work at '{}'", &now);
        write_day(&day);
        if !day.has_ended() {day.end_day_at(&now).expect("We should be able to end the day");}
        let mut config: Config = get_config();
        summarise_time(&day, &mut config);
    }
    else {
        let msg = resume_result.unwrap_err();
        println!("{}", msg);
    }
}

fn get_resume_task_from_args(other_args: Vec<String>, day: Day) -> Result<String, String> {
    return match other_args.len() {
        0 => Ok(day.get_task_name(-2)),
        1 => Ok(other_args[0].to_owned()),
        _ => Err("'punch resume' should have at most one argument!".to_string()),
    }
}

pub fn switch_to_new_task(now: &DateTime<Local>, mut day: Day, other_args: Vec<String>) {
    let new_block_task_result: Result<String, String> = get_new_task_block_from_args(other_args);
    if let Err(msg) = new_block_task_result {
        println!("{}", msg);
        return
    } 

    let new_block_task: String = new_block_task_result.expect("We've handled errors");
    let result: Result<(), &str> = day.start_new_block(new_block_task.to_owned(), &now);
    if let Ok(_) = result {
        println!("Now working on '{}' from '{}'", &new_block_task, &now);
        write_day(&day);
        if !day.has_ended() {day.end_day_at(&now).expect("We should be able to end the day");}
        let mut config: Config = get_config();
        summarise_time(&day, &mut config);
    }
    else {
        let msg = result.unwrap_err();
        println!("{}", msg);
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

pub fn edit_day(day: Day) {
    day.safe_edit_from_file();
}

pub fn summary(now: &DateTime<Local>, mut day: Day) {
    let end_result: Result<(), &str> = day.end_day_at(&now);
    match end_result {
        Ok(_) => (),
        _ => (),
    }
    let mut config: Config = get_config();
    summarise_time(&day, &mut config);
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


fn summarise_time(day: &Day, config: &mut Config) {
    let time_left: i64 = day.get_time_left_secs().expect("Day is over so we should be able to calculate time left!");
    let break_time: i64 = day.get_total_break_time_secs().expect("Day is over so we should be able to calculate total break time!");
    let task_times: HashMap<String, i64> = day.get_task_times_secs();
    config.update_minutes_behind(time_left / 60);

    let time_done_secs = day.get_time_done_secs().unwrap();
    println!("Time done today: {} m {} s", time_done_secs / 60, time_done_secs % 60);
    println!("Total time spent on break: {} m {} s", break_time / 60, break_time % 60);
    println!("Time left today: {} m {} s", time_left / 60, time_left % 60);
    println!("Task times:");
    for (task_name, time) in task_times.into_iter() {
        println!("\t{}: {} m {} s", task_name, time / 60, time % 60);
    }
    println!("Minutes behind overall: {}", config.minutes_behind());
    println!("Minutes behind since last fall behind: {}", config.minutes_behind_non_neg());
}


fn update_time_behind(day: Day) {
    if day.has_ended() {
        let mut config: Config = get_config();
        summarise_time(&day, &mut config);
        update_config(config);
    }
    else {
        panic!("Can't update time behind: The day isn't over yet")
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
        println!("'punch note' requires a msg argument!")
    }
    else if other_args.len() > 1 {
        println!("'punch note' takes a single argument. Consider wrapping your message in quotes.")
    }
    else {
        let msg: String = (&other_args[0]).to_string();
        day.add_note(now, &msg);
        write_day(&day);
        println!("New note '{}' added to today at '{}'.", msg, now);
    }
}
