use dialog::DialogBox;
use memory::sys_info::{get_percent_available_memory, SysInfoWrapper};
use serde::Deserialize;
use std::env;
use std::fs;
use std::process::Command;
use std::thread;
use std::time::Duration;

#[derive(Deserialize)]
struct Config {
    warning_threshold: u8,
    alert_threshold: u8,
    loop_delay: u8,
    refresh_memory_delay: u8,
    applications: Vec<String>,
}

fn get_config_file_path() -> String {
    match env::home_dir() {
        Some(path) => path.display().to_string() + "/.memory-config.toml",
        None => panic!("Impossible to get your home dir!"),
    }
}

fn main() -> std::io::Result<()> {
    if let Some(command) = std::env::args().nth(1) {
        if command == "config" {
            create_config_file_if_not_exists(&get_config_file_path());
            return Ok(());
        }
    }

    let config_str = fs::read_to_string(get_config_file_path())
        .expect("Could not open config file. Please run `memory config` to create it.\n");

    let config = toml::from_str(config_str.as_str()).unwrap();

    let sys_info = SysInfoWrapper::new();
    loop {
        check_memory(&sys_info, &config);
        thread::sleep(Duration::from_secs(config.loop_delay as u64));
    }
}

fn check_memory(sys_info: &SysInfoWrapper, config: &Config) {
    let percent_available_memory = get_percent_available_memory(sys_info);

    if percent_available_memory < config.alert_threshold as f64 {
        kill_applications(config);
        return;
    }

    if percent_available_memory < config.warning_threshold as f64 {
        send_warning(config);
    }
}

fn create_config_file_if_not_exists(config_file: &String) {
    match fs::metadata(config_file) {
        Ok(_) => (),
        Err(_) => {
            Command::new("cp")
                .arg("default_config.toml")
                .arg(config_file)
                .output()
                .expect("Failed to initialize configuration file");
        }
    }
}

fn kill_applications(config: &Config) {
    for application in config.applications.iter() {
        kill_application(application);
        thread::sleep(Duration::from_secs(config.refresh_memory_delay as u64));

        let sys_info = SysInfoWrapper::new();
        let percent_available_memory = get_percent_available_memory(&sys_info);
        if percent_available_memory > config.alert_threshold as f64 {
            return;
        }
    }
}

fn kill_application(application: &String) {
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

fn send_warning(config: &Config) {
    let text = format!(
        "Available memory is below {}%.\nPlease close some applications.",
        config.warning_threshold
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
