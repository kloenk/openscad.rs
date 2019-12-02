#[macro_use] extern crate log;

use openscad::lexer::TokType;

fn main() {
    env_logger::init();
    info!("reading main.scad");

    let file = std::fs::read_to_string("main.scad").expect("could not open file");

    let lex = TokType::lex(&file).expect("msg: &str");

    println!("LEX: {:?}", lex);
}
