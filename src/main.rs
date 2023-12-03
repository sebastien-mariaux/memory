use dialog::DialogBox;
use memory::sys_info::{get_percent_available_memory, SysInfoWrapper};
use std::process::Command;
use std::thread;
use std::time::Duration;

const WARNING_THRESHOLD: f64 = 10.0;
const ALERT_THRESHOLD: f64 = 5.0;
const LOOP_DELAY: u64 = 5;
const REFRESH_MEMORY_DELAY: u64 = 10;
const APPLICATIONS: [&str; 2] = ["tresorit", "discord"];

fn main() {
    let sys_info = SysInfoWrapper::new();
    loop {
        check_memory(&sys_info);
        thread::sleep(Duration::from_secs(LOOP_DELAY));
    }
}

fn check_memory(sys_info: &SysInfoWrapper) {
    let percent_available_memory = get_percent_available_memory(sys_info);
    println!("Total memory: {} kB", sys_info.get_total_memory());
    println!("Available memory: {} kB", sys_info.get_available_memory());
    println!("Percent available memory: {:.2}%", percent_available_memory);

    if percent_available_memory < ALERT_THRESHOLD {
        kill_applications();
        return;
    }

    if percent_available_memory < WARNING_THRESHOLD {
        send_warning();
    }
}

fn kill_applications() {
    for application in APPLICATIONS.iter() {
        kill_application(application);
        thread::sleep(Duration::from_secs(REFRESH_MEMORY_DELAY));

        let sys_info = SysInfoWrapper::new();
        let percent_available_memory = get_percent_available_memory(&sys_info);
        if percent_available_memory > ALERT_THRESHOLD {
            return;
        }
    }
}

fn kill_application(application: &&str) {
    let output = Command::new("sh")
        .arg("-c")
        .arg(format!("ps aux | grep {}", application))
        .output()
        .expect("failed to execute process");
    let output_str = String::from_utf8_lossy(&output.stdout);

    output_str.lines().for_each(|line| {
        let pid = line.split_whitespace().nth(1);
        Command::new("sh")
            .arg("-c")
            .arg(format!("kill -9 {}", pid.unwrap()))
            .output()
            .expect("failed to execute process");
    });

    let text = format!("{} has been closed to free some memory", application);
    send_message("APPLICATION CLOSED", &text);
}

fn send_warning() {
    let text = format!(
        "Available memory is below {}%.\nPlease close some applications.",
        WARNING_THRESHOLD
    );
    send_message("WARNING - LOW MEMORY", &text);
    println!("The dialog box has been closed.");
}

fn send_message(title: &str, text: &str) {
    dialog::Message::new(text)
        .title(title)
        .show()
        .expect("Could not display dialog box");
}
