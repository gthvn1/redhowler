extern crate redhowler;

use redhowler::repl;

fn main() {
    println!("Welcome to Monkey Islang!!!");
    println!("This is the REPL for Monkey programming language.");
    println!("Feel free to type commands or 'q;'");

    repl::start();
}
