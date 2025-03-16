mod network;
mod mount;
mod pihole;

use clap::Parser;
use crate::pihole::update_pihole_database;

#[derive(Parser)]
#[command(name = "rustypi")]
#[command(about = "Checks network connectivity, mounts and updates pi-hole gravity list", version = "1.1", author = "Artem Azamatov <azamatovartem@gmail.com>")]
struct Args {
    path_to_share: String,
}

fn main() {
    let args = Args::parse();
    if  !network::check_and_fix_network() {
        print!("Network is NOK")
    }

    if !mount::check_and_fix_mount(args.path_to_share) {
        print!("Mount is NOK")
    }

    if !update_pihole_database() {
        print!("Pihole database update is NOK")
    }
    println!("Network and Mount PiHole Database are OK!");
}