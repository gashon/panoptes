use std::error::Error;
use std::process::Command;

pub fn get_current_window() -> Result<String, Box<dyn Error>> {
    // Run the AppleScript that fetches the name of the frontmost application
    let apple_script = r#"tell application "System Events" to get the name of every process whose frontmost is true"#;
    let output = Command::new("osascript")
        .arg("-e")
        .arg(apple_script)
        .output()?;
    
    if !output.status.success() {
        return Err(format!("Failed to get active window name: {}", String::from_utf8_lossy(&output.stderr)).into());
    }

    let active_window_name = String::from_utf8(output.stdout)?.trim().to_string();
    
    if active_window_name.is_empty() {
        return Err("No active window found".into());
    }

    Ok(active_window_name)
}
