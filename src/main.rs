mod ast;
mod lexer;
mod parser;
mod repl;
mod token;

fn main() {
    println!("Welcome to Monkey Islang!!!");
    println!("This is the REPL for Monkey programming language.");
    println!("Feel free to type commands or 'q;'");

    repl::start();
}
