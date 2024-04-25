use std::{collections::HashMap, vec};


pub enum TranspileLang {
    Js,
    Py
}

pub struct Transpiler {
    pub js_functions: HashMap<String, String>,
    pub py_functions: HashMap<String, String>,
}

impl Transpiler {
    pub fn new() -> Transpiler {
        let mut trans: Transpiler = Transpiler {
            js_functions: HashMap::new(),
            py_functions: HashMap::new(),
        };
        trans.fill_js_prebuilds();
        trans.fill_py_prebuilds();
        
        return trans;
    }
    pub fn same_name_meta(&self,functions : &mut HashMap<String,String>,name : &str) {
        functions.insert(name.to_string(), format!("sys__{name}"));
    }

}
