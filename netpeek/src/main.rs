use std::fs::{File, OpenOptions};
use std::io::Write;
use std::net::ToSocketAddrs;
use std::time::SystemTime;

use humantime::format_rfc3339;
use reqwest::blocking::get;
use serde::Serialize;
use trust_dns_resolver::config::*;
use trust_dns_resolver::Resolver;

#[derive(Serialize)]
struct NetInfo {
    timestamp: String,
    domain: String,
    resolved_ips: Vec<String>,
    external_ip: String,
    reachable: bool,
}

fn log_run(message: &str) {
    let timestamp = format_rfc3339(SystemTime::now()).to_string();
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("logs/netpeek.log")
        .expect("Failed to open log file");
    writeln!(file, "[{}] {}", timestamp, message).unwrap();
}

fn main() {
    let domain = "aa9.online";
    let timestamp = format_rfc3339(SystemTime::now()).to_string();

    // DNS resolve
    let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default())
        .expect("Failed to create resolver");

    let mut resolved_ips: Vec<String> = vec![];
    if let Ok(response) = resolver.lookup_ip(domain) {
        for ip in response.iter() {
            resolved_ips.push(ip.to_string());
        }
    }

    // External IP
    let external_ip = get("https://httpbin.org/ip")
        .and_then(|r| r.json::<serde_json::Value>())
        .ok()
        .and_then(|json| json.get("origin").cloned())
        .unwrap_or("Unknown".into())
        .to_string();

    // Reachability (simulate ping with TCP connect)
    let reachable = format!("{}:80", domain)
        .to_socket_addrs()
        .ok()
        .and_then(|mut addrs| addrs.find(|a| std::net::TcpStream::connect_timeout(a, std::time::Duration::from_secs(2)).is_ok()))
        .is_some();

    let netinfo = NetInfo {
        timestamp: timestamp.clone(),
        domain: domain.to_string(),
        resolved_ips,
        external_ip,
        reachable,
    };

    // Print result
    println!("🌐 netpeek @ {}", timestamp);
    println!("- Domain: {}", netinfo.domain);
    println!("- Resolved IPs: {:?}", netinfo.resolved_ips);
    println!("- External IP: {}", netinfo.external_ip);
    println!("- Reachable: {}", netinfo.reachable);

    // Save JSON
    let json = serde_json::to_string_pretty(&netinfo).expect("Failed to serialize");
    let mut file = File::create("output/netpeek.json").expect("Failed to create output file");
    file.write_all(json.as_bytes()).unwrap();

    log_run(&format!(
        "{} | {:?} | ext_ip={} | reachable={}",
        domain, netinfo.resolved_ips, netinfo.external_ip, netinfo.reachable
    ));
}
