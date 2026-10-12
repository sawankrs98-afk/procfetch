// ==============================================================================
// PHASE 3: COMPLETE FASTFETCH-STYLE HARDWARE FETCHER (`src/main.rs`)
// ==============================================================================
//
// This is the culmination of our project: a complete, lightweight, ultra-fast
// hardware & system info fetcher in pure Rust.
//
// Features:
// - Zero external dependencies (uses ONLY std::fs and std::io).
// - Sub-millisecond execution time.
// - Clean ASCII Art & ANSI terminal colors.
// - Full hardware stats: CPU model, cores, frequency, RAM usage bar,
//   Linux distribution, Hostname, Kernel, Uptime, and Load Average.
// ==============================================================================

mod modules;

use modules::{fetch_cpu_info, fetch_memory_info, fetch_os_info, fetch_uptime_info};

/// Generates an ASCII visual progress bar for Memory usage.
fn render_bar(percentage: f64, width: usize) -> String {
    let clamped = percentage.clamp(0.0, 100.0);
    let filled_count = ((clamped / 100.0) * (width as f64)).round() as usize;
    let empty_count = width.saturating_sub(filled_count);
    format!("[{}{}]", "=".repeat(filled_count), " ".repeat(empty_count))
}

fn main() {
    // -------------------------------------------------------------------------
    // ANSI ESCAPE SEQUENCES FOR TERMINAL COLORING
    // -------------------------------------------------------------------------
    const RESET: &str = "\x1b[0m";
    const BOLD: &str = "\x1b[1m";
    const RED: &str = "\x1b[31m";
    const GREEN: &str = "\x1b[32m";
    const YELLOW: &str = "\x1b[33m";
    const BLUE: &str = "\x1b[34m";
    const MAGENTA: &str = "\x1b[35m";
    const CYAN: &str = "\x1b[36m";
    const WHITE: &str = "\x1b[37m";

    // Gather all system metrics
    let os = fetch_os_info();
    let uptime = fetch_uptime_info();
    let cpu_res = fetch_cpu_info();
    let mem_res = fetch_memory_info();

    // EndeavourOS / Linux stylized ASCII logo (9 lines)
    let logo: [&str; 9] = [
        "      /\\      ",
        "     /  \\     ",
        "    /\\   \\    ",
        "   /      \\   ",
        "  /   ,,   \\  ",
        " /   |  |  -\\ ",
        "/_-''    ''-_\\",
        "              ",
        "              ",
    ];

    println!();

    // Header: user@hostname
    let username = std::env::var("USER").unwrap_or_else(|_| "user".to_string());
    let title = format!("{}@{}", username, os.hostname);
    let separator = "-".repeat(title.len());

    // Prepare information lines
    let cpu_line = match &cpu_res {
        Ok(cpu) => {
            if let Some(mhz) = cpu.mhz {
                format!("{} ({}) @ {:.2} GHz", cpu.model_name, cpu.logical_cores, mhz / 1000.0)
            } else {
                format!("{} ({})", cpu.model_name, cpu.logical_cores)
            }
        }
        Err(e) => format!("Error: {e}"),
    };

    let (mem_line, mem_bar_line) = match &mem_res {
        Ok(mem) => (
            format!("{:.2} GiB / {:.2} GiB ({:.1}%)", mem.used_gib(), mem.total_gib(), mem.used_percentage),
            format!("{} {:.1}%", render_bar(mem.used_percentage, 16), mem.used_percentage),
        ),
        Err(e) => (format!("Error: {e}"), String::new()),
    };

    let info_lines: [String; 9] = [
        format!("{}{}{}{}{}", BOLD, MAGENTA, title, RESET, ""),
        format!("{}{}{}", WHITE, separator, RESET),
        format!("{}OS:{}      {}", CYAN, RESET, os.distro_name),
        format!("{}Kernel:{}  {}", CYAN, RESET, os.kernel_version),
        format!("{}Uptime:{}  {}", CYAN, RESET, uptime.formatted),
        format!("{}Load:{}    {:.2}, {:.2}, {:.2}", CYAN, RESET, uptime.load_1m, uptime.load_5m, uptime.load_15m),
        format!("{}CPU:{}     {}", CYAN, RESET, cpu_line),
        format!("{}Memory:{}  {}", CYAN, RESET, mem_line),
        format!("{}RAM Bar:{} {}", CYAN, RESET, mem_bar_line),
    ];

    // Print side-by-side: Logo on left, Info on right
    for i in 0..9 {
        println!(
            "{}{}{}  {}",
            MAGENTA, logo[i], RESET, info_lines[i]
        );
    }

    // Terminal Color Palette Dots (classic neofetch style)
    println!("\n                {}{}{} {}{}{} {}{}{} {}{}{} {}{}{} {}{}{}",
        RED, "●", RESET,
        GREEN, "●", RESET,
        YELLOW, "●", RESET,
        BLUE, "●", RESET,
        MAGENTA, "●", RESET,
        CYAN, "●", RESET,
    );
    println!();
}

