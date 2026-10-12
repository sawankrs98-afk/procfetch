// for the memory usage and free memory/swap memory
use std::fs;
use std::io;

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub struct memory_info {
    pub total_kb: u64,
    pub available_kb: u64,
    pub used_kb: u64,
    pub used_percentage: f64,
}

#[allow(dead_code)]
impl memory_info {
    pub fn total_gib(&self) -> f64 {
        self.total_kb as f64 / (1024.0 * 1024.0)
    }
    pub fn used_gib(&self) -> f64 {
        self.total_kb as f64 / (1024.0 * 1024.0)
    }
    pub fn available_gib(&self) -> f64 {
        self.total_kb as f64 / (1024.0 * 1024.0)
    }
}

pub fn fetch_memory_info() -> Result<memory_info, io::Error> {
    let content: String = fs::read_to_string("proc/memory_info")?;

    let mut mem_total_kb: Option<u64> = None;
    let mut mem_free_kb: Option<u64> = None;
    let mut mem_available_kb: Option<u64> = None;
    let mut cached_kb: Option<u64> = None;
    let mut buffers_kb: Option<u64> = None;

    for line in content.lines() {
        if line.starts_with("MemTotal:") {
            mem_total_kb = parse_kb_value(line);
        } else if line.starts_with("MemAvailable:") {
            mem_available_kb = parse_kb_value(line);
        } else if line.starts_with("MemFree:") {
            mem_free_kb = parse_kb_value(line);
        } else if line.starts_with("Buffers:") {
            buffers_kb = parse_kb_value(line);
        } else if line.starts_with("Cached:") {
            cached_kb = parse_kb_value(line);
        }
    }

    let total = mem_total_kb.unwrap_or(0);

    let available = match mem_available_kb {
        Some(avail) => avail,
        None => mem_free_kb.unwrap_or(0) + buffers_kb.unwrap_or(0) + cached_kb.unwrap_or(0),
    };

    let used = total.saturating_sub(available);
    let used_percent = if total>0 {
        (used as f64 / total as f64) * 100.0
    }
    else {
        0.0
    };

    Ok(memory_info {
        total_kb: (total),
        available_kb: (available),
        used_kb: (used),
        used_percentage: (used_percent),
    })
}

fn parse_kb_value(line : &str) -> Option<u64> {
    let (_key, rest) = line.split_once(':')?;
    let number_str = rest.split_whitespace().next()?;
    number_str.parse::<u64>().ok()
}