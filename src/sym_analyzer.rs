use crate::*;
use meta::meta::*;
use std::{collections::HashMap, fmt::format, vec};

//TODO: Handle prebuild function, idents
//TODO: Handle arg count

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum SymbType {
    Null,
    String,
    Number,
    HashMap,
    Array,
}

#[derive(Debug, Clone)]
struct SymbData {
    pub name: String,
    pub typ: SymbType,
    pub attrs: HashMap<String, SymbData>,
    pub is_func: bool,
    pub func_data: FuncData,
}

impl SymbData {
    fn new() -> Self {
        return SymbData {
            name: "".to_string(),
            is_func: false,
            typ: SymbType::Null,
            attrs: HashMap::new(),
            func_data: FuncData::new("".to_string(), &vec![], &vec![]),
        };
    }

    fn variable(name: String, typ: SymbType) -> SymbData {
        let mut data = SymbData::new();
        data.name = name;
        data.typ = typ;
        data
    }

    fn func(name: String, func_data: FuncData) -> SymbData {
        let mut data = SymbData::new();
        data.name = name;
        data.is_func = true;
        data.func_data = func_data;
        data
    }
}

#[derive(Debug)]
pub struct SymbTable {
    symbs: HashMap<String, SymbData>,
}
impl SymbTable {
    fn new() -> Self {
        SymbTable {
            symbs: HashMap::new(),
        }
    }
}

#[derive(Debug)]
pub struct ScopeStack {
    cur_scope: i64,
    scopes: Vec<SymbTable>,
}

impl ScopeStack {
    fn new() -> Self {
        ScopeStack {
            cur_scope: -1,
            scopes: vec![],
        }
    }

    fn push_scope(self: &mut Self) {
        self.cur_scope += 1;
        self.scopes.push(SymbTable::new());
    }

    fn pop_scope(self: &mut Self) {
        self.cur_scope -= 1;
        self.scopes.pop();
    }
    fn check_symb(self: &mut Self, name: &str) -> bool {
        let saved_scope = self.cur_scope;
        let mut found = false;
        loop {
            if (self.scopes[self.cur_scope as usize]
                .symbs
                .contains_key(name))
            {
                found = true;
                break;
            }
            self.cur_scope -= 1;
            if (self.cur_scope < 0) {
                break;
            }
        }
        self.cur_scope = saved_scope;
        return found;
    }
    fn get_symb(self: &mut Self, name: &str) -> SymbData {
        let saved_scope = self.cur_scope;
        let mut found = false;
        loop {
            if (self.scopes[self.cur_scope as usize]
                .symbs
                .contains_key(name))
            {
                found = true;
                break;
            }
            self.cur_scope -= 1;
            if (self.cur_scope < 0) {
                break;
            }
        }

        let tmp_idx = self.cur_scope;
        self.cur_scope = saved_scope;


        return self.scopes[tmp_idx as usize].symbs[name].clone();
    }

    fn push_symb(self: &mut Self, data: SymbData) {
        assert!(self.cur_scope != -1);
        self.scopes[self.cur_scope as usize]
            .symbs
            .insert(data.name.clone(), data);
    }
}

pub struct SymenticAnal {
    scope_stk: ScopeStack,
    meta: Meta,
}

impl SymenticAnal {
    pub fn new() -> Self {
        SymenticAnal {
            scope_stk: ScopeStack::new(),
            meta: Meta::init(),
        }
    }
    pub fn analyse(self: &mut Self, program: &Stmt) {
        assert!(program.typ == StmType::Program);

        self.scope_stk.push_scope();
        for i in &self.meta.functions {
            self.scope_stk.push_symb(SymbData::func(format!("$.{}",i.0) , i.1.to_owned()));
        }

        self.analyze(program);
        self.scope_stk.pop_scope();
        
    }

    pub fn get_val_typ(self: &Self, stmt: &Stmt) -> SymbType {
        match stmt.typ {
            StmType::StringLiteral => {
                return SymbType::String;
            }
            StmType::HashMap => {
                return SymbType::HashMap;
            }
            StmType::FloatLiteral => {
                return SymbType::Number;
            }
            StmType::IntLiteral => {
                return SymbType::Number;
            }
            StmType::Arr => {
                return SymbType::Array;
            }
            _ => {}
        }
        return SymbType::Null;
    }

