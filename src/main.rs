use std::env::args;
use std::path::Path;
use chrono::prelude::{DateTime, Utc};

mod utils;
use utils::{Config, write_config, create_dir_if_not_exists, expand_path, Day, write_day, read_day, read_config};

const DEFAULT_TIME_MINS: i64 = 480;
const BASE_DIR: &str = "~/.punch-card/";
const DAILY_DIR: &str = "days/";
const CONFIG_FILE: &str = "punch.cfg";

const DATE_FMT: &str = "%Y-%m-%d";


enum SubCommand {
    In,
    Out,
    Pause,
    Resume,
    Summary,
}

impl SubCommand {
    fn from_string(name: &String) -> Self {
        return match name.to_owned().trim() {
            "in" => Self::In,
            "out" => Self::Out,
            "pause" => Self::Pause,
            "resume" => Self::Resume,
            "summary" => Self::Summary,
            other => panic!("{other} is not a valid subcommand!"),
        }
    }
}

fn main() {
    let env_args: Vec<String> = args().collect();
    let command_name: &String = &env_args[1];
    let command: SubCommand = SubCommand::from_string(command_name);

    setup();

    match command {
        SubCommand::In => punch_in(),
        SubCommand::Out => punch_out(),
        SubCommand::Pause=> take_break(),
        SubCommand::Resume => resume(),
        SubCommand::Summary => summary(),
    }
}

fn setup() {
    create_dir_if_not_exists(BASE_DIR);
    let daily_dir: String = BASE_DIR.to_string() + &(DAILY_DIR.to_string());
    create_dir_if_not_exists(&daily_dir);

    let config_path: String = expand_path(&(BASE_DIR.to_owned() + &(CONFIG_FILE.to_owned())));
    if !Path::new(&config_path).exists() {
        let default_config: Config = Config::new(DEFAULT_TIME_MINS, 0);
        write_config(&config_path, &default_config);
    }
}

fn punch_in() {
    let now: DateTime<Utc> = Utc::now();
    let day_path: String = get_day_file_path(&now);
    if let Ok(_) = read_day(&day_path) {
        println!("You've already clocked in for the day!");
    }
    else {
        let new_day: Day = Day::new(&now);
        println!("Clocking in for the day at '{}'", &new_day.get_day_start_as_str());
        write_day(&day_path, &new_day);
    }
}

fn punch_out() {
    let now: DateTime<Utc> = Utc::now();
    let day_path: String = get_day_file_path(&now);
    if let Ok(mut day) = read_day(&day_path) {
        if let Ok(_) = day.end_day_at(&now) {
            println!("Punching out for the day at '{}'", &day.get_day_end_as_str().unwrap().trim());
            println!("Time done: {}", day.get_time_done().expect("Day is over, we should be able to calculate time done!"));
            write_day(&day_path, &day);
            update_time_behind(day)
        }
        else {
            println!("Can't punch out: Already punched out for the day!")
        }
    }
    else {
        println!("Can't punch out: You haven't punched in for the day yet!");
    }
}

fn take_break() {
    let now: DateTime<Utc> = Utc::now();
    let day_path: String = get_day_file_path(&now);
    if let Ok(mut day) = read_day(&day_path) {
        if let Ok(_) = day.start_break(&now) {
            println!("Taking a break at '{}'", &now);
            write_day(&day_path, &day);
            day.end_day_at(&now).expect("We should be able to end the day");
            summarise_time(&day);
        }
        else {
            println!("Can't take a break: Already on a break!")
        }
    }
    else {
        println!("Can't take a break: You haven't punched in for the day yet!");
    }
}

fn resume() {
    let now: DateTime<Utc> = Utc::now();
    let day_path: String = get_day_file_path(&now);
    if let Ok(mut day) = read_day(&day_path) {
        if let Ok(_) = day.end_current_break_at(&now) {
            println!("Back to work at '{}'", &now);
            write_day(&day_path, &day);
            day.end_day_at(&now).expect("We should be able to end the day");
            summarise_time(&day);
        }
        else {
            println!("Can't end the break: Not on a break!")
        }
    }
    else {
        println!("Can't end the break: You haven't even punched in for the day yet!");
    }
}


fn summary() {
    let now: DateTime<Utc> = Utc::now();
    let day_path: String = get_day_file_path(&now);
    if let Ok(mut day) = read_day(&day_path) {
        day.end_day_at(&now).expect("We should be able to end the day");
        summarise_time(&day);
    }
    else {
        println!("Can't summarise the day because you haven't punched in for the day yet!");
    }
}


fn summarise_time(day: &Day) {
    let time_so_far: i64 = day.get_time_done().expect("Day is over so we should be able to calculate time done!");
    let mut config: Config = get_config();
    let time_left: i64 = config.day_in_minutes() - time_so_far;
    config.update_minutes_behind(time_left);

    println!("Time done today: {}", time_so_far);
    println!("Time left today: {}", time_left);
    println!("Minutes behind overall: {}", config.minutes_behind());
    println!("Minutes behind since last fall behind: {}", config.minutes_behind_non_neg());
}


fn update_time_behind(day: Day) {
    if day.has_ended() {
        let time_so_far: i64 = day.get_time_done().expect("Day is over, we should be able to calculate time done!");
        let mut config: Config = get_config();
        let time_left: i64 = config.day_in_minutes() - time_so_far;
        config.update_minutes_behind(time_left);

        println!("Time done today: {}", time_so_far);
        println!("Time left today: {}", time_left);
        println!("Minutes behind overall: {}", config.minutes_behind());
        println!("Minutes behind since last fall behind: {}", config.minutes_behind_non_neg());
        update_config(config);
    }
    else {
        panic!("Can't update time behind: The day isn't over yet")
    }
}


#[allow(dead_code)]
fn get_current_day() -> Result<Day, String> {
    let now: DateTime<Utc> = Utc::now();
    let day_path: String = get_day_file_path(&now);
    if let Ok(day) = read_day(&day_path) {
        return Ok(day);
    }
    else {
        return Err("Can't get current day. Have you punched in?".to_string());
    }
}


fn get_config() -> Config {
    let config_path: String = expand_path(&(BASE_DIR.to_owned() + &(CONFIG_FILE.to_owned())));
    let config: Config = read_config(&config_path);
    return config;
}


fn update_config(config: Config) {
    let config_path: String = expand_path(&(BASE_DIR.to_owned() + &(CONFIG_FILE.to_owned())));
    write_config(&config_path, &config)
}


fn get_day_file_path(now: &DateTime<Utc>) -> String {
    let day_string: String = now.format(DATE_FMT).to_string();
    return expand_path(BASE_DIR) + &(DAILY_DIR.to_string()) + &day_string;
}
