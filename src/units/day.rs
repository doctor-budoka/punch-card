use std::collections::{HashMap,HashSet};
use chrono::prelude::{DateTime, Local};
use chrono::Duration;
use serde::{Serialize, Deserialize};

use crate::units::components::TimeBlock;
use crate::units::interval::{Dt,Interval, DATE_FMT, DATETIME_FMT};

use crate::utils::file_io::{
    create_dir_if_not_exists,
    expand_path, 
    read_file,
    write_file,
    FromString,
    SafeFileEdit,
    ToFile, 
    BASE_DIR};
use crate::utils::work_summary::WorkSummary;

pub const DAILY_DIR: &str = "days/";


#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct Day {
    pub overall_interval: Interval,
    pub timeblocks: Vec<TimeBlock>,
    tasks: HashMap<String, Vec<usize>>,
    breaks: Vec<usize>,
    pub on_break: bool,
    pub time_to_do: u64,
    pub summaries: Vec<WorkSummary>,
}

impl Day {
    pub fn new(start: &DateTime<Local>, initial_task: String, time_to_do: u64) -> Self {
        let initial_block: TimeBlock = TimeBlock::new(initial_task.clone(), start);
        return Self {
            overall_interval: Interval::new(start),
            timeblocks: vec![initial_block], 
            tasks: HashMap::from([(initial_task, vec![0])]),
            breaks: Vec::new(),
            on_break: false, 
            time_to_do: time_to_do,
            summaries: Vec::new(),
        };
    }

    pub fn end_day_at(&mut self, at: &DateTime<Local>) -> Result<(), &str> {
        if self.has_ended() {
            return Err("Can't end the day because the day has already ended!");
        }
        self.overall_interval.end_at(at);
        let block_result: Result<(), &str> = self.end_current_block_at(at);
        match block_result {
            Ok(_) => return Ok(()),
            Err(msg) => return Err(msg),
        };
    }

    pub fn has_ended(&self) -> bool {
        return self.overall_interval.has_end();
    }

    pub fn end_current_block_at(&mut self, at: &DateTime<Local>) -> Result<(), &str> {
        self.timeblocks.last_mut()
            .expect("Expected there to be an ongoing block!")
            .end_at(at);
        self.on_break = false;
        return Ok(());
    }

    pub fn get_latest_task_name(& self) -> String {
        return self.get_task_name(-1);
    }

    pub fn update_current_task_name(&mut self, new_task: String) -> Result<(), &str> {
        if self.on_break {
            return Err("Can't update current task while on a break!")
        }
        let current_name: String = self.get_latest_task_name();
        let current_ind: usize = self.timeblocks.len() - 1;
        
        // Remove current_ind from current_name
        if self.tasks.get(&current_name).expect("Key exists").len() == 1 {
            self.tasks.remove(&current_name);
        }
        else {
            self.tasks.get_mut(&current_name).expect("Key exists").pop();
        }
        
        // Add current_ind to new task
        if self.tasks.contains_key(&new_task) {
            self.tasks.get_mut(&new_task).expect("Key exists").push(current_ind);
        }
        else {
            self.tasks.insert(new_task.clone(), vec![current_ind]);
        }

        let update_result: Result<(), &str> = self.timeblocks.last_mut()
            .expect("Expected there to be an ongoing block!")
            .update_task_name(new_task);
        return update_result;
    }

    pub fn start_new_block(
        &mut self, 
        task_name: String, 
        at: &DateTime<Local>) 
    -> Result<(), &str> {
        if self.has_ended() {
            return Err("Can't start a new block because day is already over!");
        }
        self.end_current_block_at(at).expect("There should have been an existing block!");
        let new_block: TimeBlock = TimeBlock::new(task_name.clone(), at);
        let new_ind: usize = self.timeblocks.len();
        self.timeblocks.push(new_block);
        if self.tasks.contains_key(&task_name) {
            self.tasks.get_mut(&task_name).expect("Key exists").push(new_ind);
        }
        else {
            self.tasks.insert(task_name, vec![new_ind]);
        }
        return Ok(());
    }

    pub fn start_break_at(
        &mut self, 
        break_name: String, 
        at: &DateTime<Local>) 
    -> Result<(), &str> {
        if self.on_break {
            return Err("Can't start a break because day is already on break");
        }
        else {
            self.start_new_block(break_name, at).expect("Should be able to start a new block!");
            self.breaks.push(self.timeblocks.len() - 1);
            self.on_break = true;
            return Ok(());
        }
    }

