use std::env::args;
use chrono::prelude::{DateTime, Utc};

mod utils;
use utils::{create_base_dir_if_not_exists, create_daily_dir_if_not_exists, Config, create_default_config_if_not_exists, get_config, update_config, Day, write_day, read_day, get_current_day};

#[derive(PartialEq)]
enum SubCommand {
    In,
    Out,
    Pause,
    Resume,
    Summary,
    View,
    Edit,
}

impl SubCommand {
    fn from_string(name: &String) -> Self {
        return match name.to_owned().trim() {
            "in" => Self::In,
            "out" => Self::Out,
            "pause" => Self::Pause,
            "resume" => Self::Resume,
            "summary" => Self::Summary,
            "view" => Self::View,
            "edit" => Self::Edit,
            other => panic!("{other} is not a valid subcommand!"),
        }
    }

}

fn main() {
    let env_args: Vec<String> = args().collect();
    let command_name: &String = &env_args[1];
    let command: SubCommand = SubCommand::from_string(command_name);

    setup();

    let now: DateTime<Utc> = Utc::now();
    run_command(command, now);
}

fn setup() {
    create_base_dir_if_not_exists();
    create_daily_dir_if_not_exists();
    create_default_config_if_not_exists();
}

fn run_command(command: SubCommand, now: DateTime<Utc>) {
    if command == SubCommand::In {
        punch_in(&now);
    }
    else {
        let possible_day: Result<Day, String> = get_current_day(&now);
        if let Err(msg) = possible_day {
            println!("{}", msg);
            return
        }
        let day: Day = possible_day.unwrap();

        match command {
            SubCommand::Out => punch_out(&now, day),
            SubCommand::Pause=> take_break(&now, day),
            SubCommand::Resume => resume(&now, day),
            SubCommand::Summary => summary(&now, day),
            SubCommand::View => view_day(day),
            SubCommand::Edit => edit_day(day),
            SubCommand::In => unreachable!("We shouldn't be processing punch in here"),
        }
    }
}

fn punch_in(now: &DateTime<Utc>) {
    if let Ok(_) = read_day(now) {
        println!("You've already clocked in for the day!");
    }
    else {
        let new_day: Day = Day::new(&now);
        println!("Clocking in for the day at '{}'", &new_day.get_day_start_as_str());
        write_day(&new_day);
    }
}

fn punch_out(now: &DateTime<Utc>, mut day: Day) {
    if let Ok(_) = day.end_day_at(&now) {
        println!("Punching out for the day at '{}'", &day.get_day_end_as_str().unwrap().trim());
        println!("Time done: {}", day.get_time_done().expect("Day is over, we should be able to calculate time done!"));
        write_day(&day);
        update_time_behind(day)
    }
    else {
        println!("Can't punch out: Already punched out for the day!")
    }
}

fn take_break(now: &DateTime<Utc>, mut day: Day) {
    if let Ok(_) = day.start_break(&now) {
        println!("Taking a break at '{}'", &now);
        write_day(&day);
        day.end_day_at(&now).expect("We should be able to end the day");
        summarise_time(&day);
    }
    else {
        print!("Can't take a break: Already on a break!")
    }
}

fn resume(now: &DateTime<Utc>, mut day: Day) {
    if let Ok(_) = day.end_current_break_at(&now) {
        println!("Back to work at '{}'", &now);
        write_day(&day);
        day.end_day_at(&now).expect("We should be able to end the day");
        summarise_time(&day);
    }
    else {
        println!("Can't end the break: Not on a break!")
    }
}

fn view_day(day: Day) {
    println!("Here's the day so far: \n");
    println!("{}", day.as_string());
}

fn edit_day(day: Day) {
    let path: String = day.get_day_path();
    println!("Opening day in vim...");
    {
        std::process::Command::new("vim")
        .arg(path)
        .spawn()
        .expect("Error: Failed to run editor")
        .wait()
        .expect("Error: Editor returned a non-zero status");
    }
    println!("Vim closed.");
}


fn summary(now: &DateTime<Utc>, mut day: Day) {
    let end_result = day.end_day_at(&now);
    match end_result {
        Ok(_) => (),
        _ => (),
    }
    summarise_time(&day);
}


fn summarise_time(day: &Day) {
    let time_so_far: i64 = day.get_time_done().expect("Day is over so we should be able to calculate time done!");
    let mut config: Config = get_config();
    let time_left: i64 = config.day_in_minutes() - time_so_far;
    config.update_minutes_behind(time_left);

    println!("Time done today: {}", time_so_far);
    println!("Time left today: {}", time_left);
    println!("Minutes behind overall: {}", config.minutes_behind());
    println!("Minutes behind since last fall behind: {}", config.minutes_behind_non_neg());
}


fn update_time_behind(day: Day) {
    if day.has_ended() {
        let time_so_far: i64 = day.get_time_done().expect("Day is over, we should be able to calculate time done!");
        let mut config: Config = get_config();
        let time_left: i64 = config.day_in_minutes() - time_so_far;
        config.update_minutes_behind(time_left);

        println!("Time done today: {}", time_so_far);
        println!("Time left today: {}", time_left);
        println!("Minutes behind overall: {}", config.minutes_behind());
        println!("Minutes behind since last fall behind: {}", config.minutes_behind_non_neg());
        update_config(config);
    }
    else {
        panic!("Can't update time behind: The day isn't over yet")
    }
}
