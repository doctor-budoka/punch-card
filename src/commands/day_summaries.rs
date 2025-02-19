use std::process::exit;


pub fn summary(now: &DateTime<Local>, mut day: Day) {
    let end_result: Result<(), &str> = day.end_day_at(&now);
    match end_result {
        Ok(_) => (),
        _ => (),
    }
    if let Err(err_msg) = print_day_summary(day) {
        eprintln(err_msg);
        exit(1);
    }
}


fn print_day_summary(day: Day) -> Result<(), &str> {
    let config: Config = get_config();
    let time_behind_s: i64 = config.minutes_behind() * 60;
    let summary_result: Result<String, &str> = day.render_human_readable_summary(time_behind_s);
    
    return match summary_result {
        Ok(summary_str) => {
            println!(summary_str);
            Ok(())
        },
        Err(err_msg) => Err(err_msg),
    }
}
