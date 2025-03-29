use regex::Regex;
use std::collections::HashMap;

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

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("0", 0 ; "just 0 given")]
    #[test_case("-23", -23 ; "negative seconds")]
    #[test_case("63", 63 ; "positive seconds")]
    fn test_just_seconds(test_input: &str, expected: i64) {
        let out: i64 = convert_input_to_seconds(test_input).unwrap();
        assert_eq!(out, expected);
    }

    #[test_case("0s", 0 ; "just 0 given")]
    #[test_case("-23s", -23 ; "negative seconds")]
    #[test_case("63s",63 ; "positive seconds")]
    fn test_seconds_str(test_input: &str, expected: i64) {
        let out: i64 = convert_input_to_seconds(test_input).unwrap();
        assert_eq!(out, expected);
    }

    #[test_case("0m0s", 0 ; "just 0 given")]
    #[test_case("-1m23s", -83 ; "negative input with minutes")]
    #[test_case("-0m23s", -23 ; "negative input without minutes")]
    #[test_case("2m63s", 183 ; "positive input with minutes and large seconds")]
    #[test_case("0m63s",63 ; "positive input without minutes and large seconds")]
    #[test_case("1m3s", 63 ; "positive input with minutes")]
    #[test_case("1m0s", 60 ; "positive input with minutes and no seconds")]
    fn test_minutes_seconds_str(test_input: &str, expected: i64) {
        let out: i64 = convert_input_to_seconds(test_input).unwrap();
        assert_eq!(out, expected);
    }

    #[test_case("0h0m0s", 0 ; "just 0 given")]
    #[test_case("-0h1m23s", -83 ; "negative input with minutes")]
    #[test_case("-0h0m23s", -23 ; "negative input without minutes")]
    #[test_case("0h2m63s", 183 ; "positive input with minutes and large seconds")]
    #[test_case("0h0m63s",63 ; "positive input without minutes and large seconds")]
    #[test_case("0h1m3s", 63 ; "positive input with minutes")]
    #[test_case("0h1m0s", 60 ; "positive input with minutes and no seconds")]
    fn test_minutes_seconds_with_zero_hours_str(test_input: &str, expected: i64) {
        let out: i64 = convert_input_to_seconds(test_input).unwrap();
        assert_eq!(out, expected);
    }

    #[test_case("-1h1m23s", -3683 ; "negative input with minutes")]
    #[test_case("-2h0m23s", -7223 ; "negative input without minutes")]
    #[test_case("1h2m63s", 3783 ; "positive input with minutes and large seconds")]
    #[test_case("3h0m63s", 10863 ; "positive input without minutes and large seconds")]
    #[test_case("1h1m3s", 3663 ; "positive input with minutes")]
    #[test_case("2h1m0s",7260 ; "positive input with minutes and no seconds")]
    fn test_minutes_seconds_with_nonzero_hours_str(test_input: &str, expected: i64) {
        let out: i64 = convert_input_to_seconds(test_input).unwrap();
        assert_eq!(out, expected);
    }

    #[test]
    fn test_errors_for_bad_string() {
        let out = convert_input_to_seconds("abc");
        assert!(out.is_err());
    }
}
