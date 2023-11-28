use std::fmt::{self};
use chrono::prelude::{DateTime, Local};
use chrono::TimeZone;
use serde::de;
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use serde::de::Visitor;

pub const DATETIME_FMT: &str = "%Y-%m-%d %H:%M:%S %z";
pub const DATE_FMT: &str = "%Y-%m-%d";


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
pub struct Dt(pub DateTime<Local>);

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
