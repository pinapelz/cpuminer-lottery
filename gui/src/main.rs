use std::thread;
use std::time::Duration;

mod gui;
mod net;

fn main() {
    // Spawn miner polling thread first
    thread::spawn(|| {
        loop {
            match net::api::get_summary() {
                Ok(summary) => {
                    println!("Hashrate: {:.2} kH/s", summary.khs_per_sec);
                }
                Err(err) => {
                    eprintln!("Failed to get summary info: {}", err);
                }
            }
            // Poll every 5 seconds
            thread::sleep(Duration::from_secs(5));
        }
    });

    // Then run the GUI on the main thread
    gui::run();
}
