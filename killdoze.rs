use std::env;
use std::process::Command;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Check if a process name is provided
    if args.len() != 2 {
        eprintln!("Usage: program <PROCESS_NAME>");
        process::exit(1);
    }

    let process_name = &args[1];

    // Construct the taskkill command
    let status = Command::new("taskkill")
        .arg("/IM")
        .arg(process_name)
        .arg("/F")
        .status()
        .expect("Failed to execute command.");

    if !status.success() {
        eprintln!("Failed to kill process: {}", process_name);
        process::exit(1);
    }

    println!("Successfully killed process: {}", process_name);
}