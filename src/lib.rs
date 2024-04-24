#![allow(warnings)]
use wasm_bindgen::prelude::*;

mod lexer;
mod parser;
mod sym_analyzer;
mod transpiler;
mod meta;

use std::{ffi::{c_char, CStr, CString}, fs};

use sym_analyzer::*;
use lexer::*;
use parser::*;
use transpiler::*;

#[wasm_bindgen]
pub fn main() { 
    compile("let a = 1;".to_string());
}


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s : &str);
}

#[wasm_bindgen]
pub fn compile(src : String) -> JsValue {
    let mut lexer: Lexer = Lexer::new();
    lexer.tokenize(&src);

    if (lexer.errs.len() > 0) {
        println!("Lexer found {} errs", lexer.errs.len());
        for lex_err in lexer.errs {
            println!("{}", lex_err.error);
        }
        return JsValue::from("lexer error");
    }


    
    let mut parser: Parser = Parser::new();
    parser.parse(&src);
    if (parser.errs.len() > 0) {
        println!("{} syntax errs", parser.errs.len());
        for err in parser.errs {
            println!("{}", err.msg);
        }
        return JsValue::from("parser error");
    }

    let mut analyzer: SymenticAnal =  SymenticAnal::new();
    analyzer.analyse(&parser.program);




    let transpiler : Transpiler = Transpiler::new();
    let src = transpiler.js_transpiler(&parser.program, 0,&mut true);
    log("ur mom");
    log(src.as_str());

    return JsValue::from(src);
}
