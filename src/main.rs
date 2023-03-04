use std::fs::{File, OpenOptions, create_dir_all,read_to_string};
use std::io::Write;
use std::path::Path;
use chrono::prelude::{DateTime, Utc};
use std::env::{var, args};

const DEFAULT_TIME_MINS: u32 = 480;
const BASE_DIR: &str = "~/.punch-card/";
const CONFIG_FILE: &str = "punch.cfg";

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

    let base_dir_expanded: String = expand_path(BASE_DIR);
    if !Path::new(&base_dir_expanded).exists() {
        create_dir_all(base_dir_expanded).expect("Should be able to create directory!");
    }
    let config_path: String = expand_path(&(BASE_DIR.to_owned() + &(CONFIG_FILE.to_owned())));
    if !Path::new(&config_path).exists() {
        let default_content: String = format!("DAY_IN_MINUTES={DEFAULT_TIME_MINS}\nTIME_BEHIND=0");
        // let mut cfg_file = OpenOptions::new().create_new(true).write(true).open(config_path).expect("Couldn't create a new config file");
        // cfg_file.write_all(default_content.as_bytes()).expect("Couldn't write to config file!");
        write_file(&config_path, default_content);
    }

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
    let now_string: String = now.format("%Y-%m-%d %H:%M:%S").to_string();
    println!("Clocking in for the day at '{now_string}'");
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
    let path_to_write: String = expand_path(path);
    let file_result: Result<File, std::io::Error> = OpenOptions::new().create_new(true).write(true).open(path_to_write);
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

