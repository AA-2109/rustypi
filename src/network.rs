use std::process::Command;
use crate::Args;

pub(crate) fn check_and_fix_network(args: Args) -> bool {

    match args.ip {
        Some(ip) if !ip.is_empty() => {
            if !check_lan_network(ip) {
                println!("Network is down!");
                if !restart_network() {
                    false;
                    std::process::exit(1);
                }
            }
            true
        }
        _ => {
            eprintln!("Error: IP address is required and cannot be empty.");
            std::process::exit(1);
        }
    }
}


fn check_lan_network(ip: String) -> bool {
    let output = Command::new("ping")
        .arg("-c")
        .arg("1")
        .arg(ip)
        .output()
        .expect("Failed to execute command");

    output.status.success()
}

fn restart_network() -> bool {
    let output = Command::new("systemctl")
        .arg("restart")
        .arg("networking")
        .output()
        .expect("Failed to restart networking");

    if output.status.success() {
        true
    } else {
        println!("{}", String::from_utf8_lossy(&output.stdout));
        false
    }
}