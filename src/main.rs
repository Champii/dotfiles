use crate::diagnostic::Diagnostic;

mod ast;
mod diagnostic;
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

    let ast = macro_expansion::expand_macros(ast);
    println!("{:#?}", ast);
}
