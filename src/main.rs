use clap::Parser;
use std::process::Command;

#[derive(Parser)]
#[command(name = "LAN_check")]
#[command(about = "Checks LAN connectivity", version = "1.0", author = "Artem Azamatov <azamatovartem@gmail.com>")]
struct Args {
    ip: Option<String>,
}

fn main() {
    let args = Args::parse();

    match args.ip {
        Some(ip) if !ip.is_empty() => {
            println!("IP address: {}", ip);
            if check_lan_network(ip) {
                println!("Network is up!");
                std::process::exit(0);
            } else {
                println!("Network is down!");
                if restart_network() {
                    std::process::exit(0);
                } else {
                    std::process::exit(1);
                }
            }
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