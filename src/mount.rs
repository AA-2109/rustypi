use std::process::Command;

pub(crate) fn check_and_fix_mount() -> bool {
    if !check_for_share_mount() {
        if !remount_share() {
            return false;
        }
    }
    true
}

fn check_for_share_mount() -> bool {
    let output = Command::new("test")
        .arg("-d")
        .arg("/mnt/share")
        .output()
        .expect("Failed to execute command");
    if output.status.success() {
        true
    } else {
        println!("{}", String::from_utf8_lossy(&output.stdout));
        false
    }
}

fn remount_share() -> bool {
    let output = Command::new("mount")
        .arg("-a")
        .output()
        .expect("Failed to execute command");
    if output.status.success() {
        true
    } else {
        println!("{}", String::from_utf8_lossy(&output.stdout));
        false
    }
}