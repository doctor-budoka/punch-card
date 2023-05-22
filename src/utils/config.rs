use serde::{Serialize,Deserialize};
use std::path::Path;
use crate::utils::file_io::{expand_path,write_file,read_file,BASE_DIR};

pub const CONFIG_FILE: &str = "punch.cfg";
const DEFAULT_TIME_MINS: i64 = 480;

#[derive(Debug,Serialize,Deserialize)]
pub struct Config {
    day_in_minutes: i64,
    minutes_behind: i64,
    minutes_behind_non_neg: u64,
}

impl Config {
    pub fn new(day_length: i64, minutes_behind: i64) -> Self {
        return Self {
            day_in_minutes: day_length, 
            minutes_behind: minutes_behind,
            minutes_behind_non_neg: if minutes_behind < 0 {0} else {minutes_behind} as u64,
        }
    }

    pub fn as_string(&self) -> String {
        return serde_yaml::to_string(&self).unwrap();
    }

    pub fn try_from_string(yaml_str: &String) -> Result<Config, serde_yaml::Error> {
        return serde_yaml::from_str(yaml_str);
    }

    pub fn from_string(yaml_str: &String) -> Self {
        return Self::try_from_string(yaml_str).unwrap();
    }

    pub fn day_in_minutes(&self) -> i64 {
        return self.day_in_minutes;
    }

    pub fn minutes_behind(&self) -> i64 {
        return self.minutes_behind;
    }

    pub fn minutes_behind_non_neg(&self) -> u64 {
        return self.minutes_behind_non_neg;
    }

    pub fn update_minutes_behind(&mut self, delta: i64) {
        let true_time_behind: i64 = self.minutes_behind() + delta;
        let non_neg_time_behind: i64 = self.minutes_behind_non_neg() as i64 + delta;
        let new_non_neg_time_behind: u64 = if true_time_behind < 0 {0} else {non_neg_time_behind} as u64;
        self.minutes_behind = true_time_behind;
        self.minutes_behind_non_neg = new_non_neg_time_behind;
    }
}

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
        let default_config: Config = Config::new(DEFAULT_TIME_MINS, 0);
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
