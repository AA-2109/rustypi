use std::process::Command;

pub(crate) fn check_and_fix_network() -> bool {
    let ip = get_lan_gw_ip();
    if !check_lan_network(ip) {
        println!("Network is down!");
        if !restart_network() {
            false;
            std::process::exit(1);
        }
    }
    true
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


fn get_lan_gw_ip () -> String {
    let output = Command::new("sh")
        .arg("-c")
        .arg("ip r | awk '/default/ {print $3}'")
        .output().expect("Failed to execute command");
    if output.status.success() {
        String::from_utf8_lossy(&output.stdout).trim().to_string()
    } else {
        println!("{}", String::from_utf8_lossy(&output.stdout));
        String::from_utf8_lossy(&output.stderr).trim().to_string()
    }

}