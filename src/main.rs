#![allow(warnings)]

mod lexer;
mod parser;
mod sym_analyzer;

use sym_analyzer::*;
use lexer::*;
use parser::*;


fn main() {
    let src: String = String::from("
        let a = 2;
        let b = 1 + a;
        // let n = n;


        // ifs , else ifs , elses
        if( true ) {
            let c = 0;
            c + a;
        } elseif(false) {
            // k;
            a;
        } else {
            b;
        }

        // for loops
        for(let i = 0; i < 5; i = i + 1) {
            i + 1;   
            a;
            b;
            let c = 1 * 2;
        }

        // while loops
        while(false) {
            let k = 0;
            let a = 2 + k;
        }

        // func declaration
        func hello(a = 15, b , c = 'ur mom', d = 6) {
            let a = d + c;
            let l = 'hello';
            return a + l;
        }

        hello(a,b,1, 2 * 4);

        // arrays
        [a,b, 123];
     

        // hash map
        {
            a: 1,
            b: a,
            c: b,
            d: a + b
        };
    ");

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
    // parser.print_tree(&parser.program, 0);



    let mut analyzer: SymenticAnal =  SymenticAnal::new();
    analyzer.analyse(parser.program);
}
