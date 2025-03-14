use regex::Regex;
use std::collections::HashMap;

pub fn render_seconds_human_readable(secs: i64, show_times_in_hours: bool) -> String {
    let (sign, sign_str): (i64, &str) = if secs < 0 { (-1, "-") } else { (1, "") };
    let abs_secs: i64 = sign * secs;
    let abs_output: String;
    if show_times_in_hours & (abs_secs >= 60 * 60) {
        let hours: i64 = abs_secs / (60 * 60);
        let seconds_left: i64 = abs_secs % (60 * 60);
        abs_output = format!(
            "{}h {}",
            hours,
            render_seconds_human_readable(seconds_left, false)
        );
    } else if abs_secs >= 60 {
        let minutes: i64 = abs_secs / 60;
        let seconds_left: i64 = abs_secs % 60;
        abs_output = format!(
            "{}m {}",
            minutes,
            render_seconds_human_readable(seconds_left, false)
        );
    } else {
        abs_output = format!("{}s", abs_secs);
    }
    return format!("{}{}", sign_str, abs_output);
}

pub fn convert_input_to_seconds(input_str: &str) -> Result<i64, String> {
    let parse_result: Result<i64, std::num::ParseIntError> = input_str.parse::<i64>();
    if let Ok(secs) = parse_result {
        return Ok(secs);
    }
    let err_msg: String = format!(
        "Malformed number of seconds. Should be either an integer or of the form: [zh][ym]xs. Got {}", input_str
    );
    let mut secs: i64 = 0;
    let rest: String = input_str.to_lowercase().clone();

    let check_regex = Regex::new(r"^(\-)?(\d+h)?(\d+m)?(\d+s)$").unwrap();
    if !check_regex.is_match(&rest) {
        return Err(err_msg);
    }

    let units_to_secs: HashMap<&str, i64> = HashMap::from([("h", 60 * 60), ("m", 60), ("s", 1)]);
    let re = Regex::new(r"(\d+)([hms])").unwrap();
    let sign: i64 = if rest.starts_with('-') { -1 } else { 1 };
    for (_, [amount, unit]) in re.captures_iter(&rest).map(|c| c.extract()) {
        let parse_result = amount.parse::<i64>();
        if let Err(_) = parse_result {
            return Err(err_msg);
        }
        let num_unit: i64 = parse_result.unwrap();
        let multiplier_for_secs: i64 = *units_to_secs.get(unit).unwrap();
        secs += num_unit * multiplier_for_secs;
    }
    return Ok(sign * secs);
}
