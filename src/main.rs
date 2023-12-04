use std::env::args;
use chrono::prelude::{DateTime, Local};

mod commands;
mod units;
mod utils;
use crate::units::day::Day;
use crate::commands::core::{
    punch_in, 
    punch_out, 
    take_break, 
    resume, 
    view_day, 
    edit_day,
    add_note_to_today,
    add_summary_to_today,
    view_config,
    edit_config,

};
use crate::utils::file_io::{create_base_dir_if_not_exists};
use crate::utils::config::{Config, create_default_config_if_not_exists, get_config, update_config};



#[derive(PartialEq)]
enum SubCommand {
    In(Vec<String>),
    Out(Vec<String>),
    Pause(Vec<String>),
    Resume(Vec<String>),
    Summary(Vec<String>),
    View(Vec<String>),
    Edit(Vec<String>),
    Note(Vec<String>),
    EditConfig(Vec<String>),
    ViewConfig(Vec<String>),
    AddSummary(Vec<String>),
    Invalid(String),
}

impl SubCommand {
    fn from_string(name: &String, other_args: Vec<String>) -> Self {
        return match name.to_owned().trim() {
            "in" => Self::In(other_args),
            "out" => Self::Out(other_args),
            "pause" => Self::Pause(other_args),
            "resume" => Self::Resume(other_args),
            "summary" => Self::Summary(other_args),
            "view" => Self::View(other_args),
            "edit" => Self::Edit(other_args),
            "note" => Self::Note(other_args),
            "edit-config" => Self::EditConfig(other_args),
            "view-config" => Self::ViewConfig(other_args),
            "add-summary" => Self::AddSummary(other_args),
            other => Self::Invalid(other.to_string()),
        }
    }

    fn get_allowed_strings() -> Vec<String> {
        return Vec::from(
            [
                "in", "out", "pause", "resume", "summary", "view", "edit", "note", "edit-config", "add-summary"
            ].map(|x: &str| x.to_string())
        );
    }
}

fn main() {
    let env_args: Vec<String> = args().collect();
    let command_name: &String = &env_args[1];
    let other_args: Vec<String> = env_args[2..].to_vec();
    let command: SubCommand = SubCommand::from_string(command_name, other_args);

    setup();

    let now: DateTime<Local> = Local::now();
    run_command(command, now);
}

fn setup() {
    create_base_dir_if_not_exists();
    create_daily_dir_if_not_exists();
    create_default_config_if_not_exists();
}

fn run_command(command: SubCommand, now: DateTime<Local>) {
    if let SubCommand::In(other_args) = command {
        punch_in(&now, other_args);
    }
    else if let SubCommand::Invalid(original) = command {
        handle_invalid_cmd(&original);
    }
    else {
        let possible_day: Result<Day, String> = get_current_day(&now);
        if let Err(msg) = possible_day {
            println!("{}", msg);
            return
        }
        let day: Day = possible_day.unwrap();

        match command {
            SubCommand::Out(_) => punch_out(&now, day),
            SubCommand::Pause(other_args) => take_break(&now, other_args, day),
            SubCommand::Resume(other_args) => resume(&now, other_args, day),
            SubCommand::Summary(_) => summary(&now, day),
            SubCommand::View(_) => view_day(day),
            SubCommand::Edit(_) => edit_day(day),
            SubCommand::EditConfig(_) => edit_config(),
            SubCommand::ViewConfig(_) => view_config(),
            SubCommand::Note(other_args) => add_note_to_today(&now, day, other_args),
            SubCommand::AddSummary(other_args) => add_summary_to_today(day, other_args),
            SubCommand::In(_) => unreachable!("'punch in' commands shouldn't be being processed"),
            SubCommand::Invalid(_) => unreachable!("Invalid commands shouldn't be being processed here"),
        }
    }
}

fn handle_invalid_cmd(command: &String) {
    println!("'{}' is not a valid subcommand for punch. Try one of the following:", command);
    for str_subcommand in SubCommand::get_allowed_strings() {
        println!("\t{}", str_subcommand);
    }
}