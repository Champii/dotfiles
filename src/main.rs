mod ast;
mod lexer;
mod parser;

use lexer::Lexer;

fn main() {
    let file = r#"
macro createfn
  $name:ident, $arg1:ident =>
    $name = $arg1 -> $arg1
$createfn tata, y
"#;
    let mut lexer = Lexer::new(file).unwrap();

    let tokens = lexer.collect().unwrap();

    println!("{:#?}", tokens);

    let program = parser::parse_root(&tokens).unwrap();

    println!("{:#?}", program);
}
