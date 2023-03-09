use serde::{Serialize,Deserialize};
use crate::utils::file_io::{write_file,read_file};

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

    pub fn from_string(yaml_str: &String) -> Self {
        return serde_yaml::from_str(yaml_str).unwrap();
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