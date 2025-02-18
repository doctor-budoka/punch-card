pub fn render_seconds_human_readable(secs: i64) -> String {
    return format!("{} m {} s", secs / 60, secs % 60);
}
