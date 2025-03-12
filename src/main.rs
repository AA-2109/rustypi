mod network;
mod mount;
use clap::Parser;

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
    println!("Network and Mount are OK!");
}