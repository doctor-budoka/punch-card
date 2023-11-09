use chrono::prelude::{DateTime, Local};
use serde::{Serialize, Deserialize};

use crate::units::interval::{Dt,Interval, DATE_FMT, DATETIME_FMT};

#[derive(Debug,Serialize,Deserialize)]
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

#[derive(Debug,Serialize,Deserialize)]
pub struct Task {
    pub name: String,
    pub interval: Interval,
    pub desc: Option<String>,
}