use std::collections::{HashMap,HashSet};
use chrono::prelude::{DateTime, Local};
use chrono::Duration;
use serde::{Serialize, Deserialize};

use crate::units::components::TimeBlock;
use crate::units::interval::{Dt,Interval, DATE_FMT, DATETIME_FMT};
use crate::units::components::Day;

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


#[derive(Debug,Clone)]
pub struct AggregateDay {
    pub total_time: u64,
    pub total_break_time: u64,
    pub num_breaks: u64,
    pub total_time_to_do: u64,
    pub num_days: u64,
    task_totals: HashMap<String,(u64, u64)>,
    pub starting_time_behind: i64,
}

impl AggregateDay {
    pub fn new(starting_time_behind: u64) -> Self {
        return Self {
            total_time: 0,
            total_break_time: 0,
            num_breaks: 0,
            total_time_to_do: 0,
            num_days: 0,
            task_totals: HashMap::new(),
            starting_time_behind: staring_time_behind
        };
    }

    pub fn add_day(&mut self, day: Day) -> Result<(), &str> {
        if !day.has_ended() {
            return Err("Can't aggregate a day that hasn't ended!");
        }
        self.total_time += day.get_day_length_secs();
        self.total_break_time += day.get_total_break_time_secs();
        self.total_breaks += day.get_total_timeblocks();
        self.total_time_to_do += day.get_time_to_do();
        self.num_days += 1;

        let task_summaries: HashMap<String, (i64, u64)> = day.get_task_times_secs_and_num_blocks();        
        for task_name in day.get_tasks_in_chronological_order() {
            let (time, blocks) = task_summaries.get(&task_name).unwrap();
            let (curr_time, curr_blocks) = self.task_totals.entry(task_name).or_insert((0, 0));

            self.task_totals.insert(task_name, (curr_time + time, curr_blocks + blocks));
        }
        return Ok(());
    }

    pub fn get_total_time_done(&self) -> u64 {
        return self.total_time - self.total_breaks;
    }

    pub fn get_time_behind_over_period(&self) -> i64 {
        return self.time_to_do - self.get_total_time_done();
    }

    pub fn get_time_behind_overall(&self) -> i64 {
        return self.get_time_behind_over_period() + self.starting_time_behind();
    }

    pub fn get_total_blocks(&self) -> u64 {
        return self.tasks.clone().into_iter().map(|x, (y, z)| z).sum();
    }

    pub fn get_total_non_break_blocks(&self) -> u64 {
        return self.get_total_blocks() - self.total_breaks;
    }

    pub fn render_human_readable_summary(&self, include_overall_time_behind: bool) -> String {
        let num_days_str: String = format!("Num days summarised: {}", self.num_days);
        let total_time_str: String = format!(
            "Total work time (including breaks): {}", render_seconds_human_readable(time_done_secs));
        let total_time_done_str: String = format!(
            "Total time working (excluding breaks): {}", render_seconds_human_readable(self.get_total_time_done()));
        let total_break_time_str: String = format!(
            "Total time spent on break: {}", render_seconds_human_readable(self.total_break_time));
        let total_breaks_str: String = format!("Total breaks: {}", self.total_breaks);
        let time_behind_str: String = format!(
            "Time behind over period: {}", render_seconds_human_readable(self.get_time_behind_over_period()), 
        );
        let main_summary = format!(
            "{}\n{}\n{}\n{}\n{}\n{}", 
            num_days_str, total_time_str, total_time_done_str, total_break_time_str, total_breaks_str, time_behind_str
        );
        let total_blocks_str: String = format!(
            "Total task blocks (including breaks): {}", self.get_total_blocks());
        let total_non_break_blocks_str: String = format!(
            "Total task blocks (excluding breaks): {}", self.get_total_non_break_blocks());
        let tasks_summary: String = format!(
            "{}\n{}\nTask times, blocks:", total_blocks_str, total_non_break_blocks_str
        );
        for (task_name, (time, blocks)) in self.tasks.into_iter() {
            tasks_summary += &format!("\n\t{}: {}, {} blocks", task_name, render_seconds_human_readable(time), blocks);
        }

        let full_message: String = format!("{}\n{}", main_summary, tasks_summary);
        if include_overall_time_behind {
            full_message += &format!(
                "\nTime behind overall: {}", render_seconds_human_readable(self.get_time_behind_overall()), 
            );
        }
        return full_message;
    }
}

pub fn render_seconds_human_readable(secs: i64) -> String {
    return format!("{} m {} s", secs / 60, secs % 60);
}
