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

static POLLING_DURATION_IN_SECONDS: u64 = 120;

fn make_request(http: &HttpClient, password: &str) -> Result<(), reqwest::Error> {
    let current_application = monitor::get_current_window().unwrap_or("Unknown".to_string());
    http.post("https://live.ghussein.org/api/desktop")
        .json(&Ping {
            activity: current_application,
        })
        .header(
            "Authorization",
            format!("Bearer {}", password),
        )
        .send()?;
    Ok(())
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

    let tick_duration = Duration::from_secs(POLLING_DURATION_IN_SECONDS);
    let http = HttpClient::new();

    loop {
        if let Err(err) = make_request(&http, &password) {
            eprintln!("{}", err);
        }

        sleep(tick_duration);
    }
}
