use std::fs;

#[derive(Debug, Clone)]
pub struct UptimeInfo {
    pub formatted : String,
    pub load_1m : f64, 
    pub load_5m : f64, 
    pub load_15m : f64,
}

pub fn fetch_uptime_info() -> UptimeInfo {
    let formatted = parse_uptime();
    let (l1, l5, l15) = parse_load_avg();

    UptimeInfo {
        formatted,
        load_1m: (l1),
        load_5m: (l5),
        load_15m: (l15),
    }
}

fn parse_uptime() -> String {
    let content = fs::read_to_string("/proc/uptime").unwrap_or_default();
    let seconds_float : f64 = content.split_whitespace().next().and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.0);

    let total_secs = seconds_float as u64;
    let days = total_secs/86400;
    let hours = (total_secs%86400) / 3600;
    let minutes = (total_secs % 3600)/60;
    let seconds = total_secs % 60;

    if days > 0 {
        format!("{}d {}h {}m", days, hours, minutes)
    }
    else if hours > 0 {
        format!("{}h {}m", hours, minutes)
    }
    else if minutes > 0 {
        format!("{}m {}s", minutes, seconds)
    } 
    else {
        format!("{}s", seconds)
    }
}

fn parse_load_avg() -> (f64, f64, f64) {
    let content = fs::read_to_string("/proc/loadavg").unwrap_or_default();
    let mut parts = content.split_whitespace();
    let l1 = parts.next().and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.0);
    let l5 = parts.next().and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.0);
    let l15 = parts.next().and_then(|s| s.parse::<f64>().ok()).unwrap_or(0.0);
    (l1, l5, l15)
}
