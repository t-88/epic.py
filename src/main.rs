#![allow(warnings)]

use parser::Parser;

mod lexer;
mod parser;

fn main() {
    let src: String = String::from("
        while(true) {
            i = i + 1;
            i = i + 2;

            a || b + 68;
            'nice!';
        }
    ",
    );

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
