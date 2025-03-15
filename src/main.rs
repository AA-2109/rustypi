mod network;
mod mount;
mod pihole;

use clap::Parser;
use crate::pihole::update_pihole_database;

#[derive(Parser)]
#[command(name = "LAN_check")]
#[command(about = "Checks LAN connectivity", version = "1.0", author = "Artem Azamatov <azamatovartem@gmail.com>")]
struct Args {
    ip: Option<String>,
}

fn main() {
    let args = Args::parse();
    if  !network::check_and_fix_network(args) {
        print!("Network is NOK")
    }
    if !mount::check_and_fix_mount() {
        print!("Mount is NOK")
    }

    if !update_pihole_database() {
        print!("Pihole database update is NOK")
    }
    println!("Network and Mount are OK!");
}