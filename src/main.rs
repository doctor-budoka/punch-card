use std::env::args;
use chrono::prelude::{DateTime, Local};

mod utils;
use crate::utils::file_io::{create_base_dir_if_not_exists, read_file};
use crate::utils::config::{Config, create_default_config_if_not_exists, get_config_path, get_config, update_config, write_config};
use crate::utils::day::{Day, create_daily_dir_if_not_exists, write_day, read_day, get_current_day};

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
            SubCommand::Pause(_) => take_break(&now, day),
            SubCommand::Resume(_) => resume(&now, day),
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

fn punch_in(now: &DateTime<Local>, other_args: Vec<String>) {
    if let Ok(_) = read_day(now) {
        println!("You've already clocked in for the day!");
    }
    else{
        let time_to_do: u64 = get_time_to_do_for_day(other_args);
        let new_day: Day = Day::new(&now, time_to_do);
        println!("Clocking in for the day at '{}'", &new_day.get_day_start_as_str());
        write_day(&new_day);
    }
}

fn get_time_to_do_for_day(other_args: Vec<String>) -> u64 {
    if other_args.len() == 0 {
        let default_ttd: u64 = get_default_day_in_minutes();
        println!("No time to do for the day provided. Using the default value ({}).", default_ttd);
        println!("You can use `punch edit` to edit this value if this doesn't suit.");
        return default_ttd;
    }
    let first_arg: Result<u64, std::num::ParseIntError> = other_args[0].parse::<u64>();
    return match first_arg {
        Ok(ttd) => {
            println!("Time to do for today: {}", ttd);
            ttd
        },
        Err(_) => {
            let ttd: u64 = get_default_day_in_minutes();
            println!("'{}' is not a valid value for time to do today. Using default instead ({}).", other_args[0], ttd);
            println!("You can use `punch edit` to edit this value if this doesn't suit.");
            ttd
        },
    };
}

fn get_default_day_in_minutes() -> u64 {
    return get_config().day_in_minutes() as u64;
}

fn handle_invalid_cmd(command: &String) {
    println!("'{}' is not a valid subcommand for punch. Try one of the following:", command);
    for str_subcommand in SubCommand::get_allowed_strings() {
        println!("\t{}", str_subcommand);
    }
}

fn punch_out(now: &DateTime<Local>, mut day: Day) {
    if let Ok(_) = day.end_day_at(&now) {
        println!("Punching out for the day at '{}'", &day.get_day_end_as_str().unwrap().trim());
        write_day(&day);
        update_time_behind(day)
    }
    else {
        println!("Can't punch out: Already punched out for the day!")
    }
}

fn take_break(now: &DateTime<Local>, mut day: Day) {
    let break_result: Result<(), &str> = day.start_break(&now);
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

fn resume(now: &DateTime<Local>, mut day: Day) {
    let resume_result: Result<(), &str> = day.end_current_break_at(&now);
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

fn view_day(day: Day) {
    println!("Here's the day so far: \n");
    println!("{}", day.as_string());
}

fn view_config() {
    println!("Here's the current config: \n");
    let config: Config = get_config();
    println!("{}", config.as_string());
}

fn edit_day(day: Day) {
    let path: String = day.get_day_path();
    let temp_path: String = (&path).to_string() + "-temp";
    std::process::Command::new("cp").args([&path, &temp_path]).output().expect("Failed to create temporary data!");

    println!("Opening day in vim...");
    edit_file_in_vim(&temp_path);
    println!("Vim closed.");
    let yaml_str: String = read_file(&temp_path).unwrap();
    let new_day_result: Result<Day, serde_yaml::Error> = Day::try_from_string(&yaml_str);
    match new_day_result {
        Ok(new_day) => write_day(&new_day),
        Err(_) => println!("Invalid Day created. Please try again"),
    };
    std::process::Command::new("rm").arg(&temp_path).output().expect("Failed to clean up the temporary data!");
}

fn edit_config() {
    let config_path: String = get_config_path();
    let temp_path: String = (&config_path).to_string() + "-temp";
    std::process::Command::new("cp").args([&config_path, &temp_path]).output().expect("Failed to create temporary data!");

    println!("Opening config in vim...");
    edit_file_in_vim(&temp_path);
    println!("Vim closed.");
    let yaml_str: String = read_file(&temp_path).unwrap();
    let new_config_result: Result<Config, serde_yaml::Error> = Config::try_from_string(&yaml_str);
    match new_config_result {
        Ok(new_config_result) => write_config(&config_path, &new_config_result),
        Err(_) => println!("Invalid Config created. Please try again"),
    };
    std::process::Command::new("rm").arg(&temp_path).output().expect("Failed to clean up the temporary data!");
}


fn edit_file_in_vim(path: &String) {
    std::process::Command::new("vim")
    .arg(path)
    .spawn()
    .expect("Error: Failed to run editor")
    .wait()
    .expect("Error: Editor returned a non-zero status");
}


fn summary(now: &DateTime<Local>, mut day: Day) {
    let end_result: Result<(), &str> = day.end_day_at(&now);
    match end_result {
        Ok(_) => (),
        _ => (),
    }
    let mut config: Config = get_config();
    summarise_time(&day, &mut config);
}


fn add_note_to_today(now: &DateTime<Local>, mut day: Day, other_args: Vec<String>) {
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

fn add_summary_to_today(mut day: Day, other_args: Vec<String>) {
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
