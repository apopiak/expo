extern crate expo;
extern crate nom;

use std::io;
use std::io::Write;

use expo::parser;
use expo::ast::Eval;

fn main() {
    // Print Version and Exit Information
    println!("Expo Version 0.0.1");
    println!("Press Ctrl+c to Exit");
    let stdin = io::stdin();
    let mut input = String::new();
    loop {
        input.clear();
        print!("expo> ");
        io::stdout().flush().unwrap();
        match stdin.read_line(&mut input) {
            Ok(_) => {
                let result = parser::parse(&mut input);
                if let nom::IResult::Done(_, output) = result {
                    println!("{:?}", output.eval());
                }
                else {
                    println!("error while parsing: {:?}", result);
                }
            }
            Err(error) => println!("error {}", error),
        }
    }
}
