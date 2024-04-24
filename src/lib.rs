#![allow(warnings)]
use serde_json::Value;
use wasm_bindgen::prelude::*;

mod js_transpiler;
mod lexer;
mod meta;
mod parser;
mod py_transpiler;
mod sym_analyzer;
mod transpiler;

use std::{
    collections::HashMap,
    ffi::{c_char, CStr, CString},
    fs::{self, File},
    io::{Read, Write},
};

use js_transpiler::*;
use lexer::*;
use parser::*;
use py_transpiler::*;
use sym_analyzer::*;
use transpiler::*;

static DEBUG_MODE: bool = true;

// [x] python build
// [x] make pong work
// [x] build a full game
// [] fix js restart, maybe use engine
// [] fix python stuff
// [] handle some edje cases
// [] day off
// [] docs
// [] last work

fn main() {
    build_python();
}

fn py_transpile_entity(
    comps: &serde_json::Map<std::string::String, Value>,
    name: &str,
    func_prefix: &str,
) -> (String, String) {
    let spacing = "\t";
    let mut src = "".to_string();
    let mut funcs = "".to_string();

    src += &format!("{}{name} = esper.create_entity()\n", spacing.repeat(1));
    src += &format!(
        "{}esper.add_component({name},ecs_component.RectShape())\n",
        spacing.repeat(1),
    );
    if comps.contains_key("pos") {
        src += &format!(
            "{}esper.add_component({name},ecs_component.Position({},{}))\n",
            spacing.repeat(1),
            comps["pos"]["x"].as_f64().unwrap(),
            comps["pos"]["y"].as_f64().unwrap()
        );
    }
    if comps.contains_key("size") {
        src += &format!(
            "{}esper.add_component({name},ecs_component.Size({},{}))\n",
            spacing.repeat(1),
            comps["size"]["w"].as_f64().unwrap(),
            comps["size"]["h"].as_f64().unwrap()
        );
    }
    if comps.contains_key("color") {
        src += &format!(
            "{}esper.add_component({name},ecs_component.Color({},{},{}))\n",
            spacing.repeat(1),
            comps["color"]["r"].as_f64().unwrap(),
            comps["color"]["g"].as_f64().unwrap(),
            comps["color"]["b"].as_f64().unwrap(),
        );
    }
    if comps.contains_key("id") {
        src += &format!(
            "{}esper.add_component({name},ecs_component.IdComponent({:?}))\n",
            spacing.repeat(1),
            comps["id"].as_str().unwrap(),
        );
    }
    if comps.contains_key("storage") {
        let mut storage_src = "[".to_string();
        let storage = comps["storage"].as_array().unwrap();
        for i in 0..storage.len() {
            storage_src += &format!(
                "{{ 'key' : '{}' , 'val' : '{}'  }},",
                storage[i]["key"].as_str().unwrap(),
                storage[i]["val"].as_str().unwrap()
            );
        }
        storage_src += "]";

        src += &format!(
            "{}esper.add_component({name},ecs_component.Storage({}))\n",
            spacing.repeat(1),
            storage_src
        );
    }

    let script = comps["script"].as_str().unwrap();
    let (status, content) = compile(script.to_string(), format!("{func_prefix}").as_str());

    if (status != 0) {
        println!("transpiler expects the code to be transpilable, its not!");
        assert!(status == 0);
    }

    let transpiled_script = content[0].clone();
    funcs += transpiled_script.as_str();

    src += &format!(
        "{}esper.add_component({name},UpdateCallback(callback={func_prefix}on_update))\n",
        spacing.repeat(1)
    );
    src += &format!(
        "{}esper.add_component({name},InitCallback(callback={func_prefix}on_init))\n",
        spacing.repeat(1)
    );

    return (src, funcs);
}
pub fn build_python() {
    let mut file = File::open("src/src.json").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let json: Value = serde_json::from_str(&data).unwrap();

    let spacing = "\t";

    // init entities
    let mut init_entities_src = "".to_string();
    let mut functions = "".to_string();

    let children = json["children"].as_array().unwrap();
    let mut idx = 0;
    for i in 0..children.len() {
        let (src, funcs) = py_transpile_entity(
            children[i]["comps"].as_object().unwrap(),
            format!("rect{i}").as_str(),
            format!("_{i}_").as_str(),
        );
        init_entities_src += src.as_str();
        functions += funcs.as_str();
    }
    let (src, funcs) = py_transpile_entity(json["comps"].as_object().unwrap(), "scene", "");
    init_entities_src += src.as_str();
    functions += funcs.as_str();

    // app config
    let scene_comps = json["comps"].as_object().unwrap();

    let app_configs = format!(
        r#"
engine.width = {}
engine.height = {}
engine.background_color = ({},{},{})
"#,
        scene_comps["size"]["w"].as_f64() .unwrap(),
        scene_comps["size"]["h"].as_f64() .unwrap(),
        scene_comps["color"]["r"].as_f64().unwrap(),
        scene_comps["color"]["g"].as_f64().unwrap(),
        scene_comps["color"]["b"].as_f64().unwrap(),

    );


    // combine code
    let src = format!(r#"
from engine import *
import pygame
import ecs.ecs_component as  ecs_component
import ecs.ecs_system as  ecs_system
import esper
from op_lang.build.lookup_tables import *
from op_lang.build.functions import *
import random
import math

entities = {{}}
    
    
# generated funcs
{functions}
    
    
# generated code
def init():
{spacing}esper.add_processor(ecs_system.RectRendererSystem())  
{spacing}esper.add_processor(ecs_system.InitCallbackSystem())    
{spacing}esper.add_processor(ecs_system.UpdateCallbackSystem())    
{init_entities_src}


def process():
    esper.process()
    
{app_configs}
engine.init_callback = init 
engine.process_callback = process 
engine.init()


engine.run()
"#);

    let mut f = File::create("./pygame_engine/game_code.py").unwrap();
    f.write(src.as_bytes());
}

pub fn compile(src: String, func_prefix: &str) -> (i32, Vec<String>) {
    let mut lexer: Lexer = Lexer::new();
    lexer.tokenize(&src);
    if (lexer.errs.len() > 0) {
        return (1, lexer.errs);
    }

    let mut parser: Parser = Parser::new();
    parser.parse(&src);
    if (parser.errs.len() > 0) {
        return (2, parser.errs);
    }

    let mut analyzer: SymenticAnal = SymenticAnal::new();
    analyzer.analyse(&parser.program);
    if (analyzer.errs.len() > 0) {
        return (3, analyzer.errs);
    }

    let transpiler: Transpiler = Transpiler::new();
    let src = transpiler.py_transpiler(&parser.program, 0, &mut true, &func_prefix.to_string());
    return (0, vec![src]);
}

// js section
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
#[wasm_bindgen]
pub fn js_compile(src: String, func_prefix: &str) -> JsValue {
    let comped = compile(src, func_prefix);
    return JsValue::from(format!("{}#{:?}", comped.0, comped.1));
}