    fn push_hashmap_attrs(self: &Self, node: &Stmt, attrs: &mut HashMap<String, SymbData>) {
        let hashtable: &Vec<Vec<Stmt>> = get_stmt_typ!(&node.props["vals"], StmtValue::HashMap);

        for i in 0..hashtable.len() {
            let mut attr: SymbData = SymbData::new();
            attr.name = get_stmt_typ!(&hashtable[i][0].props["name"], StmtValue::Str).clone();
            attr.typ = self.get_val_typ(&hashtable[i][1]);

            if attr.typ == SymbType::HashMap {
                self.push_hashmap_attrs(&hashtable[i][1], &mut attr.attrs);
            }

            attrs.insert(attr.name.clone(), attr);
        }
    }
    fn from_literal_to_str(&self, stmt: &Stmt) -> String {
        match stmt.typ {
            StmType::StringLiteral => {
                format!("{:?}", get_stmt_typ!(&stmt.props["val"], StmtValue::Str))
            }
            StmType::FloatLiteral => get_stmt_typ!(&stmt.props["val"], StmtValue::Float)
                .to_string()
                .clone(),
            StmType::IntLiteral => get_stmt_typ!(&stmt.props["val"], StmtValue::Int)
                .to_string()
                .clone(),
            StmType::Ident => get_stmt_typ!(&stmt.props["name"], StmtValue::Str).clone(),
            StmType::Arr => {
                let mut vals: Vec<String> = vec![];
                let arr = get_stmt_typ!(&stmt.props["vals"], StmtValue::Arr);
                for elm in arr {
                    vals.push(self.from_literal_to_str(elm));
                }
                format!("{:?}", vals)
            }
            StmType::HashMap => {
                let mut out: String = "{".to_string();
                let arr = get_stmt_typ!(&stmt.props["vals"], StmtValue::HashMap);
                for i in 0..arr.len() {
                    out += format!(
                        "{:?} : {}",
                        self.from_literal_to_str(&arr[i][0]),
                        self.from_literal_to_str(&arr[i][1])
                    )
                    .as_str();
                    if (i != arr.len() - 1) {
                        out += ",";
                    }
                }
                out += "}";

                out
            }

            _ => {
                println!("{:?}", stmt);
                unreachable!()
            }
        }
    }
    fn parse_arglist(&self, node: &Stmt) -> (Vec<ArgInfo>, Vec<ArgInfo>) {
        let args = get_stmt_typ!(&node.props["list"], StmtValue::Arr);

        let mut optional_arg: Vec<ArgInfo> = vec![];
        let mut required_arg: Vec<ArgInfo> = vec![];

        for stmt in args {
            match stmt.typ {
                StmType::VariableAssignment => {
                    let name = get_stmt_typ!(
                        &get_stmt_typ!(&stmt.props["ident"]).props["name"],
                        StmtValue::Str
                    )
                    .clone();

                    let mut val: String =
                        self.from_literal_to_str(get_stmt_typ!(&stmt.props["val"]));
                    optional_arg.push(ArgInfo::not_required(name, val));
                }
                StmType::Ident
                | StmType::Arr
                | StmType::IntLiteral
                | StmType::HashMap
                | StmType::FloatLiteral
                | StmType::StringLiteral => {
                    required_arg.push(ArgInfo::required(self.from_literal_to_str(stmt)));
                }
                _ => {
                    unreachable!()
                }
            }
        }
        return (required_arg, optional_arg);
    }

