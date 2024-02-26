mod ast;
mod lexer;
mod parser;
mod span;
mod token;

use lexer::Lexer;
use parser::Parsable;

fn main() {
    let mut lexer = Lexer::new("main = x -> x + 1").unwrap();
    let tokens = lexer.collect().unwrap();

    println!("{:#?}", tokens);

    let program = ast::Program::parse(&tokens).unwrap().0;

    println!("{:#?}", program);
}
