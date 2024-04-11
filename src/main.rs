#![allow(warnings)]

use parser::Parser;

mod lexer;
mod parser;

fn main() {
    let src: String = String::from("
    a = 5 + 5;
    let b = 5 * 5 + 2 / 3 - 1;
",);

    let mut lexer: lexer::Lexer = lexer::Lexer::new();
    lexer.tokenize(&src);
    if (lexer.errs.len() > 0) {
        println!("Lexer found {} errs", lexer.errs.len());
        for lex_err in lexer.errs {
            println!("{}", lex_err.error);
        }
        return;
    }

    let mut parser: Parser = parser::Parser::new();
    parser.parse(&src);
    parser.print_tree(&parser.program, 0);
}
