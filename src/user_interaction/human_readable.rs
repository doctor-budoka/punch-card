pub fn render_seconds_human_readable(secs: i64, show_times_in_hours: bool) -> String {
    let (sign, sign_str): (i64, &str) = if secs < 0 { (-1, "-") } else { (1, "") };
    let abs_secs: i64 = sign * secs;

    let hours: i64 = abs_secs / (60 * 60);
    let time_after_hours_removed: i64 = abs_secs % (60 * 60);
    let minutes: i64 = time_after_hours_removed / 60;
    let seconds: i64 = time_after_hours_removed % 60;

    if show_times_in_hours & (abs_secs >= 60 * 60) {
        return format!("{}{}h {}m {}s", sign_str, hours, minutes, seconds);
    } else if abs_secs >= 60 {
        return format!("{}{}m {}s", sign_str, hours * 60 + minutes, seconds);
    } else {
        return format!("{}{}s", sign_str, seconds);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_matrix;

    #[test_matrix(
        [true, false],
        [0, 1, 19],
        [true, false]
    )]
    fn test_less_than_a_minute(is_positive: bool, seconds: i64, show_times_in_hours: bool) {
        let test_input: i64 = if is_positive { seconds } else { -seconds };
        let out: String = render_seconds_human_readable(test_input, show_times_in_hours);
        assert_eq!(format!("{}s", test_input), out);
    }

    #[test_matrix(
        [true, false],
        [0, 1, 19],
        [1, 3, 7],
        [true, false]
    )]
    fn test_more_than_a_miute_less_than_an_hour(
        is_positive: bool,
        seconds: i64,
        minutes: i64,
        show_times_in_hours: bool,
    ) {
        let total_seconds: i64 = minutes * 60 + seconds;
        let test_input: i64 = if is_positive {
            total_seconds
        } else {
            -total_seconds
        };
        let sign: &str = if is_positive { "" } else { "-" };
        let out: String = render_seconds_human_readable(test_input, show_times_in_hours);
        assert_eq!(format!("{}{}m {}s", sign, minutes, seconds), out);
    }

    #[test_matrix(
        [true, false],
        [0, 1, 19],
        [0, 3, 7],
        [1, 3]
    )]
    fn test_more_than_an_hour_with_show_times_in_hours(
        is_positive: bool,
        seconds: i64,
        minutes: i64,
        hours: i64,
    ) {
        let total_seconds: i64 = hours * 60 * 60 + minutes * 60 + seconds;
        let test_input: i64 = if is_positive {
            total_seconds
        } else {
            -total_seconds
        };
        let sign: &str = if is_positive { "" } else { "-" };
        let out: String = render_seconds_human_readable(test_input, true);
        assert_eq!(format!("{}{}h {}m {}s", sign, hours, minutes, seconds), out);
    }

    #[test_matrix(
        [true, false],
        [0, 1, 19],
        [0, 3, 7],
        [1, 3]
    )]
    fn test_more_than_an_hour_without_show_times_in_hours(
        is_positive: bool,
        seconds: i64,
        minutes: i64,
        hours: i64,
    ) {
        let total_seconds: i64 = hours * 60 * 60 + minutes * 60 + seconds;
        let test_input: i64 = if is_positive {
            total_seconds
        } else {
            -total_seconds
        };
        let sign: &str = if is_positive { "" } else { "-" };
        let out: String = render_seconds_human_readable(test_input, false);
        assert_eq!(
            format!("{}{}m {}s", sign, hours * 60 + minutes, seconds),
            out
        );
    }
}