    fn analyze(self: &mut Self, node: &Stmt) {
        match node.typ {
            StmType::Program => {
                self.scope_stk.push_scope();
                let body: &Vec<Stmt> = get_stmt_typ!(&node.props["body"], StmtValue::Arr);
                for stmt in body {
                    self.analyze(stmt);
                }
            }
            StmType::EOP => {}
            StmType::FloatLiteral => {}
            StmType::IntLiteral => {}
            StmType::StringLiteral => {}
            StmType::BooleanLiteral => {}
            StmType::Ident => {
                let ident_name = get_stmt_typ!(&node.props["name"], StmtValue::Str);
                if (!self.scope_stk.check_symb(ident_name)) {
                    println!("line {}: variable '{}' not declared", node.line, ident_name);
                }
            }
            StmType::VariableDeclaration => {
                self.analyze(get_stmt_typ!(&node.props["val"]));

                let mut data = SymbData::variable(
                    get_stmt_typ!(
                        &get_stmt_typ!(&node.props["ident"]).props["name"],
                        StmtValue::Str
                    )
                    .clone(),
                    self.get_val_typ(get_stmt_typ!(&node.props["val"])),
                );

                if (data.typ == SymbType::HashMap) {
                    self.push_hashmap_attrs(get_stmt_typ!(&node.props["val"]), &mut data.attrs);
                }

                self.scope_stk.push_symb(data);
            }
            StmType::VariableAssignment => {
                self.analyze(get_stmt_typ!(&node.props["ident"]));
                self.analyze(get_stmt_typ!(&node.props["val"]));
            }
            StmType::ArthExpr => {
                self.analyze(get_stmt_typ!(&node.props["lhs"]));
                self.analyze(get_stmt_typ!(&node.props["rhs"]));
            }
            StmType::BooleanExpr => {
                self.analyze(get_stmt_typ!(&node.props["lhs"]));
                self.analyze(get_stmt_typ!(&node.props["rhs"]));
            }
            StmType::DotExpr => {
                if (get_stmt_typ!(&node.props["lhs"]).typ == StmType::SysIdent) {
                    let rhs = get_stmt_typ!(&node.props["rhs"]);
                    match rhs.typ {
                        StmType::FuncCall => {
                            self.analyze(rhs);
                        }
                        StmType::Ident => {
                            unreachable!();
                        }
                        _ => {
                            println!("line {}: system only accesses variables or functions you tried to access {:?}",rhs.line,rhs.typ);
                        }
                    }
                }
            }
            StmType::IfStmt => {
                self.analyze(get_stmt_typ!(&node.props["condition"]));
                self.analyze(get_stmt_typ!(&node.props["body"]));

                let else_ifs = get_stmt_typ!(&node.props["else_ifs"], StmtValue::Arr);
                for else_if in else_ifs {
                    self.analyze(get_stmt_typ!(&else_if.props["condition"], StmtValue::Stmt));
                    self.analyze(get_stmt_typ!(&else_if.props["body"]));
                }

                // else
                if (node.props.contains_key("else")) {
                    self.analyze(get_stmt_typ!(
                        &get_stmt_typ!(&node.props["else"]).props["body"]
                    ));
                }
            }
            StmType::ForStmt => {
                self.scope_stk.push_scope();
                self.analyze(get_stmt_typ!(&node.props["decl"]));
                self.analyze(get_stmt_typ!(&node.props["condition"]));
                self.analyze(get_stmt_typ!(&node.props["action"]));
                self.analyze(get_stmt_typ!(&node.props["body"]));
                self.scope_stk.pop_scope();
            }
            StmType::WhileStmt => {
                self.analyze(get_stmt_typ!(&node.props["condition"]));
                self.analyze(get_stmt_typ!(&node.props["body"]));
            }
            StmType::FuncDeclaration => {
                let func_name = get_stmt_typ!(
                    &get_stmt_typ!(&node.props["name"]).props["name"],
                    StmtValue::Str
                )
                .clone();

                if self.scope_stk.check_symb(&func_name) {
                    println!("line {}: function '{func_name}' already been declared",node.line);
                }
                
                self.scope_stk.push_scope();

                let mut required_arg: Vec<ArgInfo> = vec![];
                let mut optional_arg: Vec<ArgInfo> = vec![];

                let mut args: &Vec<Stmt>;

                if node.props.contains_key("arglist") {
                    args = get_stmt_typ!(
                        &get_stmt_typ!(&node.props["arglist"]).props["list"],
                        StmtValue::Arr
                    );

                    for stmt in args {
                        match stmt.typ {
                            StmType::VariableAssignment => {
                                let name = get_stmt_typ!(
                                    &get_stmt_typ!(&stmt.props["ident"]).props["name"],
                                    StmtValue::Str
                                )
                                .clone();
                                self.scope_stk.push_symb(SymbData::variable(
                                    name.clone(),
                                    self.get_val_typ(get_stmt_typ!(&stmt.props["val"])),
                                ));

                                let mut val: String =
                                    self.from_literal_to_str(get_stmt_typ!(&stmt.props["val"]));
                                optional_arg.push(ArgInfo::not_required(name, val));
                            }
                            StmType::Ident => {
                                let name =
                                    get_stmt_typ!(&stmt.props["name"], StmtValue::Str).clone();
                                self.scope_stk
                                    .push_symb(SymbData::variable(name.clone(), SymbType::Null));
                                required_arg.push(ArgInfo::required(name.clone()));
                            }
                            _ => unreachable!(),
                        }
                    }
                }
                self.analyze(get_stmt_typ!(&node.props["body"]));
                self.scope_stk.pop_scope();


                self.scope_stk.push_symb(SymbData::func(
                    func_name.clone(),
                    FuncData::new(func_name.clone(), &required_arg, &optional_arg),
                ));
            }
            StmType::ArgList => {
                let list = get_stmt_typ!(&node.props["list"], StmtValue::Arr);
                for stmt in list {
                    // skip optional args
                    if stmt.typ != StmType::VariableAssignment {
                        self.analyze(stmt);
                    }
                }
            }
            StmType::FuncCall => {
                if node.props.contains_key("arglist") {
                    self.analyze(get_stmt_typ!(&node.props["arglist"]));
                }

                self.analyze(get_stmt_typ!(&node.props["name"]));
                let func_name = get_stmt_typ!(
                    &get_stmt_typ!(&node.props["name"]).props["name"],
                    StmtValue::Str
                );

                if (!self.scope_stk.check_symb(&func_name)) {
                    println!("line {}: function '{}' not declared",node.line, func_name);
                } else {
                    if (!self.scope_stk.get_symb(&func_name).is_func) {
                        println!("variable '{}' is not callable", func_name);
                    }
                }

                let symb = self.scope_stk.get_symb(&func_name);
                let mut required_args: Vec<ArgInfo> = vec![];
                let mut optional_args: Vec<ArgInfo> = vec![];

                if (node.props.contains_key("arglist")) {
                    (required_args, optional_args) =
                        self.parse_arglist(get_stmt_typ!(&node.props["arglist"]));
                }

                // wrong number of required args
                if symb.func_data.required_args.len() != required_args.len() {
                    if symb.func_data.required_args.len() == 0
                        || symb.func_data.required_args.len() == 1
                    {
                        println!("line {}: wrong number of arguments for function '{}', {} is required but got {}",node.line,func_name,symb.func_data.required_args.len(),required_args.len());
                    } else {
                        println!("line {}: wrong number of arguments for function '{}', {} are required but got {}",node.line,func_name,symb.func_data.required_args.len(),required_args.len());
                    }
                }

                if node.props.contains_key("arglist") {
                    // check optional args
                    let mut func_optional_args: Vec<String> = vec![];
                    for i in 0..symb.func_data.optional_args.len() {
                        func_optional_args.push(symb.func_data.optional_args[i].name.clone());
                    }

                    for arg in &optional_args {
                        if !func_optional_args.contains(&arg.name) {
                            println!(
                                "line {}: unknown optional argument provided '{}'",
                                node.line, arg.name
                            );
                        }
                    }
                }
            }

            StmType::StmtBlock => {
                self.scope_stk.push_scope();
                let body: &Vec<Stmt> = get_stmt_typ!(&node.props["body"], StmtValue::Arr);
                for stmt in body {
                    self.analyze(stmt);
                }
                self.scope_stk.pop_scope();
            }

            StmType::Return => {
                self.analyze(get_stmt_typ!(&node.props["val"]));
            }
            StmType::Arr => {
                let vals = get_stmt_typ!(&node.props["vals"], StmtValue::Arr);
                for val in vals {
                    self.analyze(&val);
                }
            }
            StmType::GroupExpr => {
                self.analyze(get_stmt_typ!(&node.props["val"], StmtValue::Stmt));
            }

            StmType::HashMap => {
                let vals = get_stmt_typ!(&node.props["vals"], StmtValue::HashMap);
                for val in vals {
                    self.analyze(&val[1]);
                }
            }

            _ => {
                println!("{:?}", node);
                unimplemented!()
            }
        }
    }
}
