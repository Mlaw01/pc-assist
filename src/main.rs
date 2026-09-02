use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let arguments: Vec<String> = env::args().collect();
    println!("Arguments received: {arguments:?}");

    if arguments.len() < 2 {
        print_help();
        return;
    }

    let command = arguments[1].as_str();

    match command {
        "disks" => println!("Disks command selected"),
        "scan" => scan_command(&arguments[2..]),
        "largest" => println!("Largest command selected"),
        _ => {
            eprintln!("Unknown command: {command}");
            print_help();
        }
    }
}

fn scan_command(arguments: &[String]) {
    if arguments.is_empty() {
        eprintln!("Missing directory path");
        eprintln!("Usage: pc-assistant scan <path>");
    }

    let path = Path::new(&arguments[0]);

    if !path.exists() {
        eprintln!("Path does not exist: {}", path.display());
        return;
    }

    if !path.is_dir() {
        eprintln!("Path is not a directory: {}", path.display());
        return;
    }

    // =========================================================================
    // DIRECTORY SCANNER & METADATA AGGREGATOR
    // 1. Opens the target path safely (returns early on critical failure).
    // 2. Iterates over entries one by one, filtering out OS read errors.
    // 3. Inspects file type metadata to categorize into files, folders, or other.
    // =========================================================================

    let entries = match fs::read_dir(path) {
        Ok(entries) => entries,
        Err(error) => {
            eprintln!("Could not read {}: {error}", path.display());
            return;
        }
    };

    let mut file_count = 0;
    let mut directory_count: u64 = 0;
    let mut other_count: u64 = 0;
    let mut skipped_count: u64 = 0;

    for entry_result in entries {
        let entry = match entry_result {
            Ok(entry) => entry,
            Err(error) => {
                eprintln!("Could not read an entry: {error}");
                skipped_count += 1;
                continue;
            }
        };

        let file_type = match entry.file_type() {
            Ok(file_type) => file_type,
            Err(error) => {
                eprintln!(
                    "Could not determine the type of {}: {error}",
                    entry.path().display()
                );
                skipped_count += 1;
                continue;
            }
        };

        if file_type.is_file() {
            file_count += 1;
        } else if file_type.is_dir() {
            directory_count += 1;
        } else {
            other_count += 1;
        }
    }

    println!("Directory: {}", path.display());
    println!("Files: {file_count}");
    println!("Directories: {directory_count}");
    println!("Other: {other_count}");
    println!("Skipped: {skipped_count}");
}

fn print_help() {
    println!("PC Assistant");
    println!();
    println!("Available commands:");
    println!("   disks");
    println!("   scan <path>");
    println!("   largest <path>");
}
