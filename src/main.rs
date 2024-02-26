mod ast;
mod lexer;
mod parser;
mod span;
mod token;

use lexer::Lexer;
use parser::Parsable;

fn main() {
    let file = r#"
main = x -> x + 1
toto = -> 2
"#;
    let mut lexer = Lexer::new(file).unwrap();

    let tokens = lexer.collect().unwrap();

    println!("{:#?}", tokens);

    let program = ast::Program::parse(&tokens).unwrap().0;

    println!("{:#?}", program);
}
