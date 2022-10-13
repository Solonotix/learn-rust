use std::env;

use animals::main as animals;
use loops::main as loops;
use travel::main as travel;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    match args.split_first() {
        Some((command, params)) => match command.to_lowercase().as_str() {
            "animals" => animals(params),
            "loops" => loops(params),
            "travel" => travel(),
            _ => println!("Unsupported command: {}", command)
        }
        _ => println!("No args provided.")
    }
}
