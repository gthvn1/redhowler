mod lexer;
mod token;

fn main() {
    println!("Hello, Monkey Islang!");
    let mut s = String::new();
    const C: char = '=';
    const B: char = '=';
    s.push(C);
    s.push(B);
    println!("{}", s)
}
