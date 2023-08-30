mod monitor;

use std::time::Duration;
use std::thread::sleep;

fn main() {

    let tick = Duration::from_secs(1);

    loop {
        match monitor::get_current_window() {
            Ok(name) => println!("Current active window: {}", name),
            Err(err) => eprintln!("An error occurred: {}", err),
        }

        sleep(tick);
    }  
}
