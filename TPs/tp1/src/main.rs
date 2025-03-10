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
            println!("Programme pour les Tours d'Hanoï...");
            hanoi::main();
        },
        "2" => {
            println!("Programme pour Fibonacci...");
            fibonacci::main();
        },
        "3" => {
            println!("Programme pour le Crible d'Eratosthènes...");
            sieve::main();
        }
        "help" | "--help" | "-h" => {
            print_usage();
        },
        _ => {
            println!("Option invalide: {}", args[1]);
            print_usage();
        }
    }
}

fn print_usage() {
    println!("Utilisation: cargo run <option>");
    println!("Options:");
    println!("  1     Exécuter le program des Tours d'Hanoï");
    println!("  2     Exécuter le program de Fibonacci");
    println!("  3     Exécuter le program du Crible d'Eratosthènes");
    println!("  help  Affiche de ce message");
}