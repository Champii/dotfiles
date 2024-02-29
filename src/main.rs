mod ast;
mod lexer;
mod macro_expansion;
mod parser;

fn main() {
    let file_path = std::env::args().nth(1).unwrap();
    let ast = parser::parse_file(file_path.into()).unwrap();

    let ast = macro_expansion::expand_macros(ast);
    println!("{:#?}", ast);
}
