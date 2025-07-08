use std::env;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::process;
use std::time::{Instant, SystemTime};

use humantime::format_rfc3339;
use reqwest;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
struct NetcheckResult {
    timestamp: String,
    url: String,
    success: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    status: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    duration_ms: Option<u128>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

fn log_to_file(message: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("logs/netcheck.log")
        .expect("Failed to open log file");

    writeln!(file, "{}", message).unwrap();
}

fn save_json(result: &NetcheckResult) {
    let json = serde_json::to_string_pretty(result).expect("JSON serialization failed");

    let mut file = File::create("output/netcheck.json").expect("Failed to create output file");
    file.write_all(json.as_bytes()).unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: netcheck <URL>");
        process::exit(1);
    }

    let url = &args[1];
    println!("[INFO] Checking URL: {}", url);

    let start_time = Instant::now();
    let timestamp = format_rfc3339(SystemTime::now()).to_string();

    match reqwest::blocking::get(url) {
        Ok(resp) => {
            let duration = start_time.elapsed();
            let result = NetcheckResult {
                timestamp: timestamp.clone(),
                url: url.clone(),
                success: true,
                status: Some(resp.status().as_u16()),
                duration_ms: Some(duration.as_millis()),
                error: None,
            };
            let log_line = format!(
                "[OK] {} | URL: {} | Status: {} | Time: {}ms",
                timestamp,
                url,
                resp.status(),
                duration.as_millis()
            );
            println!("{}", log_line);
            log_to_file(&log_line);
            save_json(&result);
            process::exit(0);
        }
        Err(err) => {
            let result = NetcheckResult {
                timestamp: timestamp.clone(),
                url: url.clone(),
                success: false,
                status: None,
                duration_ms: None,
                error: Some(err.to_string()),
            };
            let log_line = format!(
                "[ERROR] {} | URL: {} | Reason: {}",
                timestamp,
                url,
                err
            );
            eprintln!("{}", log_line);
            log_to_file(&log_line);
            save_json(&result);
            process::exit(1);
        }
    }
}
