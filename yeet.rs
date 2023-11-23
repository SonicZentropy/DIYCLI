// src/main.rs

use std::{env, path::Path};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        eprintln!("Usage: program [--dry] <PATH1> <PATH2> ...");
        return;
    }

    let dry_run = args.contains(&"--dry".to_string());
    let paths = if dry_run {
        &args[1..]
    } else {
        &args[..]
    };

    for path_str in paths {
        let path = Path::new(path_str);

        if path.is_dir() {
            // Check for dry run
            if dry_run {
                println!("Would remove directory: {}", path_str);
            } else {
                // Perform actual directory removal
                match std::fs::remove_dir_all(&path) {
                    Ok(_) => println!("Successfully removed directory: {}", path_str),
                    Err(e) => eprintln!("Failed to remove directory '{}': {}", path_str, e),
                }
            }
        } else if path.is_file() {
            // Check for dry run
            if dry_run {
                println!("Would remove file: {}", path_str);
            } else {
                // Perform actual file removal
                match std::fs::remove_file(&path) {
                    Ok(_) => println!("Successfully removed file: {}", path_str),
                    Err(e) => eprintln!("Failed to remove file '{}': {}", path_str, e),
                }
            }
        } else {
            eprintln!("No such file or directory: {}", path_str);
        }
    }
}
