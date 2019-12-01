mod lexer;

use lexer::TokType;

fn main() {
    println!("reading main.scad");

    let file = std::fs::read_to_string("main.scad").expect("could not open file");

    let lex = TokType::lex(&file).expect("msg: &str");

    println!("LEX: {:?}", lex);
}
