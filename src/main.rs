mod ast;
mod lexer;
mod parser;

fn main() {
    let file_path = std::env::args().nth(1).unwrap();
    let ast = parser::parse_file(file_path.into()).unwrap();

    println!("{:#?}", ast);
}
