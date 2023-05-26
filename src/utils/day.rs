use std::fmt::{self, Display};
use chrono::prelude::{DateTime, Local};
use chrono::TimeZone;
use chrono::Duration;
use serde::de;
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use serde::de::Visitor;

use crate::utils::file_io::{expand_path,write_file,read_file, BASE_DIR, create_dir_if_not_exists, FromString, ToFile, SafeFileEdit};
use crate::utils::work_summary::WorkSummary;

pub const DAILY_DIR: &str = "days/";
const DATETIME_FMT: &str = "%Y-%m-%d %H:%M:%S %z";
const DATE_FMT: &str = "%Y-%m-%d";


struct DtVisitor;

impl<'de> Visitor<'de> for DtVisitor {
    type Value = Dt;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        return write!(formatter, "A datetime string");
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        return match Local.datetime_from_str(&s, DATETIME_FMT) {
            Ok(time) => Ok(Dt::new(time.with_timezone(&Local))),
            Err(_) => Err(E::custom("Incorrect format for string")),
        }
    }
}

#[derive(Debug,Copy,Clone)]
pub struct Dt(DateTime<Local>);

impl Dt {
    pub fn new(time: DateTime<Local>) -> Self {
        return Dt(time);
    }

    pub fn as_string(&self) -> String {
        return serde_yaml::to_string(&self).unwrap().trim().to_string();
    }

    #[allow(dead_code)]
    pub fn from_string(yaml_str: &String) -> Self {
        return serde_yaml::from_str(yaml_str).unwrap();
    }

    #[allow(dead_code)]
    pub fn as_dt(&self) -> DateTime<Local> {
        return self.0;
    }
}

impl Serialize for Dt {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        return serializer.serialize_str(&self.0.format(DATETIME_FMT).to_string());
    }
}

impl<'de> Deserialize<'de> for Dt {
    fn deserialize<D>(deserializer: D) -> Result<Dt, D::Error>
    where
        D: Deserializer<'de>,
    {
        return deserializer.deserialize_str(DtVisitor);
    }
}

#[derive(Debug,Copy,Clone,Serialize,Deserialize)]
pub struct Interval {
    start: Dt,
    end: Option<Dt>,
}

impl Interval {
    pub fn new(start: &DateTime<Local>) -> Self {
        return Self {start: Dt(*start), end: None};
    }

    #[allow(dead_code)]
    pub fn new_now() -> Self {
        let now: DateTime<Local> = Local::now();
        return Self::new(&now);
    }

    pub fn end_at(&mut self, end: &DateTime<Local>) {
        self.end = Some(Dt(*end));
    }

    #[allow(dead_code)]
    pub fn end_now(&mut self) {
        let now: DateTime<Local> = Local::now();
        self.end_at(&now);
    }

    pub fn has_end(&self) -> bool {
        return match self.end {
            Some(_) => true,
            None => false,
        };
    }

    pub fn get_start(&self) -> Dt {
        return self.start;
    }

    #[allow(dead_code)]
    pub fn get_start_as_str(&self) -> String {
        return self.get_start().as_string();
    }

    pub fn get_end(&self) -> Option<Dt> {
        return self.end;
    }

    pub fn get_end_as_str(&self) -> Option<String> {
        return match self.get_end() {
            Some(end_time) => Some(end_time.as_string()),
            None => None,
        };
    }

    #[allow(dead_code)]
    pub fn as_string(&self) -> String {
        return serde_yaml::to_string(&self).unwrap();
    }

    #[allow(dead_code)]
    pub fn from_string(yaml_str: &String) -> Self {
        return serde_yaml::from_str(yaml_str).unwrap();
    }

    pub fn get_length(&self) -> Option<i64> {
        return match self.get_end() {
            Some(end_time) => Some((end_time.0 - self.start.0).num_minutes()),
            None => None, 
        }
    }
}


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
pub struct Day {
    pub overall_interval: Interval,
    pub breaks: Vec<Interval>,
    pub on_break: bool,
    pub time_to_do: u64,
    pub notes: Vec<Note>,
    pub summaries: Vec<WorkSummary>,
}

impl Day {
    pub fn new(start: &DateTime<Local>, time_to_do: u64) -> Self {
        return Self {
            overall_interval: Interval::new(start), 
            breaks: Vec::new(), 
            on_break: false, 
            time_to_do: time_to_do,
            notes: Vec::new(),
            summaries: Vec::new(),
        };
    }

    #[allow(dead_code)]
    pub fn new_now(time_to_do: u64) -> Self {
        let now: DateTime<Local> = Local::now();
        return Self::new(&now, time_to_do);
    }

    pub fn end_day_at(&mut self, at: &DateTime<Local>) -> Result<(), &str> {
        if self.overall_interval.has_end() {
            return Err("Can't end the day because the day has already ended!");
        }
        let break_result: Result<(), &str> = self.end_current_break_at(at);
        match break_result {
            _ => (),
        }
        self.overall_interval.end_at(at);
        return Ok(());
    }

    #[allow(dead_code)]
    pub fn end_day_now(&mut self) -> Result<(), &str> {
        let now: DateTime<Local> = Local::now();
        return self.end_day_at(&now);
    }

    pub fn has_ended(&self) -> bool {
        return self.overall_interval.has_end();
    }

