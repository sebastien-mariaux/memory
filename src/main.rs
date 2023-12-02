use memory::sys_info::{get_percent_available_memory, SysInfoWrapper};

const ALERT_THRESHOLD: f64 = 5.0;

fn main() {
    let sys_info = SysInfoWrapper::new();
    println!("Total memory: {} kB", sys_info.get_total_memory());
    println!("Available memory: {} kB", sys_info.get_available_memory());

    println!(
        "Percent available memory: {:.2}%",
        get_percent_available_memory(&sys_info)
    );

    if get_percent_available_memory(&sys_info) < ALERT_THRESHOLD {
        println!("Memory is low!");
    }
}