    pub fn get_day_start(&self) -> Dt {
        return self.overall_interval.get_start();
    }

    pub fn get_day_start_as_str(&self) -> String {
        return self.get_day_start().as_string();
    }

    pub fn get_day_end(&self) -> Option<Dt> {
        return self.overall_interval.get_end();
    }

    pub fn get_task_name(&self, ind: isize) -> String {
        let out_ind: usize;
        if ind < 0 {
            let size: usize = self.timeblocks.len();
            out_ind = ((size as isize) + ind) as usize;
        }
        else {
            out_ind = ind as usize;
        }
        return self.timeblocks[out_ind].get_task_name();
    }

    pub fn get_day_end_as_str(&self) -> Option<String> {
        return match self.get_day_end() {
            Some(dt) => Some(dt.as_string()),
            None => None,
        };
    }

    pub fn as_string(&self) -> String {
        return serde_yaml::to_string(&self).unwrap();
    }

    pub fn get_day_length_secs(&self) -> Option<i64> {
        return self.overall_interval.get_length_secs() 
    }

    #[allow(dead_code)]
    pub fn get_day_length_mins(&self) -> Option<i64> {
        return self.overall_interval.get_length_mins() 
    }

    pub fn get_task_times_secs(&self) -> HashMap<String, i64> {
        return HashMap::from_iter(
            self.tasks.clone().into_iter().map(
                |(x, y): (String, Vec<usize>)| (
                    x, 
                    y.into_iter()
                    .map(
                        |i: usize| 
                        self.timeblocks[i].get_length_secs().unwrap_or(0)
                    )
                    .sum()
                )
            )
        );
    }

    pub fn get_task_times_secs_and_num_blocks(&self) -> HashMap<String, (i64, u64)> {
        return HashMap::from_iter(
            self.tasks.clone().into_iter().map(
                |(x, y): (String, Vec<usize>)| (
                    x, 
                    (
                        y.clone().into_iter()
                        .map(
                            |i: usize| 
                            self.timeblocks[i].get_length_secs().unwrap_or(0)
                        )
                        .sum(),
                        y.len() as u64
                    )
                )
            )
        );
    }

    pub fn get_total_break_time_secs(&self) -> Option<i64> {
        return match self.on_break {
            true => None,
            false => self.breaks.iter().map(|x: &usize| self.timeblocks[*x].get_length_secs()).sum(),
        };
    }

    pub fn get_time_done_secs(&self) -> Option<i64> {
        return match (self.get_day_length_secs(), self.get_total_break_time_secs()) {
            (Some(day), Some(breaks)) => Some(day - breaks),
            (_, _) => None,
        };
    }

    pub fn get_time_to_do(&self) -> u64 {
        return self.time_to_do;
    }

    pub fn get_time_left_secs(&self) -> Option<i64> {
        return match self.get_time_done_secs() {
            Some(td) => Some((self.get_time_to_do() * 60) as i64 - td),
            None => None,
        }
    }

    pub fn add_note(&mut self, time: &DateTime<Local>, msg: &String) {
        self.timeblocks.last_mut()
            .expect("Expected there to be an ongoing block!")
            .add_note(time, msg);
    }

    pub fn add_summary(&mut self, category: String, project: String, task: String, summary: String) {
        let summary: WorkSummary = WorkSummary::new(category, project, task, summary);
        self.summaries.push(summary);
    }

    pub fn get_total_timeblocks(&self) -> u64 {
        return self.timeblocks.len() as u64;
    }

    pub fn get_total_timeblocks_without_breaks(&self) -> u64 {
        return self.get_total_timeblocks() - (self.breaks.len() as u64);
    }

    pub fn get_tasks_in_chronological_order(&self) -> Vec<String> {
        let mut task_set = HashSet::new();
        let mut task_name_vec: Vec<String> =  self.timeblocks.clone().into_iter().map(|x| x.get_task_name()).collect();
        task_name_vec.retain(|x| task_set.insert(x.clone()));
        return task_name_vec;
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
    let start_time: DateTime<Local> = DateTime::parse_from_str(&time_str, DATETIME_FMT)
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
