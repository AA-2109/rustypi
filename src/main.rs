mod network;
mod mount;

use std::ops::Not;
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
    network::check_and_fix_network(args);
    mount::check_and_fix_mount();
}