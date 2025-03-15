use std::process::Command;

pub(crate) fn update_pihole_database() -> bool {
    if !update_pihole_gravity_list() {
        repair_pihole()
    } else {
        true
    }
}


fn check_pihole_status() -> bool {
    let output = Command::new("pihole")
        .arg("status")
        .output()
        .expect("Failed to execute command");
    if output.status.success() {
        true
    } else {
        println!("{}", String::from_utf8_lossy(&output.stdout));
        false
    }
}



fn update_pihole_gravity_list() -> bool {
    if !check_pihole_status() {
        return false;
    }

    let output = Command::new("pihole")
        .arg("-g")
        .output()
        .expect("Failed to execute command");
    if output.status.success() {
        true
    } else {
        println!("{}", String::from_utf8_lossy(&output.stdout));
        false
    }
}


fn repair_pihole() -> bool {
    if check_pihole_status() {
        return true;
    }
    let output = Command::new("pihole")
        .arg("-r")
        .output()
        .expect("Failed to execute command");
    if output.status.success() {
        true
    } else {
        println!("{}", String::from_utf8_lossy(&output.stdout));
        false
    }
}