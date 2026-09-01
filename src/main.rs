use std::env;

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
        "scan" => println!("Scan command selected"),
        "largest" => println!("Largest command selected"),
        _ => {
            eprintln!("Unknown command: {command}");
            print_help();
        }
    }
}

fn print_help() {
    println!("PC Assistant");
    println!();
    println!("Available commands:");
    println!("   disks");
    println!("   scan <path>");
    println!("   largest <path>");
}
