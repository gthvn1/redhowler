use crate::interpreter::lexer::Lexer;
use std::io::{self, BufRead, Write};

pub fn start() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut handle = stdin.lock();

    println!("Welcome to Monkey Islang!!!");
    println!("This is the REPL for Monkey programming language.");
    println!("Feel free to type some code. Ctrl+D to quit");

    loop {
        let mut input = String::new();

        // Print the prompt
        print!(">> ");
        stdout.flush().expect("Failed to flush stdout");

        match handle.read_line(&mut input) {
            Ok(0) => {
                // 0 bytes means EOF (Ctrl+D)
                println!("detected Ctrl+D, exited.");
                println!("May your trip be as enjoyable as finding extra bananas at the bottom of the bag!");
                break;
            }
            Ok(_) => {
                let lex = Lexer::from_str(&input);
                for tok in lex {
                    println!("{:?}", tok);
                }
            }
            Err(e) => {
                eprintln!("Error reading line: {}", e);
                println!("Same player shoot again");
            }
        }
    }
}
