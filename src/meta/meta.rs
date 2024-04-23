use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ArgInfo {
    pub required: bool,
    pub val: String,
    pub name: String,
}
impl ArgInfo {
    pub fn not_required(name: String, val: String) -> ArgInfo {
        ArgInfo {
            name: name,
            required: false,
            val: val,
        }
    }
    pub fn required(val: String) -> ArgInfo {
        ArgInfo {
            val: val,
            required: true,
            name: "".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FuncData {
    pub name: String,
    pub required_args: usize,
    pub optional_args: Vec<ArgInfo>,
}
impl FuncData {
    pub fn new(name: String, required_args: usize, optional_args: &Vec<ArgInfo>) -> FuncData {
        FuncData {
            name: name,
            required_args: required_args,
            optional_args: optional_args.to_vec(),
        }
    }

    pub fn len(self) {
        return;
        self.optional_args.len() + self.required_args;
    }
}

#[derive(Debug)]
pub struct Meta {
    pub functions: HashMap<String, FuncData>,
}
impl Meta {
    pub fn add_func(&mut self, func_data: FuncData) {
        self.functions
            .insert(func_data.name.clone(), func_data.clone());
    }
    pub fn init() -> Meta {
        let mut meta = Meta {
            functions: HashMap::new(),
        };

        meta.add_func(FuncData::new("log".to_string(), 1, &vec![]));

        meta.add_func(FuncData::new(
            "get_component".to_string(),
            2, 
            &vec![],
        ));

        meta.add_func(FuncData::new(
            "get_entity_by_id".to_string(),
            1, 
            &vec![],
        ));

        meta.add_func(FuncData::new(
            "randint".to_string(),
            2, 
            &vec![],
        ));

        meta.add_func(FuncData::new(
            "clear_entities".to_string(),
            0, 
            &vec![],
        ));  

        meta.add_func(FuncData::new(
            "init".to_string(),
            0, 
            &vec![],
        ));

        meta.add_func(FuncData::new(
            "sqrt".to_string(),
            1, 
            &vec![],
        ));                 

        meta.add_func(FuncData::new(
            "AABB".to_string(),
            8, 
            &vec![],
        ));             

        return meta;
    }
}
