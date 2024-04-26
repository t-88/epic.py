#![allow(warnings)]
use pybuild::build::*;
use serde_json::Value;
use wasm_bindgen::prelude::*;
use std::{
    collections::HashMap,
    ffi::{c_char, CStr, CString},
    fs::{self, File},
    io::{Read, Write},
};

mod meta;
mod  op_lang;
use op_lang::{compile::compile, lexer::Lexer, parser::Parser, sym_analyzer::SymenticAnal, transpiler::{TranspileLang, Transpiler}, *};
mod pybuild;

fn main() {
    build_python();
}
// js section
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
#[wasm_bindgen]
pub fn js_compile(src: String, func_prefix: &str) -> JsValue {
    let comped = compile(TranspileLang::Js,src, func_prefix);
    return JsValue::from(format!("{}#{:?}", comped.0, comped.1));
}
