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

fn main() {
    println!("{:?}",compile("      let id = $.create_entity(x = i * 30 + 60,y = j * 30 + 10,w = 25,h = 25,r = 20,g = 120,b = 255,1,2);".to_string(),"".to_string()));
}

pub fn compile(src : String, func_prefix : String) -> (i32,Vec<String>) {
    let mut lexer: Lexer = Lexer::new();
    lexer.tokenize(&src);
    if (lexer.errs.len() > 0) {
        return (1 , lexer.errs);
    }

    let mut parser: Parser = Parser::new();
    parser.parse(&src);
    if (parser.errs.len() > 0) {
        return (2 , parser.errs);
    }

    let mut analyzer: SymenticAnal =  SymenticAnal::new();
    analyzer.analyse(&parser.program);
    if (analyzer.errs.len() > 0) {
        return (3 , analyzer.errs);
    }

    let transpiler : Transpiler = Transpiler::new();
    let src = transpiler.js_transpiler(&parser.program, 0,&mut true,&func_prefix);
    return (0,vec![src]);
}


// js section

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s : &str);
}
#[wasm_bindgen]
pub fn js_compile(src : String,func_prefix : String) -> JsValue {
    let comped = compile(src,func_prefix);
    return JsValue::from(format!("{}#{:?}",comped.0,comped.1));
}
