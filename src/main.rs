mod error;
mod expression;
mod lox;
mod scanner;
mod token;
mod token_type;
use std::env;

use lox::Lox;
fn main() {
    let args: Vec<String> = env::args().collect();
    let mut lox = Lox::new();
    lox.start(args);
}
