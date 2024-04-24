#![allow(warnings)]
use wasm_bindgen::prelude::*;

mod lexer;
mod parser;
mod sym_analyzer;
mod transpiler;
mod  js_transpiler;
mod  py_transpiler;
mod meta;

use std::{ffi::{c_char, CStr, CString}, fs};

use sym_analyzer::*;
use lexer::*;
use parser::*;
use transpiler::*;
use js_transpiler::*;
use py_transpiler::*;

static DEBUG_MODE: bool = true;

// python build
// work on python transpiler
// make pong work
// build a full game
// fix js restart, maybe use engine 
// fix python stuff
// handle some edje cases
// day off
// docs
// last work

fn main() {
    let src = r#"
    func on_block_init(ID) { 
    }
    func on_block_update(ID) {
        let pos = $.get_component(ID,Components.Position);  
        let size = $.get_component(ID,Components.Size);  
        
        let ball_id = $.get_entity_by_id("ball");
        let ball_pos = $.get_component(ball_id,Components.Position);
        let ball_size = $.get_component(ball_id,Components.Size);
        let ball_data = $.get_component(ball_id,Components.Storage);
    
        if(ball_data.collided == 0) {
          if($.AABB(ball_pos.x + ball_data.vel_x,ball_pos.y + ball_data.vel_y,ball_size.w,ball_size.h,pos.x,pos.y,size.w,size.h)) {
            
            if($.AABB(ball_pos.x + ball_data.vel_x,ball_pos.y,ball_size.w,ball_size.h,pos.x,pos.y,size.w,size.h)) { 
              ball_data.collided = 1;
              ball_data.x_dir =  ball_data.x_dir * -1; 
            }
            if(ball_data.collided == 0) {
              if($.AABB(ball_pos.x,ball_pos.y + ball_data.vel_y,ball_size.w,ball_size.h,pos.x,pos.y,size.w,size.h)) {
                ball_data.collided = 1;
                ball_data.y_dir =  ball_data.y_dir * -1; 
              }
            }
            
              $.remove_entity(ID);
          }
        }
    }
    
    
    func on_init(ID) {
        for(let i = 0; i < 10; i = i + 1) {
        for(let j = 0; j < 10; j = j +1) {
          let id = $.create_entity(x = 30 * i + 60 ,y = j * 30 + 10,w = 25,h = 25,r = 20,g = 120,b = 255,on_block_init,on_block_update);
      }
        }
    }
    func on_update(ID) {
    
    }    
    "#.to_string();

    println!("{}",compile(src,"".to_string()).1[0]);
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
    let src = transpiler.py_transpiler(&parser.program, 0,&mut true,&func_prefix);
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
