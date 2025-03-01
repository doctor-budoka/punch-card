pub fn render_seconds_human_readable(secs: i64) -> String {
    if secs >= 60 * 60 {
        let hours: i64 = secs / (60 * 60);
        let seconds_left:i64 = secs % (60 * 60);
        return format!("{} h {}", hours, render_seconds_human_readable(seconds_left));
    }
    else if secs >= 60 {
        let minutes: i64 = secs / 60;
        let seconds_left: i64 = secs % 60;
        return format!("{} m {}", minutes, render_seconds_human_readable(seconds_left));
    }
    else {
        return format!("{} s", secs);
    }

}

pub fn convert_input_to_seconds(input_str: &str) -> Result<i64, String> {
    let parse_result: Result<i64, std::num::ParseIntError> = input_str.parse::<i64>();
    if let Ok(secs) = parse_result {
        return Ok(secs);
    }
    let err_msg: String = format!("Malformed number of seconds. Should be of the form: [zh][ym]xs. Got {}", input_str);
    let mut secs: i64 = 0;
    let mut rest: String = input_str.to_lowercase().clone();
    let steps: Vec<(&str, i64)> = vec![("h", 60 * 60), ("m", 60), ("s", 60)];
    for (sep, secs_per) in steps {
        let this_split: Vec<&str> = rest.split(sep).collect();
        match this_split.len() {
            0 => unreachable!("This shouldn't happen"),
            2 => {
                let parse_result = this_split[0].trim().parse::<i64>();
                if let Err(_) = parse_result {
                    return Err(err_msg);
                }
                secs += parse_result.expect(&err_msg) * secs_per;
                rest = this_split[1].to_string();
            },
            1 => rest = this_split[0].to_string(),
            _ => return Err(format!("Malformed number of seconds. Should be of the form: [zh][ym]xs. Got {}", input_str)),
        }
    }
    return Ok(secs);
}
