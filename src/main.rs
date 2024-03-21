use crate::diagnostic::Diagnostic;

mod ast;
mod diagnostic;
mod fmt;
mod lexer;
mod macro_expansion;
mod parser;

fn main() {
    let file_path = std::env::args().nth(1).unwrap();
    let ast = match parser::parse_file(file_path.into()) {
        Ok(ast) => ast,
        Err(e) => {
            Diagnostic::from(e).report();
            return;
        }
    };

    // println!("{}", ast);
    std::fs::write(std::env::args().nth(1).unwrap(), ast.to_string()).unwrap();

    let ast = match macro_expansion::expand_macros(ast) {
        Ok(ast) => ast,
        Err(e) => {
            Diagnostic::from(e).report();
            return;
        }
    };

    println!("{:#?}", ast);
    // println!("{}", ast);
}
