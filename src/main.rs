mod daemon;
mod monitor;

use std::env;
use std::time::Duration;
use std::thread::sleep;
use reqwest::blocking::Client as HttpClient;
use serde::Serialize;

#[derive(Serialize)]
struct Ping {
    activity: String,
}

fn main() {
    let password = match env::var("OPTES_PASSWORD") {
        Ok(value) => value,
        Err(_) => {
            eprintln!("OPTES_PASSWORD environment variable not set.");
            return;
        },
    };

    match daemon::daemonize() {
        Ok(_) => {
            println!("Successfully daemonized.");
        },
        Err(err) => {
            eprintln!("Failed to daemonize: {}", err);
            return;
        }
    }

    let tick_duration = Duration::from_secs(1);
    let http = HttpClient::new();

    loop {
        let req = || -> Result<(), reqwest::Error> {
            let current_application = monitor::get_current_window().unwrap_or("Unavailable".to_string());
            http.post("https://live.ghussein.org/api/desktop")
                .json(&Ping {
                    activity: current_application,
                })
                .header(
                    "Authorization",
                    format!("Bearer {}", password),
                )
                .send()?;
            println!("sent req");
            Ok(())
        };

        if let Err(err) = req() {
            eprintln!("{}", err);
        }

        match monitor::get_current_window() {
            Ok(name) => println!("Current active window: {}", name),
            Err(err) => eprintln!("An error occurred: {}", err),
        }

        sleep(tick_duration);
    }
}