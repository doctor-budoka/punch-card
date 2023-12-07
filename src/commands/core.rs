use std::env::args;
use chrono::prelude::{DateTime, Local};

use crate::utils::file_io::SafeFileEdit;

use crate::units::day::{
    Day,
    create_daily_dir_if_not_exists, 
    get_current_day,
    read_day,
    write_day};

use crate::utils::file_io::{create_base_dir_if_not_exists};
use crate::utils::config::{Config, create_default_config_if_not_exists, get_config, update_config};

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
    println!("Using the default time to do for the day: {}", default_time_to_do);
    let punch_in_task: String; 
    if other_args.len() == 0 {
        punch_in_task = get_default_punch_in_task();
        println!("No start task for the day provided. Using the default value.");
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
        update_time_behind(day)
    }
    else {
        println!("Can't punch out: Already punched out for the day!")
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
    if other_args.len() > 1 {
        println!("'punch resume' should have at most one argument!");
        return
    }
    // TODO: Make it so we can get the task from before the break as a default
    else if other_args.len() == 0 {
        println!("'punch resume' needs a new task name!");
        return
    }
    let new_block_task: String = 
        get_resume_task_from_args(other_args)
        .expect("We've precluded no arguments");
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

fn get_resume_task_from_args(other_args: Vec<String>) -> Option<String> {
    return match other_args.len() {
        0 => None,
        _ => Some(other_args[0].to_owned()) 
    }
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
    let time_left: i64 = day.get_time_left().expect("Day is over so we should be able to calculate time left!");
    let break_time: i64 = day.get_total_break_time().expect("Day is over so we should be able to calculate total break time!");
    config.update_minutes_behind(time_left);

    println!("Time done today: {}", day.get_time_done().unwrap());
    println!("Total time spent on break: {}", break_time);
    println!("Time left today: {}", time_left);
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