mod lexer;
mod repl;
mod token;

use std::env;
use std::io;

fn main() {
    let username = env::var("USER").unwrap_or_else(|_| "anonymous".into());
    println!(
        "Hello {}! This is the Moneky programming language!",
        username
    );
    println!("Feel free to type in commands");
    repl::start(io::stdin().lock(), io::stdout());
}
