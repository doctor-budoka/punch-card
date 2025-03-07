use serde::{Serialize, Deserialize};
use std::path::Path;
use crate::utils::file_io::{expand_path, write_file, read_file, BASE_DIR, FromString, ToFile, SafeFileEdit};

pub const CONFIG_FILE: &str = "punch.cfg";
const DEFAULT_TIME_MINS: i64 = 480;
const DEFAULT_PUNCH_IN_TASK: &str = "Starting-up";
const DEFAULT_BREAK_TASK: &str = "Break";
const SHOW_TIMES_IN_HOURS_DEFAULT: bool = true;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    day_in_minutes: i64,
    default_punch_in_task: String,
    default_break_task: String,
    minutes_behind: i64,
    seconds_behind_in_addition: Option<i64>,
    minutes_behind_non_neg: u64,
    editor_path: Option<String>,
    show_times_in_hours: Option<bool>
}

impl Config {
    pub fn new(
        day_length: i64,
        default_punch_in_task: String,
        default_break_task: String,
        minutes_behind: i64,
        seconds_behind_in_addition: Option<i64>,
        show_times_in_hours: Option<bool>)
        -> Self {
        return Self {
            day_in_minutes: day_length,
            default_punch_in_task: default_punch_in_task,
            default_break_task: default_break_task,
            minutes_behind: minutes_behind,
            seconds_behind_in_addition: seconds_behind_in_addition,
            minutes_behind_non_neg: if minutes_behind < 0 { 0 } else { minutes_behind } as u64,
            editor_path: Some("vim".to_string()),
            show_times_in_hours: show_times_in_hours,
        };
    }

    pub fn as_string(&self) -> String {
        return serde_yaml::to_string(&self).unwrap();
    }

    pub fn day_in_minutes(&self) -> i64 {
        return self.day_in_minutes;
    }

    pub fn get_default_punch_in_task(&self) -> &str {
        return &self.default_punch_in_task;
    }

    pub fn get_default_break_task(&self) -> &str {
        return &self.default_break_task;
    }

    pub fn minutes_behind(&self) -> i64 {
        return self.minutes_behind;
    }

    pub fn editor_path(&self) -> Option<&String> { return self.editor_path.as_ref(); }

    pub fn show_times_in_hours_or_default(&self) -> bool { 
        return self.show_times_in_hours.unwrap_or(SHOW_TIMES_IN_HOURS_DEFAULT); 
    }

    pub fn get_seconds_behind(&self) -> i64 {
        let minutes_behind: i64 = self.minutes_behind;
        let seconds_in_addition: i64 = self.seconds_behind_in_addition.unwrap_or(0);
        return minutes_behind * 60 + seconds_in_addition;
    }

    pub fn update_time_behind(&mut self, delta_secs: i64) {
        let current_time_behind: i64 = self.get_seconds_behind();
        let new_total_seconds_behind: i64 = current_time_behind + delta_secs;
        let sign: i64 = new_total_seconds_behind.signum();
        let abs_seconds_behind: i64 = new_total_seconds_behind.abs();
        let new_minutes_behind: i64 = abs_seconds_behind / 60;
        let new_seconds_in_addition: i64 = new_total_seconds_behind % 60;
        self.minutes_behind = sign * new_minutes_behind;
        self.seconds_behind_in_addition = Some(sign * new_seconds_in_addition);
    }

    pub fn update_minutes_behind(&mut self, delta: i64) {
        let true_time_behind: i64 = self.minutes_behind() + delta;
        self.minutes_behind = true_time_behind;
        self.minutes_behind_non_neg = 0;
    }
}

impl FromString<Config, serde_yaml::Error> for Config {
    fn try_from_string(yaml_str: &String) -> Result<Config, serde_yaml::Error> {
        return serde_yaml::from_str(yaml_str);
    }

    fn from_string(yaml_str: &String) -> Self {
        return Self::try_from_string(yaml_str).unwrap();
    }
}

impl ToFile for Config {
    fn get_path(&self) -> String {
        return get_config_path();
    }

    fn write(&self) {
        let path: &String = &self.get_path();
        write_config(path, self);
    }
}

impl SafeFileEdit<Config, serde_yaml::Error> for Config {}

pub fn write_config(path: &String, config: &Config) {
    write_file(path, config.as_string());
}

pub fn read_config(path: &String) -> Config {
    let yaml_str = read_file(path).unwrap();
    return Config::from_string(&yaml_str);
}

pub fn create_default_config_if_not_exists() {
    let config_path: String = expand_path(&(BASE_DIR.to_owned() + &(CONFIG_FILE.to_owned())));
    if !Path::new(&config_path).exists() {
        let default_config: Config = Config::new(
            DEFAULT_TIME_MINS,
            DEFAULT_PUNCH_IN_TASK.to_owned(),
            DEFAULT_BREAK_TASK.to_owned(),
            0,
            Some(0),
            Some(SHOW_TIMES_IN_HOURS_DEFAULT));
        write_config(&config_path, &default_config);
    }
}

pub fn get_config() -> Config {
    let config_path: String = get_config_path();
    let config: Config = read_config(&config_path);
    return config;
}

pub fn get_config_path() -> String {
    return expand_path(&(BASE_DIR.to_owned() + &(CONFIG_FILE.to_owned())));
}


pub fn update_config(config: Config) {
    let config_path: String = expand_path(&(BASE_DIR.to_owned() + &(CONFIG_FILE.to_owned())));
    write_config(&config_path, &config)
}
