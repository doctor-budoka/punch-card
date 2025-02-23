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
