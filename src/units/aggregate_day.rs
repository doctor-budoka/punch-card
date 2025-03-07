use std::collections::HashMap;

use crate::units::day::Day;
use crate::utils::misc::render_seconds_human_readable;


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
    pub fn new(starting_time_behind: i64) -> Self {
        return Self {
            total_time: 0,
            total_break_time: 0,
            num_breaks: 0,
            total_time_to_do: 0,
            num_days: 0,
            task_totals: HashMap::new(),
            starting_time_behind: starting_time_behind
        };
    }

    pub fn add_day(&mut self, day: Day) -> Result<(), &str> {
        if !day.has_ended() {
            return Err("Can't aggregate a day that hasn't ended!");
        }
        self.total_time += day.get_day_length_secs().expect("Day has ended so day length should be known.") as u64;
        self.total_break_time += day.get_total_break_time_secs().expect("Day has ended so day length should be known.") as u64;
        self.num_breaks += day.get_number_of_breaks().expect("Day has ended so day length should be known.");
        self.total_time_to_do += day.get_time_to_do_secs();
        self.num_days += 1;

        let task_summaries: HashMap<String, (i64, u64)> = day.get_task_times_secs_and_num_blocks();
        let old_task_totals: HashMap<String, (u64, u64)> = self.task_totals.clone();
        for task_name in day.get_tasks_in_chronological_order() {
            let (time, blocks) = task_summaries.get(&task_name).unwrap();
            let (curr_time, curr_blocks): (u64, u64) = *old_task_totals.get(&task_name).unwrap_or(&(0, 0));

            self.task_totals.insert(task_name, (curr_time + (*time as u64), curr_blocks + blocks));
        }
        return Ok(());
    }

    pub fn get_total_time_done(&self) -> u64 {
        return self.total_time - self.total_break_time;
    }

    pub fn get_time_behind_over_period(&self) -> i64 {
        return (self.total_time_to_do as i64) - (self.get_total_time_done() as i64);
    }

    pub fn get_time_behind_overall(&self) -> i64 {
        return self.get_time_behind_over_period() + self.starting_time_behind;
    }

    pub fn get_total_blocks(&self) -> u64 {
        return self.task_totals.clone().into_iter().map(|(_x, (_y, z))| z).sum();
    }

    pub fn get_total_non_break_blocks(&self) -> u64 {
        return self.get_total_blocks() - self.num_breaks;
    }

    pub fn render_human_readable_summary(&self, include_overall_time_behind: bool, show_times_in_hours: bool) -> Result<String, String> {
        let mut summary_str: String = format!("Num days summarised: {}", self.num_days);
        summary_str += &format!(
            "\nTotal work time (including breaks): {}", render_seconds_human_readable(self.total_time as i64, show_times_in_hours));
        summary_str += &format!(
            "\nTotal time working (excluding breaks): {}", render_seconds_human_readable(self.get_total_time_done() as i64, show_times_in_hours));
        summary_str += &format!(
            "\nTotal time spent on break: {}", render_seconds_human_readable(self.total_break_time as i64, show_times_in_hours));
        summary_str += "\n";
        
        summary_str += &format!(
            "\nTotal task blocks (including breaks): {}", self.get_total_blocks());
        summary_str += &format!(
            "\nTotal task blocks (excluding breaks): {}", self.get_total_non_break_blocks());
        summary_str += &format!("\nTotal breaks: {}", self.num_breaks);
        summary_str += "\n";
        summary_str += &"\nTask times, blocks:";
        for (task_name, (time, blocks)) in self.task_totals.clone().into_iter() {
            summary_str += &format!("\n\t{}: {}, {} blocks", task_name, render_seconds_human_readable(time as i64, show_times_in_hours), blocks);
        }
        summary_str += "\n";

        summary_str += &format!("\nTime to do over period: {}", render_seconds_human_readable(self.total_time_to_do as i64, show_times_in_hours));
        summary_str += &format!(
            "\nTime behind over period: {}", render_seconds_human_readable(self.get_time_behind_over_period(), show_times_in_hours), 
        );
        if include_overall_time_behind {
            summary_str += &format!(
                "\nTime behind overall: {}", render_seconds_human_readable(self.get_time_behind_overall(), show_times_in_hours), 
            );
        }
        return Ok(summary_str);
    }
}
