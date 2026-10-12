use std::fs;

#[derive(Debug, Clone)]
pub struct OsInfo {
    pub distro_name : String,
    pub hostname : String,
    pub kernel_version : String,
}

pub fn fetch_os_info() -> OsInfo{
    OsInfo {
        distro_name: parse_distro_name(),
        hostname: parse_distro_name(),
        kernel_version: parse_distro_name(),
    }
}

fn parse_distro_name() -> String{

    // syntax 1 to parse from the diff file paths out of which one is correct...
    let content = fs::read_to_string("etc/os-release")
        .or_else(|_| fs::read_to_string("usr/lib/os-release"))
        .unwrap_or_default();

    let mut pretty_name = None;
    let mut name = None;

    for line in content.lines() {
        if let Some((key, value)) = line.split_once('=') {
            let cleaned = value.trim().trim_matches('"').trim_matches('\'');
            if key == "PRETTY_NAME" {
                pretty_name = Some(cleaned.to_string());
                break;
            }
            else if key == "NAME" && name.is_none() {
                name = Some(cleaned.to_string());
            }
        }
    }

    pretty_name.or(name).unwrap_or_else(|| "Linux".to_string())
}

fn parse_hostname() -> String{
    if let Ok(host) = fs::read_to_string("/etc/hostname") {
        let trimmed = host.trim();
        if !trimmed.is_empty() {
            return trimmed.to_string();
        }
    }
    if let Ok(host) = fs::read_to_string("/proc/sys/kernel/hostname") {
        let trimmed = host.trim();
        if !trimmed.is_empty() {
            return trimmed.to_string();
        }
    }
    return "localhost".to_string();
}

fn parse_kernel_version() -> String{
    let content = fs::read_to_string("/proc/version").unwrap_or_default();
    let mut words = content.split_whitespace();
    let _ = words.next();
    let _ = words.next();

    words.next().unwrap_or("Unknown kernel").to_string()
}