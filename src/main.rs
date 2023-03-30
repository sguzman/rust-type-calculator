use rust_type_calculator::process_input;
use rust_type_calculator::Environment;
use std::io::{self, Write};

fn main() {
    let mut env = Environment::new();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "quit" || input == "exit" {
            break;
        }

        match process_input(input, &mut env) {
            Ok(output) => {
                if !output.is_empty() {
                    println!("{}", output);
                }
            }
            Err(err) => println!("Error: {}", err),
        }
    }
}