    pub fn start_break(&mut self, at: &DateTime<Local>) -> Result<(), &str> {
        if self.has_ended() {
            return Err("Can't start a break because day is already over!")
        }

        if self.on_break {
            return Err("Can't start a break because day is already on break");
        }
        else {
            self.on_break = true;
            self.breaks.push(Interval::new(at));
            return Ok(());
        }
    }

    #[allow(dead_code)]
    pub fn start_break_now(&mut self) -> Result<(), &str> {
        let now: DateTime<Local> = Local::now();
        return self.start_break(&now);
    }

    pub fn end_current_break_at(&mut self, at: &DateTime<Local>) -> Result<(), &str> {
        if self.has_ended() {
            return Err("Can't end the break because day is already over!")
        }

        if !self.on_break {
            return Err("Can't end the break: currently not on break!");
        }
        else {
            self.breaks.last_mut().expect("Expected break to be ongoing!").end_at(at);
            self.on_break = false;
            return Ok(());
        }
    }

    #[allow(dead_code)]
    pub fn end_current_break_now(&mut self) -> Result<(), &str> {
        let now: DateTime<Local> = Local::now();
        return self.end_current_break_at(&now);
    }

    pub fn get_day_start(&self) -> Dt {
        return self.overall_interval.get_start();
    }

    pub fn get_day_start_as_str(&self) -> String {
        return self.get_day_start().as_string();
    }

    #[allow(dead_code)]
    pub fn get_day_end(&self) -> Option<Dt> {
        return self.overall_interval.get_end();
    }

    pub fn get_day_end_as_str(&self) -> Option<String> {
        return self.overall_interval.get_end_as_str();
    }

    pub fn as_string(&self) -> String {
        return serde_yaml::to_string(&self).unwrap();
    }

    pub fn get_day_length(&self) -> Option<i64> {
        return self.overall_interval.get_length() 
    }

    pub fn get_total_break_time(&self) -> Option<i64> {
        return match self.on_break {
            true => None,
            false => self.breaks.iter().map(|x: &Interval| x.get_length()).sum(),
        };
    }

    pub fn get_time_done(&self) -> Option<i64> {
        return match (self.get_day_length(), self.get_total_break_time()) {
            (Some(day), Some(breaks)) => Some(day - breaks),
            (_, _) => None,
        };
    }

    pub fn get_day_path(&self) -> String {
        return get_day_file_path(&self.get_day_start().as_dt());
    }

    pub fn get_time_to_do(&self) -> u64 {
        return self.time_to_do;
    }

    pub fn get_time_left(&self) -> Option<i64> {
        return match self.get_time_done() {
            Some(td) => Some(self.get_time_to_do() as i64 - td),
            None => None,
        }
    }

    pub fn add_note(&mut self, time: &DateTime<Local>, msg: &String) {
        let new_note: Note = Note::new(time, msg);
        self.notes.push(new_note);
    }

    pub fn add_summary(&mut self, category: String, project: String, task: String, summary: String) {
        let summary: WorkSummary = WorkSummary::new(category, project, task, summary);
        self.summaries.push(summary);
    }
}

impl FromString<Day, serde_yaml::Error> for Day {
    fn try_from_string(yaml_str: &String) -> Result<Day, serde_yaml::Error> {
        return serde_yaml::from_str(yaml_str);
    }

    fn from_string(yaml_str: &String) -> Self {
        return Self::try_from_string(yaml_str).unwrap();
    }
}

impl ToFile for Day {
    fn get_path(&self) -> String {
        return get_day_file_path(&self.get_day_start().as_dt());
    }

    fn write(&self) {
        let path: &String = &self.get_path();
        write_file(path, self.as_string());
    }
}

impl SafeFileEdit<Day, serde_yaml::Error> for Day{}

#[allow(dead_code)]
pub fn string_as_time(time_str: &String) -> DateTime<Local> {
    let start_time: DateTime<Local> = Local.datetime_from_str(&time_str, DATETIME_FMT)
    .expect(&format!("Expected time in ISO format! Given: {}", time_str))
    .with_timezone(&Local);
    return start_time;
}


pub fn get_day_file_path(now: &DateTime<Local>) -> String {
    let day_string: String = now.format(DATE_FMT).to_string();
    return expand_path(BASE_DIR) + &(DAILY_DIR.to_string()) + &day_string;
}


pub fn write_day(day: &Day) {
    let path: &String = &get_day_file_path(&day.get_day_start().as_dt());
    write_file(path, day.as_string());
}


pub fn read_day(now: &DateTime<Local>) -> Result<Day, std::io::Error> {
    let path: &String = &get_day_file_path(&now);
    let read_result: Result<String, std::io::Error> = read_file(path);
    return match read_result {
        Ok(string) => Ok(Day::from_string(&string)),
        Err(err) => Err(err),
    };
}

pub fn get_current_day(now: &DateTime<Local>) -> Result<Day, String> {
    let yesterday: DateTime<Local> = *now - Duration::days(1);
    if let Ok(day) = read_day(&now) {
        return Ok(day);
    }
    else if let Ok(day) = read_day(&yesterday) {
        return Ok(day);
    }
    else {
        return Err("Can't get current day. Have you punched in?".to_string());
    }
}

pub fn create_daily_dir_if_not_exists() {
    let daily_dir: String = BASE_DIR.to_string() + &(DAILY_DIR.to_string());
    create_dir_if_not_exists(&daily_dir);
}
