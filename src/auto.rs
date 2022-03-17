use std::env;
use std::fs;
use std::thread;
use std::time;
use std::process;
use std::path;
use std::io::prelude::*;
use sysinfo::{self, SystemExt};
use dirs;
use crate::curtain;

pub fn run(message: Option<&str>) {
    println!("Started detecting Screen Sharing session");

    let mut is_connected = false;

    loop {
        let system = sysinfo::System::new_with_specifics(sysinfo::RefreshKind::new().with_processes(sysinfo::ProcessRefreshKind::new()));
        let is_screen_sharing = system.processes_by_name("screensharingd").next().is_some();
        if !is_screen_sharing {
            println!("Screen Sharing session is not detected. Exiting...");
            break;
        }

        let is_connected_new = is_screen_sharing && !curtain::is_session_locked();

        if !is_connected && is_connected_new {
            curtain::lock_screen(message);
            println!("Screen Sharing session detected. Screen is locked.");
        }
        is_connected = is_connected_new;

        thread::sleep(time::Duration::from_secs(3));
    }
}

fn get_current_exe() -> String {
    match env::current_exe() {
        Ok(path) => path.to_str().unwrap().to_string(),
        Err(e) => {
            println!("Failed to get current executable path, {}", e);
            let output = process::Command::new("which")
                .arg("curtain")
                .output()
                .expect("Failed to find curtain executable. Make sure it is in `PATH`");
            String::from_utf8(output.stdout).unwrap()
        }
    }
}

fn get_launch_agent_file_path() -> path::PathBuf {
    let home_dir = dirs::home_dir().unwrap();

    path::PathBuf::from(home_dir)
        .join("Library")
        .join("LaunchAgents")
        .join("com.zhuorantan.curtain.plist")
}

pub fn enable(message: Option<&str>) {
    let exe = get_current_exe();
    let message = message.clone().unwrap_or("");

    let daemon_content = format!(
    r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>com.zhuorantan.curtain</string>
    <key>ProgramArguments</key>
    <array>
        <string>{exe}</string>
        <string>auto</string>
        <string>run</string>
        <string>--message</string>
        <string>{message}</string>
    </array>
    <key>KeepAlive</key>
    <dict>
        <key>OtherJobEnabled</key>
        <dict>
            <key>com.apple.screensharing.agent</key>
            <true/>
        </dict>
    </dict>
</dict>
</plist>
"#);

    let launch_agent_file_path = get_launch_agent_file_path();
    let mut file = fs::File::create(&launch_agent_file_path).unwrap();
    file.write_all(daemon_content.as_bytes()).unwrap();

    println!("Saved launch agent file to {}", launch_agent_file_path.display());

    process::Command::new("launchctl")
        .arg("load")
        .arg(&launch_agent_file_path)
        .output()
        .expect("Failed to load launch agent");

    println!("Enabled Screen Sharing session detection");
}

pub fn disable() {
    curtain::unlock_screen();

    let launch_agent_file_path = get_launch_agent_file_path();

    process::Command::new("launchctl")
        .arg("unload")
        .arg(&launch_agent_file_path)
        .output()
        .expect("Failed to unload launch agent");

    if launch_agent_file_path.exists() {
        fs::remove_file(&launch_agent_file_path).unwrap();
        println!("Removed launch agent file from {}", launch_agent_file_path.display());
    }

    println!("Disabled Screen Sharing session detection");
}
