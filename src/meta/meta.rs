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
    pub fn add_func(&mut self, name: &str, num_required: usize, optional_info: Vec<ArgInfo>) {
        self.functions.insert(
            name.to_string(),
            FuncData::new(name.to_string(), num_required, &optional_info),
        );
    }
    pub fn init() -> Meta {
        let mut meta = Meta {
            functions: HashMap::new(),
        };

        meta.add_func("log", 1, vec![]);
        meta.add_func("get_component", 2, vec![]);
        meta.add_func("get_entity_by_id", 1, vec![]);
        meta.add_func("randint", 2, vec![]);
        meta.add_func("clear_entities", 0, vec![]);
        meta.add_func("init", 0, vec![]);
        meta.add_func("sqrt", 1, vec![]);
        meta.add_func("AABB", 8, vec![]);
        meta.add_func("is_pressed", 1, vec![]);
        meta.add_func("remove_entity", 1, vec![]);
        meta.add_func(
            "create_entity",
            2,
            vec![
                ArgInfo::not_required("x".to_string(), "0".to_string()),
                ArgInfo::not_required("y".to_string(), "0".to_string()),
                ArgInfo::not_required("w".to_string(), "0".to_string()),
                ArgInfo::not_required("h".to_string(), "0".to_string()),
                ArgInfo::not_required("r".to_string(), "0".to_string()),
                ArgInfo::not_required("g".to_string(), "0".to_string()),
                ArgInfo::not_required("b".to_string(), "0".to_string()),
            ],
        );

        return meta;
    }
}
