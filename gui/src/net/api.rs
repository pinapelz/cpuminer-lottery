use std::io::{Read, Write};
use std::net::TcpStream;

#[derive(Default)]
pub struct APISummary{
    pub name: String,
    pub version: String,
    pub api: String,
    pub algo: String,
    pub cpu_count: i32,
    pub url: String,
    pub hs_per_sec: f32,
    pub khs_per_sec: f32,
    pub accepted_shares: i32,
    pub rejected_shares: i32,
    pub solutions_found: i32,
    pub earnings: f32,
    pub difficulty: f32,
    pub temperature: f32,
    pub fan: i32,
    pub frequency: i32,
    pub uptime: i32, // in sec
    pub timestamp: i64,
}

fn parse_summary_data(raw_summary_message: &str) -> APISummary {
    let mut summary = APISummary::default();
    let terms = raw_summary_message
        .trim_end_matches('%')
        .trim_end_matches('|')
        .split(';');

    for term in terms {
        let (key, value) = term.split_once('=').unwrap_or(("", ""));
        let value = value.trim();

        match key {
            "NAME" => summary.name = value.to_string(),
            "VER" => summary.version = value.to_string(),
            "API" => summary.api = value.to_string(),
            "ALGO" => summary.algo = value.to_string(),
            "CPUS" => summary.cpu_count = value.parse().unwrap_or(0),
            "URL" => summary.url = value.to_string(),
            "HS" => summary.hs_per_sec = value.parse().unwrap_or(0.0),
            "KHS" => summary.khs_per_sec = value.parse().unwrap_or(0.0),
            "ACC" => summary.accepted_shares = value.parse().unwrap_or(0),
            "REJ" => summary.rejected_shares = value.parse().unwrap_or(0),
            "SOL" => summary.solutions_found = value.parse().unwrap_or(0),
            "ACCMN" => summary.earnings = value.parse().unwrap_or(0.0),
            "DIFF" => summary.difficulty = value.parse().unwrap_or(0.0),
            "TEMP" => summary.temperature = value.parse().unwrap_or(0.0),
            "FAN" => summary.fan = value.parse().unwrap_or(0),
            "FREQ" => summary.frequency = value.parse().unwrap_or(0),
            "UPTIME" => summary.uptime = value.parse().unwrap_or(0),
            "TS" => summary.timestamp = value.parse().unwrap_or(0),
            _ => {}
        }
    }
    summary
}

#[derive(Default)]
pub struct ThreadStat{
    cpu_id: u32,
    hashrate: f64,
    units: String
}
#[derive(Default)]
pub struct APIThreads{
    thread_stats: Vec<ThreadStat>
}

pub fn unit_multiplier(unit: &str) -> f64 {
    match unit {
        "k/s" => 1e3,
        "M/s" => 1e6,
        "G/s" => 1e9,
        "T/s" => 1e12,
        "P/s" => 1e15,
        "E/s" => 1e18,
        "Z/s" => 1e21,
        "Y/s" => 1e24,
        _   => 1.0,
    }
}

fn parse_thread_data(raw_threads_message: &str) -> APIThreads{
    let mut new_threads_report = APIThreads::default();
    let stats = raw_threads_message.split("|");
    for stat in stats{
        let (cpu_stat, hashrate_stat) = stat.split_once(";").unwrap_or(("", ""));
        let (_, cpu_id) = cpu_stat.split_once("=").unwrap_or(("","0"));
        let (units, hashrate) = hashrate_stat.split_once("=").unwrap_or(("H/s","0.0"));
        let cpu_id_u32: u32 = cpu_id.parse().unwrap();
        let hashrate_f64: f64 = hashrate.parse().unwrap();
        new_threads_report.thread_stats.push(ThreadStat{cpu_id: cpu_id_u32, hashrate: hashrate_f64, units: units.to_string()});
    }
    new_threads_report
}

pub fn get_summary() -> Result<APISummary, Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect("127.0.0.1:4048")?;
    stream.write_all(b"summary|")?;
    let mut response = String::new();
    stream.read_to_string(&mut response)?;
    let summary = parse_summary_data(&response);
    Ok(summary)
}

pub fn get_threads() -> Result<APIThreads, Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect("127.0.0.1:4048")?;
    stream.write_all(b"threads|")?;
    let mut response = String::new();
    stream.read_to_string(&mut response)?;
    let summary = parse_thread_data(&response);
    Ok(summary)
}
