use std::fmt;
use std::io;
use chrono::prelude::{DateTime, Utc};
use chrono::TimeZone;
use serde::de;
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use serde::de::Visitor;

use crate::utils::file_io::{write_file,read_file};

const DATETIME_FMT: &str = "%Y-%m-%d %H:%M:%S";


struct DtUtcVisitor;

impl<'de> Visitor<'de> for DtUtcVisitor {
    type Value = DtUtc;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        return write!(formatter, "A datetime string");
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        return match Utc.datetime_from_str(&s, DATETIME_FMT) {
            Ok(time) => Ok(DtUtc::new(time.with_timezone(&Utc))),
            Err(err) => Err(E::custom("Incorrect format for string")),
        }
    }
}

#[derive(Debug,Copy,Clone)]
pub struct DtUtc(DateTime<Utc>);

impl DtUtc {
    pub fn new(time: DateTime<Utc>) -> Self {
        return DtUtc(time);
    }

    pub fn as_string(&self) -> String {
        return serde_yaml::to_string(&self).unwrap().trim().to_string();
    }

    pub fn from_string(yaml_str: &String) -> Self {
        return serde_yaml::from_str(yaml_str).unwrap();
    }

    pub fn as_dt(&self) -> DateTime<Utc> {
        return self.0;
    }
}

impl Serialize for DtUtc {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        return serializer.serialize_str(&self.0.format(DATETIME_FMT).to_string());
    }
}

impl<'de> Deserialize<'de> for DtUtc {
    fn deserialize<D>(deserializer: D) -> Result<DtUtc, D::Error>
    where
        D: Deserializer<'de>,
    {
        return deserializer.deserialize_str(DtUtcVisitor);
    }
}

#[derive(Debug,Copy,Clone,Serialize,Deserialize)]
pub struct Interval {
    start: DtUtc,
    end: Option<DtUtc>,
}

impl Interval {
    pub fn new(start: &DateTime<Utc>) -> Self {
        return Self {start: DtUtc(*start), end: None};
    }

    #[allow(dead_code)]
    pub fn new_now() -> Self {
        let now: DateTime<Utc> = Utc::now();
        return Self::new(&now);
    }

    pub fn end_at(&mut self, end: &DateTime<Utc>) {
        self.end = Some(DtUtc(*end));
    }

    #[allow(dead_code)]
    pub fn end_now(&mut self) {
        let now: DateTime<Utc> = Utc::now();
        self.end_at(&now);
    }

    pub fn has_end(&self) -> bool {
        return match self.end {
            Some(_) => true,
            None => false,
        };
    }

    pub fn get_start(&self) -> DtUtc {
        return self.start;
    }

    pub fn get_start_as_str(&self) -> String {
        return self.get_start().as_string();
    }

    pub fn get_end(&self) -> Option<DtUtc> {
        return self.end;
    }

    pub fn get_end_as_str(&self) -> Option<String> {
        return match self.get_end() {
            Some(end_time) => Some(end_time.as_string()),
            None => None,
        };
    }

    pub fn as_string(&self) -> String {
        return serde_yaml::to_string(&self).unwrap();
    }

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
pub struct Day {
    pub overall_interval: Interval,
    pub breaks: Vec<Interval>,
    pub on_break: bool,
}

impl Day {
    pub fn new(start: &DateTime<Utc>) -> Self {
        return Self {overall_interval: Interval::new(start), breaks: Vec::new(), on_break: false};
    }

    #[allow(dead_code)]
    pub fn new_now() -> Self {
        let now: DateTime<Utc> = Utc::now();
        return Self::new(&now);
    }

    pub fn end_day_at(&mut self, at: &DateTime<Utc>) -> Result<(), &str> {
        if self.overall_interval.has_end() {
            return Err("Can't end the day because the day has already ended!");
        }
        else {
            self.overall_interval.end_at(at);
            if self.on_break {
                return self.end_current_break_at(at);
            }
            return Ok(());
        }
    }

    #[allow(dead_code)]
    pub fn end_day_now(&mut self) -> Result<(), &str> {
        let now: DateTime<Utc> = Utc::now();
        return self.end_day_at(&now);
    }

    pub fn start_break(&mut self, at: &DateTime<Utc>) -> Result<(), &str> {
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
        let now: DateTime<Utc> = Utc::now();
        return self.start_break(&now);
    }

    pub fn end_current_break_at(&mut self, at: &DateTime<Utc>) -> Result<(), &str> {
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
        let now: DateTime<Utc> = Utc::now();
        return self.end_current_break_at(&now);
    }

    pub fn get_day_start(&self) -> DtUtc {
        return self.overall_interval.get_start();
    }

    pub fn get_day_start_as_str(&self) -> String {
        return self.get_day_start().as_string();
    }

    pub fn get_day_end(&self) -> Option<DtUtc> {
        return self.overall_interval.get_end();
    }

    pub fn get_day_end_as_str(&self) -> Option<String> {
        return self.overall_interval.get_end_as_str();
    }

    pub fn as_string(&self) -> String {
        return serde_yaml::to_string(&self).unwrap();
    }

    pub fn from_string(yaml_str: &String) -> Self {
        return serde_yaml::from_str(yaml_str).unwrap();
    }

    pub fn get_day_length(&self) -> Option<i64> {
        return self.overall_interval.get_length() 
    }

    pub fn get_total_break_time(&self) -> Option<i64> {
        return match self.on_break {
            true => None,
            false => self.breaks.iter().map(|x| x.get_length()).sum(),
        };
    }

    pub fn get_time_done(&self) -> Option<i64> {
        return match (self.get_day_length(), self.get_total_break_time()) {
            (Some(day), Some(breaks)) => Some(day - breaks),
            (_, _) => None,
        };
    }
}


pub fn string_as_time(time_str: &String) -> DateTime<Utc> {
    let start_time: DateTime<Utc> = Utc.datetime_from_str(&time_str, DATETIME_FMT)
    .expect(&format!("Expected time in ISO format! Given: {}", time_str))
    .with_timezone(&Utc);
    return start_time;
}

pub fn write_day(path: &str, day: &Day) {
    write_file(path, day.as_string());
}


pub fn read_day(path: &String) -> Result<Day, std::io::Error> {
    let read_result = read_file(path);
    return match read_result {
        Ok(string) => Ok(Day::from_string(&string)),
        Err(err) => Err(err),
    };
}