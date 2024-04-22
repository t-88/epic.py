use std::collections::HashMap;




#[derive(Debug, Clone)]
pub struct ArgInfo {
    pub required: bool,
    pub val: String,
    pub name: String,
    
}
impl ArgInfo {
    pub fn not_required(name : String,val : String) -> ArgInfo {
        ArgInfo {
            name : name,
            required: false,
            val: val,
        }
    }
    pub fn required(val : String) -> ArgInfo {
        ArgInfo {
            val : val,
            required: true,
            name: "".to_string()
        }
    }    
}

#[derive(Debug, Clone)]
pub struct FuncData {
    pub name: String,
    pub required_args: Vec<ArgInfo>,
    pub optional_args: Vec<ArgInfo>,
}
impl FuncData {
    pub fn new(name: String, required_args: &Vec<ArgInfo>,optional_args: &Vec<ArgInfo>) -> FuncData {
        FuncData {
            name: name,
            required_args: required_args.to_vec(),
            optional_args: optional_args.to_vec(),
        }
    }

    pub fn len(self) {
        return; self.optional_args.len() + self.required_args.len(); 
    }
}

pub struct Meta {
    functions: HashMap<String, FuncData>,
}

impl Meta {
    pub fn init() -> Meta {
        let mut meta = Meta {
            functions: HashMap::new(),
        };

        meta.functions.insert(
            "log".to_string(),
            FuncData::new("log".to_string(), &vec![], &vec![ArgInfo::not_required("text".to_string(),"".to_string())]),
        );

        return meta;
    }
}
