pub fn fmt_duration(d: std::time::Duration) -> String {
    let millis = d.as_millis();

    let mut duration = String::new();
    if millis > 1000 {
        duration.push_str(&format!("{}s", millis / 1000));
    }
    let millis = millis % 1000;
    if millis > 0 {
        duration.push_str(&format!("{}ms", millis));
    }

    duration
}
