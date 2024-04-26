use std::{
    fs::File,
    io::{Read, Write},
    process::Command,
};

use serde_json::Value;

use crate::{
    compile::compile,
    transpiler::{TranspileLang, Transpiler},
};

fn py_transpile_entity(
    comps: &serde_json::Map<std::string::String, Value>,
    name: &str,
    func_prefix: &str,
) -> (String, String) {
    let spacing = "\t";
    let mut src = "".to_string();
    let mut funcs = "".to_string();

    let mut x = "0".to_string();
    let mut y = "0".to_string();
    let mut w = "0".to_string();
    let mut h = "0".to_string();
    let mut r = "0".to_string();
    let mut g = "0".to_string();
    let mut b = "0".to_string();
    let mut id = "None".to_string();
    let mut store = "[]".to_string();

    if comps.contains_key("pos") {
        x = comps["pos"]["x"].as_f64().unwrap().to_string();
        y = comps["pos"]["y"].as_f64().unwrap().to_string();
    }
    if comps.contains_key("size") {
        w = comps["size"]["w"].as_f64().unwrap().to_string();
        h = comps["size"]["h"].as_f64().unwrap().to_string();
    }
    if comps.contains_key("color") {
        r = comps["color"]["r"].as_f64().unwrap().to_string();
        g = comps["color"]["g"].as_f64().unwrap().to_string();
        b = comps["color"]["b"].as_f64().unwrap().to_string();
    }
    if comps.contains_key("id") {
        id = comps["id"].as_str().unwrap().to_string();
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
        store = storage_src;
    }
    
    let mut  script = "";
    let mut transpiled_script = "None".to_string();
    let mut on_init = "None".to_string();
    let mut on_update = "None".to_string();

    if comps.contains_key("script") {
        script = comps["script"].as_str().unwrap();
        let (status, content) = compile(
            TranspileLang::Py,
            script.to_string(),
            format!("{func_prefix}").as_str(),
        );

        if (status != 0) {
            println!("transpiler expects the code to be transpilable, its not!");
            assert!(status == 0);
        }
        transpiled_script = content[0].clone();
        funcs += transpiled_script.as_str();

        on_init = format!("{func_prefix}on_init");
        on_update = format!("{func_prefix}on_update"); 

    }


    src = format!(
        "{}sys__create_entity({on_init},
                              {on_update}, 
                              x = {x}, 
                              y = {y}, 
                              w = {w}, 
                              h = {h},
                              r = {r},
                              g = {g},
                              b = {b},
                              id='{id}',
                              storage = {store})\n",spacing.repeat(1));
    return (src, funcs);
}

pub fn build_python() {
    let mut file = File::open("src.json").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    let json: Value = serde_json::from_str(&data).unwrap();

    let spacing = "\t";

    // init entities
    let mut init_entities_src = "".to_string();
    let mut functions = "".to_string();

    let (src, funcs) = py_transpile_entity(json["comps"].as_object().unwrap(), "scene", "");
    init_entities_src += src.as_str();
    functions += funcs.as_str();


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

    // app config
    let scene_comps = json["comps"].as_object().unwrap();



    let app_configs = format!(
        r#"
engine.width = {}
engine.height = {}
engine.background_color = ({},{},{})
"#,
        scene_comps["size"]["w"].as_f64().unwrap(),
        scene_comps["size"]["h"].as_f64().unwrap(),
        scene_comps["color"]["r"].as_f64().unwrap(),
        scene_comps["color"]["g"].as_f64().unwrap(),
        scene_comps["color"]["b"].as_f64().unwrap()
    );

    // combine code
    let src = format!(
        r#"
from engine.engine import engine
from engine.meta import *        
import random
import math

engine.entities = {{}}

    
# generated funcs
{functions}
    
    
# generated code
def init():
{init_entities_src}

    
{app_configs}

engine.width = 400
engine.height = 600
engine.pre_init = init
engine.init()
engine.run()

"#
    );

    let mut f = File::create("./py-env/game.py").unwrap();
    f.write(src.as_bytes());

    println!("python script been generated at py-env/game.py");
    println!("building to standalone executable...");
    let cmd = Command::new("bash")
        .arg("build_python.sh")
        .output()
        .expect("failed to build python");
    assert!(cmd.status.success());
    println!("done! run ./dist/game");
}
