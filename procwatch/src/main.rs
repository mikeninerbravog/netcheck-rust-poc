use std::fs::{File, OpenOptions};
use std::io::Write;
use std::time::SystemTime;

use clap::Parser;
use humantime::format_rfc3339;
use procfs::process::all_processes;
use serde::Serialize;

/// Process Monitor CLI
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Optional name filter (partial match)
    #[arg(short, long)]
    filter: Option<String>,
}

#[derive(Serialize)]
struct ProcInfo {
    pid: i32,
    name: String,
    status: String,
}

fn main() {
    let args = Args::parse();
    let filter = args.filter.map(|f| f.to_lowercase());

    let mut results: Vec<ProcInfo> = Vec::new();

    for proc_result in all_processes().unwrap() {
        if let Ok(proc) = proc_result {
            if let Ok(stat) = proc.stat() {
                let name = stat.comm.to_string();
                let status = stat.state.to_string();
                let pid = stat.pid;

                if let Some(ref f) = filter {
                    if !name.to_lowercase().contains(f) {
                        continue;
                    }
                }

                results.push(ProcInfo { pid, name, status });
            }
        }
    }

    // Print to terminal
    for proc in &results {
        println!("[{}] {} ({})", proc.pid, proc.name, proc.status);
    }

    // Save to JSON
    if let Ok(json) = serde_json::to_string_pretty(&results) {
        let mut file = File::create("output/procs.json").expect("Unable to write file");
        file.write_all(json.as_bytes()).unwrap();
    }

    // Log run info
    log_run(&filter, results.len());
}

fn log_run(filter: &Option<String>, count: usize) {
    let timestamp = format_rfc3339(SystemTime::now()).to_string();
    let filter_info = filter
        .as_ref()
        .map(|f| format!("Filter: '{}'", f))
        .unwrap_or_else(|| "No filter".to_string());

    let msg = format!("[{}] {} | Found {} process(es)", timestamp, filter_info, count);

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("logs/procwatch.log")
        .expect("Unable to open log file");

    writeln!(file, "{}", msg).unwrap();
}
