mod network;
mod mount;
use std::ops::Not;
use clap::Parser;

#[derive(Parser)]
#[command(name = "LAN_check")]
#[command(about = "Checks LAN connectivity", version = "1.0", author = "Artem Azamatov <azamatovartem@gmail.com>")]
struct Args {
    ip: Option<String>,
}

fn main() {
    let args = Args::parse();
    if !network::check_and_fix_network(args) {
        print!("Network not ok")
    }
    if !mount::check_and_fix_mount() {
        print!("Mount not ok")
    }
    println!("Network and Mount are OK!");
}