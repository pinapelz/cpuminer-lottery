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

pub fn get_summary() -> Result<APISummary, Box<dyn std::error::Error>> {
    let mut stream = TcpStream::connect("127.0.0.1:4048")?;
    stream.write_all(b"summary|")?;
    let mut response = String::new();
    stream.read_to_string(&mut response)?;
    let summary = parse_summary_data(&response);
    Ok(summary)
}
