use super::{LexType, TokType};

#[test]
fn lex() {
    let scad = r#"// scad basic lexing file
Lex(20);

module Lex(size=20, $fn=100) {
    sphere(d=size);
}
echo(version=version());
"#;
    let scad = TokType::lex(scad).unwrap();

    assert_eq!(scad.len(), 35);
    assert_eq!(
        scad[0],
        LexType::new(TokType::IDENTIFIER("Lex".to_string()), 2, 1)
    );
    assert_eq!(scad[1], LexType::new(TokType::LParen, 2, 5));
    assert_eq!(scad[2], LexType::new(TokType::IConstant(20), 2, 6));
    assert_eq!(scad[3], LexType::new(TokType::RParen, 2, 8));
    assert_eq!(scad[4], LexType::new(TokType::Semicolon, 2, 9));

    assert_eq!(scad[5], LexType::new(TokType::Module, 4, 1));

    assert_eq!(scad[34], LexType::new(TokType::EOF, 8, 1));
}

#[test]
fn lex_fail() {
    let scad = r#"// scad basic lexing file
Lex(20);

module Lex(size=20, §fn=100) {
    sphere(d=size);
}
echo(version=version());
"#;
    let scad = TokType::lex(scad);

    assert_eq!(scad, Err("unexpected Character § at 4:21".to_string()));
}
