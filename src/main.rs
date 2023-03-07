use std::env::args;
use std::path::Path;
use chrono::TimeZone;
use chrono::prelude::{DateTime, Utc};

mod utils;
use utils::{Config, write_config, read_config, create_dir_if_not_exists, expand_path, write_file, read_file};

const DEFAULT_TIME_MINS: u32 = 480;
const BASE_DIR: &str = "~/.punch-card/";
const DAILY_DIR: &str = "days/";
const CONFIG_FILE: &str = "punch.cfg";

const DATE_FMT: &str = "%Y-%m-%d";
const DATETIME_FMT: &str = "%Y-%m-%d %H:%M:%S";



enum SubCommand {
    In,
    Out,
    Pause,
    Resume,
}

impl SubCommand {
    fn from_string(name: &String) -> Self {
        return match name.to_owned().trim() {
            "in" => Self::In,
            "out" => Self::Out,
            "pause" => Self::Pause,
            "resume" => Self::Resume,
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
    if Path::new(&day_path).exists() {
        println!("You've already clocked in for the day!");
    }
    else {
        let now_string: String = now.format(DATETIME_FMT).to_string();
        println!("Clocking in for the day at '{}'", &now_string);
        let contents: String = format!("{now_string}=start");
        write_file(&day_path, contents);
    }
}

fn punch_out() {
    let now: DateTime<Utc> = Utc::now();
    let day_path: String = get_day_file_path(&now);
    if let Ok(day_raw) = read_file(&day_path) {
        let now_string: String = now.format(DATETIME_FMT).to_string();
        println!("Clocking out for the day at '{}'", &now_string);
        let time_string: String = day_raw.trim().lines().nth(0).expect("No data for today found!").to_string().replace("=start", "");
        let start_time: DateTime<Utc> = Utc.datetime_from_str(&time_string, DATETIME_FMT)
            .expect(&format!("Expected time in ISO format! Given: {}", time_string))
            .with_timezone(&Utc);
        let time_done_mins = (now - start_time).num_minutes();
        println!("Time done: {}", time_done_mins);
    }
    else {
        println!("Can't punch out: You haven't punched in for the day yet!");
    }
}

fn take_break() {
    println!("Taking a break")
}

fn resume() {
    println!("Getting back to work")
}

fn get_day_file_path(now: &DateTime<Utc>) -> String {
    let day_string: String = now.format(DATE_FMT).to_string();
    return expand_path(BASE_DIR) + &(DAILY_DIR.to_string()) + &day_string;
}
