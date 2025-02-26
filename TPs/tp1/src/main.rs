mod hanoi;
mod fibonacci;
mod sieve;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2{
        print_usage();
        return;
    }

    match args[1].as_str() {
        "1" => {
            println!("Running Hanoi Tower program...");
            hanoi::main();
        },
        "2" => {
            println!("Running Recursive Fibonacci program...");
            fibonacci::main();
        },
        "3" => {
            println!("Running Sieve of Eratosthenes program...");
            sieve::main();
        }
        "help" | "--help" | "-h" => {
            print_usage();
        },
        _ => {
            println!("Invalid option: {}", args[1]);
            print_usage();
        }
    }
}

fn print_usage() {
    println!("Usage: cargo run <option>");
    println!("Options:");
    println!("  1     Run Hanoi Tower program");
    println!("  2     Run Recursive Fibonacci program");
    println!("  3     Run Sieve of Eratosthenes program");
    println!("  help  Display this help message");
}