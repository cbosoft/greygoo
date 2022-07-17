pub fn fmt_t(s: i64) -> String {
    if s > 604800 {
        let w = (s as f64) / 604800f64;
        format!("{:.1}w", w)
    }
    else if s > 86400 {
        let d = (s as f64) / 86400f64;
        format!("{:.1}d", d)
    }
    else if s > 3600 {
        let h = (s as f64) / 3600f64;
        format!("{:.1}h", h)
    }
    else if s > 60 {
        let m = (s as f64) / 60f64;
        format!("{:.1}m", m)
    }
    else {
        format!("{}s", s)
    }
}