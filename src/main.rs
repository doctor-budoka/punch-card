use std::fs::{File, OpenOptions, create_dir_all,read_to_string};
use std::io::Write;
use std::path::Path;
use chrono::prelude::{DateTime, Utc, Datelike, Month};
use std::env::{var, args};

const DEFAULT_TIME_MINS: u32 = 480;
const BASE_DIR: &str = "~/.punch-card/";
const CONFIG_PATH: &str = "punch.cfg";

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

    match command {
        SubCommand::In => punch_in(),
        SubCommand::Out => punch_out(),
        SubCommand::Pause=> take_break(),
        SubCommand::Resume => resume(),
    }

    // Continued program logic goes here...
}

fn punch_in() {
    let now: DateTime<Utc> = Utc::now();

    println!("Clocking in for the day");
}

fn punch_out() {
    println!("Clocking out for the day");
}

fn take_break() {
    println!("Taking a break")
}

fn resume() {
    println!("Getting back to work")
}

fn write_file(path: &str, contents: String) {
    let file_result: Result<File, std::io::Error> = OpenOptions::new().create_new(true).write(true).open(path);
    if let Ok(mut file) = file_result {
        file.write_all(contents.as_bytes()).expect("Couldn't write to file!");
    }
    panic!("Couldn't create file {path}");
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

