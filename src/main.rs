#![allow(warnings)]

use parser::Parser;


mod lexer;
mod parser;


fn main() {
    let src: String = String::from("
        let a = 5;
    "); 

    let mut lexer : lexer::Lexer = lexer::Lexer::new(); 
    lexer.tokenize(&src);
    if(lexer.errs.len() > 0) {
        println!("Lexer found {} errs",lexer.errs.len());
        for lex_err in lexer.errs  {
            println!("{}",lex_err.error);
        }
        return;
    }
    // for tkn in lexer.tknz  {
        // println!("{:?}",tkn);
    // }    

    let mut parser: Parser = parser::Parser::new();
    parser.parse(&src);
    for stmt in parser.program  {
        println!("{:?}",stmt);
    }    

}
