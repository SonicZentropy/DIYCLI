// src/main.rs

use std::process::Command;

fn main() {
    let command = "C:\\Users\\KC\\.cargo\\bin\\kanata.exe";
    let args = "--cfg D:/Apps/kmonad/kanata.kbd";

    // Spawn the process without waiting for it to finish
    let _ = Command::new(command)
        .args(args.split_whitespace())
        .spawn()
        .expect("Failed to start command.");
}

