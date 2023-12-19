use crate::interpreter::{lexer, token};
use std::io::{self, Write};

pub fn start() {
    loop {
        let mut input = String::new();
        io::stdout().write_all(b">> ").unwrap();
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if input.trim() == "q;" {
            io::stdout()
                .write_all(
                    b"May your trip be as enjoyable as finding \
                extra bananas at the bottom of the bag!",
                )
                .unwrap();
            break;
        }

        let mut l = lexer::Lexer::new(&input);
        loop {
            let tok = l.next_token();
            if tok.token_type == token::TokenType::EOF {
                break;
            }
            println!("{:?}", tok);
        }
    }
}
