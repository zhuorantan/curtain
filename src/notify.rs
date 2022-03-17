use std::path;
use std::process;
use std::io;
use std::thread;
use std::time;

fn get_notifier_path() -> io::Result<path::PathBuf> {
    process::Command::new("which")
        .arg("terminal-notifier")
        .env("PATH", "/opt/homebrew/bin:/usr/local/bin:/usr/bin:/bin:${PATH}")
        .output()
        .map(|output| {
            let path = String::from_utf8(output.stdout).unwrap();
            path::PathBuf::from(path.trim())
        })
}

fn send() {
    match get_notifier_path() {
        Ok(path) => {
            process::Command::new(path)
                .arg("-title")
                .arg("curtain")
                .arg("-message")
                .arg("Locked physical screens")
                .arg("-group")
                .arg("com.zhuorantan.curtain")
                .arg("-ignoreDnD")
                .spawn()
                .expect("Failed to send notification");
        }
        Err(err) => println!("Could not find terminal-notifier executable {}", err),
    }
}

fn remove() {
    match get_notifier_path() {
        Ok(path) => {
            process::Command::new(path)
                .arg("-remove")
                .arg("com.zhuorantan.curtain")
                .spawn()
                .expect("Failed to remove notification");
        }
        Err(err) => println!("Could not find terminal-notifier executable {}", err),
    }
}

pub fn notify(duration: u64) {
    if duration == 0 {
        return;
    }

    send();
    thread::sleep(time::Duration::from_secs(duration));
    remove();
}
