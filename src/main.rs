#![allow(warnings)]

mod lexer;
mod parser;
mod sym_analyzer;

use std::fs;

use sym_analyzer::*;
use lexer::*;
use parser::*;


fn main() {
    let src: String = fs::read_to_string("./src/src.op").unwrap();


    let mut lexer: Lexer = Lexer::new();
    lexer.tokenize(&src);

    if (lexer.errs.len() > 0) {
        println!("Lexer found {} errs", lexer.errs.len());
        for lex_err in lexer.errs {
            println!("{}", lex_err.error);
        }
        return;
    }


    
    let mut parser: Parser = Parser::new();
    parser.parse(&src);
    if (parser.errs.len() > 0) {
        println!("{} syntax errs", parser.errs.len());
        for err in parser.errs {
            println!("{}", err.msg);
        }
        return;
    }
    parser.print_tree(&parser.program, 0);
    

    // let mut analyzer: SymenticAnal =  SymenticAnal::new();
    // analyzer.analyse(parser.program);
}
