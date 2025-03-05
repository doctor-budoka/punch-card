use std::env::args;
use std::process::exit;
use chrono::prelude::{DateTime, Local};

mod commands;
mod units;
mod utils;
use crate::units::day::{create_daily_dir_if_not_exists,get_current_day,Day};
use crate::commands::core::{
    punch_in,
    punch_out,
    punch_back_in,
    take_break,
    resume,
    view_day,
    view_past,
    edit_day,
    switch_to_new_task,
    update_current_task_name,
    add_note_to_today,
    add_summary_to_today,
    view_config,
    edit_config,
};
use crate::commands::day_summaries::{summary, summary_past, summarise_week, summarise_days};
use crate::utils::file_io::create_base_dir_if_not_exists;
use crate::utils::config::create_default_config_if_not_exists;

const VERSION: &str = "2.5.0";


#[derive(PartialEq,Clone)]
enum SubCommand {
    In(Vec<String>),
    Out(Vec<String>),
    BackIn(Vec<String>),
    Pause(Vec<String>),
    Resume(Vec<String>),
    Summary(Vec<String>),
    SummaryPast(Vec<String>),
    SummariseWeek(Vec<String>),
    SummariseDays(Vec<String>),
    View(Vec<String>),
    ViewPast(Vec<String>),
    Edit(Vec<String>),
    Task(Vec<String>),
    Note(Vec<String>),
    EditConfig(Vec<String>),
    ViewConfig(Vec<String>),
    AddSummary(Vec<String>),
    UpdateTask(Vec<String>),
    Version(Vec<String>),
    Invalid(String),
}

impl SubCommand {
    fn from_string(name: &String, other_args: Vec<String>) -> Self {
        return match name.to_owned().to_lowercase().trim() {
            "in" => Self::In(other_args),
            "out" => Self::Out(other_args),
            "back-in" => Self::BackIn(other_args),
            "pause" => Self::Pause(other_args),
            "resume" => Self::Resume(other_args),
            "summary" => Self::Summary(other_args),
            "summary-past" => Self::SummaryPast(other_args),
            "summarise-week" => Self::SummariseWeek(other_args),
            "summarise-days" => Self::SummariseDays(other_args),
            "view" => Self::View(other_args),
            "view-past" => Self::ViewPast(other_args),
            "edit" => Self::Edit(other_args),
            "task" => Self::Task(other_args),
            "note" => Self::Note(other_args),
            "edit-config" => Self::EditConfig(other_args),
            "view-config" => Self::ViewConfig(other_args),
            "add-summary" => Self::AddSummary(other_args),
            "update-task" => Self::UpdateTask(other_args),
            "version" | "-v" | "--version" => Self::Version(other_args),
            other => Self::Invalid(other.to_string()),
        }
    }
    
    fn to_string(self) -> String {
        return match self {
            Self::In(_) => "in",
            Self::Out(_) => "out",
            Self::BackIn(_) => "back-in",
            Self::Pause(_) => "pause",
            Self::Resume(_) => "resume",
            Self::Summary(_) => "summary",
            Self::SummaryPast(_) => "summary-past",
            Self::SummariseWeek(_) => "summarise-week",
            Self::SummariseDays(_) => "summarise-days",
            Self::View(_) => "view",
            Self::ViewPast(_) => "view-past",
            Self::Edit(_) => "edit",
            Self::Task(_) => "task",
            Self::Note(_) => "note",
            Self::EditConfig(_) => "edit-config",
            Self::ViewConfig(_) => "view-config",
            Self::AddSummary(_) => "add-summary",
            Self::UpdateTask(_) => "update-task",
            Self::Version(_) => "version",
            Self::Invalid(_) => "invalid",
        }.to_string();
    }

    fn get_allowed_strings() -> Vec<String> {
        return Vec::from(
            [
                "in", "out", "back-in", "pause", "resume", "summary", "summary-past",
                "summarise-week", "summarise-days", "view", "view-past", "edit",
                "task", "note", "edit-config", "add-summary", "update-task",
                "version", "-v", "--version"
            ].map(|x: &str| x.to_string())
        )
    }
}

fn main() {
    let env_args: Vec<String> = args().collect();
    let command_name: &String;

    if let Some(name) = env_args.get(1) {
        command_name = name;
    } else {
        handle_invalid_cmd(" ");
        return;
    }
    let other_args: Vec<String> = env_args[2..].to_vec();
    let command: SubCommand = SubCommand::from_string(command_name, other_args);

    setup();

    let now: DateTime<Local> = Local::now();
    run_command(command, now);
}

fn setup() {
    create_base_dir_if_not_exists();
    create_daily_dir_if_not_exists();
    create_default_config_if_not_exists();
}

fn run_command(command: SubCommand, now: DateTime<Local>) {
    let mut processed: bool = true;
    match command.clone() {
        SubCommand::Version(_) => println!("Current punch-card version: {}", VERSION),
        SubCommand::Invalid(original) => handle_invalid_cmd(&original),
        SubCommand::In(other_args) => punch_in(&now, other_args),
        SubCommand::ViewPast(other_args) => view_past(other_args),
        SubCommand::SummaryPast(other_args) => summary_past(other_args),
        SubCommand::EditConfig(_) => edit_config(),
        SubCommand::ViewConfig(_) => view_config(),
        SubCommand::SummariseWeek(other_args) => summarise_week(other_args),
        SubCommand::SummariseDays(other_args) => summarise_days(other_args),
        _ => {processed = false},
    }
    if processed {
        exit(0);
    }

    let possible_day: Result<Day, String> = get_current_day(&now);
    if let Err(msg) = possible_day {
        eprintln!("{}", msg);
        exit(1);
    }
    let day: Day = possible_day.unwrap();

    match command {
        SubCommand::Out(_) => punch_out(&now, day),
        SubCommand::BackIn(other_args) => punch_back_in(&now, other_args, day),
        SubCommand::Pause(other_args) => take_break(&now, other_args, day),
        SubCommand::Resume(other_args) => resume(&now, other_args, day),
        SubCommand::Summary(_) => summary(&now, day),
        SubCommand::View(_) => view_day(day),
        SubCommand::Edit(_) => edit_day(day),
        SubCommand::Task(other_args) => switch_to_new_task(&now, day, other_args),
        SubCommand::Note(other_args) => add_note_to_today(&now, day, other_args),
        SubCommand::AddSummary(other_args) => add_summary_to_today(day, other_args),
        SubCommand::UpdateTask(other_args) => update_current_task_name(&now, day, other_args),
        SubCommand::Version(_) => unreachable!("`punch version/--version/-v` commands should already be processed."),
        _ => unreachable!("'punch {}' commands shouldn't be processed here.", command.to_string()),
    }
}

fn handle_invalid_cmd(command: &str) {
    eprintln!("'{}' is not a valid subcommand for punch. Try one of the following:", command);
    for str_subcommand in SubCommand::get_allowed_strings() {
        eprintln!("\t{}", str_subcommand);
    }
    exit(1);
}
