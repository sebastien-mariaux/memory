use dialog::DialogBox;
use memory::sys_info::{get_percent_available_memory, SysInfoWrapper};

const WARNING_THRESHOLD: f64 = 10.0;
const ALERT_THRESHOLD: f64 = 5.0;

fn main() {
    let sys_info = SysInfoWrapper::new();
    let percent_available_memory = get_percent_available_memory(&sys_info);
    println!("Total memory: {} kB", sys_info.get_total_memory());
    println!("Available memory: {} kB", sys_info.get_available_memory());

    println!("Percent available memory: {:.2}%", percent_available_memory);

    if percent_available_memory < ALERT_THRESHOLD {
        let text = format!(
            "Available memory is below {}%.\nPlease close some applications.",
            ALERT_THRESHOLD
        );
        dialog::Message::new(text)
            .title("ALERT - VERY LOW MEMORY")
            .show()
            .expect("Could not display dialog box");
        return;
    }

    if percent_available_memory < WARNING_THRESHOLD {
        let text = format!(
            "Available memory is below {}%.\nPlease close some applications.",
            WARNING_THRESHOLD
        );
        dialog::Message::new(text)
            .title("WARNING - LOW MEMORY")
            .show()
            .expect("Could not display dialog box");
        return;
    }
}
