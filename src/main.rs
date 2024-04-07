#![windows_subsystem = "windows"]

use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::{env, thread};
use xcap::Monitor;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = &args[1];
    loop {
        let start = SystemTime::now();
        let monitors = Monitor::all().unwrap();

        for (index, monitor) in monitors.iter().enumerate() {
            let image = monitor.capture_image().unwrap();
            let status = image.save(format!(
                "{}/{}_{}.png",
                path,
                start.duration_since(UNIX_EPOCH).unwrap().as_millis(),
                index + 1
            ));

            if let Err(e) = status {
                println!("Unable to save image: {}", e)
            }
        }

        thread::sleep(Duration::from_secs(60 * 30));
    }
}
