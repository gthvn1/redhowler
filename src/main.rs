extern crate redhowler;

use redhowler::repl;

fn main() {
    println!("Welcome to Monkey Islang!!!");
    println!("This is the REPL for Monkey programming language.");
    println!("Feel free to type some code or 'q;'");

    repl::start();
}
