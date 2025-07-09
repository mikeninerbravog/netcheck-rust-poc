use std::fs::{File, OpenOptions};
use std::io::Write;
use std::time::SystemTime;

use humantime::format_rfc3339;
use serde::Serialize;
use sysinfo::{DiskExt, System, SystemExt};

#[derive(Serialize)]
struct PerfStat {
    timestamp: String,
    total_memory: u64,
    used_memory: u64,
    total_swap: u64,
    used_swap: u64,
    load_avg: (f64, f64, f64),
    disk_total: u64,
    disk_used: u64,
}

fn log_run(summary: &str) {
    let timestamp = format_rfc3339(SystemTime::now()).to_string();
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("logs/perfstat.log")
        .expect("Unable to open log file");
    writeln!(file, "[{}] {}", timestamp, summary).unwrap();
}

fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();

    let total_memory = sys.total_memory();
    let used_memory = sys.used_memory();
    let total_swap = sys.total_swap();
    let used_swap = sys.used_swap();
    let load_avg = sys.load_average();

    let disk_total = sys.disks().iter().map(|d| d.total_space()).sum();
    let disk_used = sys.disks().iter().map(|d| d.total_space() - d.available_space()).sum();

    let timestamp = format_rfc3339(SystemTime::now()).to_string();

    let stats = PerfStat {
        timestamp: timestamp.clone(),
        total_memory,
        used_memory,
        total_swap,
        used_swap,
        load_avg: (load_avg.one, load_avg.five, load_avg.fifteen),
        disk_total,
        disk_used,
    };

    // Print to terminal
    println!("📊 perfstat @ {}", timestamp);
    println!(
        "- Memory: {}/{} KB",
        used_memory, total_memory
    );
    println!(
        "- Swap:   {}/{} KB",
        used_swap, total_swap
    );
    println!(
        "- Load Average: 1m: {:.2}, 5m: {:.2}, 15m: {:.2}",
        load_avg.one, load_avg.five, load_avg.fifteen
    );
    println!(
        "- Disk Usage: {} / {} bytes",
        disk_used, disk_total
    );

    // Save JSON
    let json = serde_json::to_string_pretty(&stats).expect("Failed to serialize JSON");
    let mut file = File::create("output/stats.json").expect("Unable to write output JSON");
    file.write_all(json.as_bytes()).unwrap();

    // Log summary
    let summary = format!(
        "Memory: {}/{} | Swap: {}/{} | Load: {:.2}/{:.2}/{:.2} | Disk: {} used",
        used_memory, total_memory, used_swap, total_swap,
        load_avg.one, load_avg.five, load_avg.fifteen,
        disk_used
    );
    log_run(&summary);
}
