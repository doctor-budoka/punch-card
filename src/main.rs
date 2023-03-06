use std::fs::{File, OpenOptions, create_dir_all,read_to_string};
use std::io::Write;
use std::path::Path;
use chrono::TimeZone;
use chrono::prelude::{DateTime, Utc};
use std::env::{var, args};

mod day;
use day::{Day, Interval};

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
        let default_content: String = format!("DAY_IN_MINUTES={DEFAULT_TIME_MINS}\nTIME_BEHIND=0");
        write_file(&config_path, default_content);
    }
}

fn create_dir_if_not_exists(path: &str)  {
    let dir_expanded: String = expand_path(path);
    if !Path::new(&dir_expanded).exists() {
        let expect_msg: String = format!("Unable to create directory: '{path}'");
        create_dir_all(dir_expanded).expect(&expect_msg);
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

fn write_file(path: &str, contents: String) {
    let path_to_write: String = expand_path(path);
    let file_result: Result<File, std::io::Error> = OpenOptions::new().create_new(true).write(true).open(path_to_write);
    if let Ok(mut file) = file_result {
        file.write_all(contents.as_bytes()).expect("Couldn't write to file!");
    }
    else {
        panic!("Couldn't create file {path}");
    }
}


fn read_file(path: &str) -> Result<String,std::io::Error> {
    let path_to_read = expand_path(path);
    return read_to_string(path_to_read);
}

fn expand_path(path: &str) -> String {
    return if path.starts_with("~/") {
        var("HOME").unwrap() + &path[1..]
    }else {path.to_string()};
}

