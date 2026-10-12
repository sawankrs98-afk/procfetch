pub mod cpu;
pub mod memory;

pub use cpu::{cpuinfo, fetch_cpu_info};
pub use memory::{fetch_memory_info, memory_info};

pub mod os;
pub use os::{fetch_os_info, OsInfo};

pub mod uptime;
pub use uptime::fetch_uptime_info;
