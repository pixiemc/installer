use std::env;
use std::error::Error;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

// I'm very much hoping the following works noting my experience with MacOS in general, if you run into any issues/have any suggestions
// Please make a PR <3

#[allow(dead_code)]
pub fn set_env_var(name: &str, value: &str) -> Result<(), Box<dyn Error>> {
    std::env::set_var(name, value);
    let home = env::var("HOME")?;
    let launch_agents_dir = PathBuf::from(&home).join("Library/LaunchAgents");

    fs::create_dir_all(&launch_agents_dir)?;
    let plist_path = launch_agents_dir.join(format!("com.pixie.{}.plist", name.to_lowercase()));
    let plist_content = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>setenv.{}</string>
    <key>ProgramArguments</key>
    <array>
        <string>/bin/launchctl</string>
        <string>setenv</string>
        <string>{}</string>
        <string>{}</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
</dict>
</plist>"#,
        name, name, value
    );

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&plist_path)?;

    file.write_all(plist_content.as_bytes())?;

    Command::new("launchctl")
        .arg("load")
        .arg(&plist_path)
        .output()?;

    Command::new("launchctl")
        .arg("setenv")
        .arg(name)
        .arg(value)
        .output()?;

    Ok(())
}
