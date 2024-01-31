use chrono::prelude::{DateTime, Local};
use serde::{Serialize, Deserialize};

use crate::units::interval::{Dt,Interval};

#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct Note {
    time: Dt,
    msg: String,
}

impl Note {
    pub fn new(time: &DateTime<Local>, msg: &String) -> Self {
        return Note {
            time: Dt(*time),
            msg: msg.to_string(),
        };
    }
}

#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct TimeBlock {
    task_name: String,
    interval: Interval,
    notes: Vec<Note>,
}

impl TimeBlock {
    pub fn new(task_name: String, start: &DateTime<Local>) -> Self {
        return Self {
            task_name: task_name,
            interval: Interval::new(start),
            notes: Vec::new(),
        };
    }

    #[allow(dead_code)]
    pub fn get_task_name(&self) -> String {
        return self.task_name.clone();
    }

    pub fn end_at(&mut self, end: &DateTime<Local>) {
        self.interval.end_at(end);
    }

    #[allow(dead_code)]
    pub fn has_end(&self) -> bool {
        return self.interval.has_end();
    }

    #[allow(dead_code)]
    pub fn get_start(&self) -> Dt {
        return self.interval.get_start();
    }

    #[allow(dead_code)]
    pub fn get_start_as_str(&self) -> String {
        return self.interval.get_start_as_str();
    }

    #[allow(dead_code)]
    pub fn get_end(&self) -> Option<Dt> {
        return self.interval.get_end();
    }

    #[allow(dead_code)]
    pub fn get_end_as_str(&self) -> Option<String> {
        return self.interval.get_end_as_str();
    }

    #[allow(dead_code)]
    pub fn as_string(&self) -> String {
        return serde_yaml::to_string(&self).unwrap();
    }

    #[allow(dead_code)]
    pub fn from_string(yaml_str: &String) -> Self {
        return serde_yaml::from_str(yaml_str).unwrap();
    }

    pub fn get_length_secs(&self) -> Option<i64> {
        return self.interval.get_length_secs();
    }

    pub fn get_length_mins(&self) -> Option<i64> {
        return self.interval.get_length_mins();
    }

    pub fn add_note(&mut self, time: &DateTime<Local>, msg: &String) {
        let new_note: Note = Note::new(time, msg);
        self.notes.push(new_note);
    }
}
