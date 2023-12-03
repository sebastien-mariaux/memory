use dialog::DialogBox;
use memory::sys_info::{get_percent_available_memory, SysInfoWrapper};
use std::process::Command;
use std::thread;
use std::time::Duration;

const WARNING_THRESHOLD: f64 = 95.0;
const ALERT_THRESHOLD: f64 = 5.0;
const LOOP_DELAY: u64 = 5;

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
        send_alert();
        return;
    }

    if percent_available_memory < WARNING_THRESHOLD {
        send_warning();
        return;
    }
}

fn send_alert() {
    let text = format!(
        "Available memory is below {}%.\nPlease close some applications.",
        ALERT_THRESHOLD
    );
    send_message("ALERT - VERY LOW MEMORY", &text);
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
