use serde::{Serialize,Deserialize};
use crate::utils::file_io::{write_file,read_file};

#[derive(Debug,Serialize,Deserialize)]
pub struct Config {
    day_in_minutes: i64,
    minutes_behind: i64,
}

impl Config {
    pub fn new(day_length: i64, minutes_behind: i64) -> Self {
        return Self {
            day_in_minutes: day_length, 
            minutes_behind: minutes_behind
        }
    }

    pub fn as_string(&self) -> String {
        return serde_yaml::to_string(&self).unwrap();
    }

    pub fn from_string(yaml_str: &String) -> Self {
        return serde_yaml::from_str(yaml_str).unwrap();
    }

    pub fn day_in_minutes(&self) -> i64 {
        return self.day_in_minutes;
    }

    pub fn minutes_behind(&self) -> i64 {
        return self.minutes_behind;
    }
}

pub fn write_config(path: &String, config: &Config) {
    write_file(path, config.as_string());
}

pub fn read_config(path: &String) -> Config {
    let yaml_str = read_file(path).unwrap();
    return Config::from_string(&yaml_str);
}