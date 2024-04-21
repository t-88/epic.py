use std::collections::HashMap;

use crate::{get_stmt_typ, parser::*, TknType};

#[derive(Debug, PartialEq, Eq,Clone)]
pub enum SymbType {
    Null,
    String,
    Number,
    HashMap,
    Array,
}

#[derive(Debug,Clone)]
struct SymbData {
    name: String,
    is_func: bool,
    typ: SymbType,
    attrs: HashMap<String, SymbData>,
}

impl SymbData {
    fn new() -> Self {
        return SymbData {
            name: "".to_string(),
            is_func: false,
            typ: SymbType::Null,
            attrs: HashMap::new(),
        };
    }

    fn variable(name: String, typ: SymbType) -> SymbData {
        let mut data = SymbData::new();
        data.name = name;
        data.typ = typ;
        data
    }

    fn func(name: String) -> SymbData {
        let mut data = SymbData::new();
        data.name = name;
        data.is_func = true;
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

        assert!(found);

        let tmp_idx = self.cur_scope;
        self.cur_scope = saved_scope;

        return self.scopes[tmp_idx as usize].symbs[name].clone();
    }

    fn push_symb(self: &mut Self, data: SymbData) {
        self.scopes[self.cur_scope as usize]
            .symbs
            .insert(data.name.clone(), data);
    }
}

pub struct SymenticAnal {
    scope_stk: ScopeStack,
}

impl SymenticAnal {
    pub fn new() -> Self {
        SymenticAnal {
            scope_stk: ScopeStack::new(),
        }
    }
    pub fn analyse(self: &mut Self, program: &Stmt) {
        assert!(program.typ == StmType::Program);
        self.analyse_variables(program);
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
            _ => {
            }
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

    fn analyse_variables(self: &mut Self, node: &Stmt) {
        match node.typ {
            StmType::Program => {
                self.scope_stk.push_scope();
                let body: &Vec<Stmt> = get_stmt_typ!(&node.props["body"], StmtValue::Arr);
                for stmt in body {
                    self.analyse_variables(stmt);
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
                    println!("line {}: variable '{}' not declared",node.line, ident_name);
                }
            }
            StmType::VariableDeclaration => {
                self.analyse_variables(get_stmt_typ!(&node.props["val"]));

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
                self.analyse_variables(get_stmt_typ!(&node.props["ident"]));
                self.analyse_variables(get_stmt_typ!(&node.props["val"]));
            }
            StmType::ArthExpr => {
                self.analyse_variables(get_stmt_typ!(&node.props["lhs"]));
                self.analyse_variables(get_stmt_typ!(&node.props["rhs"]));
            }
            StmType::BooleanExpr => {
                self.analyse_variables(get_stmt_typ!(&node.props["lhs"]));
                self.analyse_variables(get_stmt_typ!(&node.props["rhs"]));
            }
            StmType::DotExpr => {
            }

            StmType::IfStmt => {
                self.analyse_variables(get_stmt_typ!(&node.props["condition"]));
                self.analyse_variables(get_stmt_typ!(&node.props["body"]));

                let else_ifs = get_stmt_typ!(&node.props["else_ifs"], StmtValue::Arr);
                for else_if in else_ifs {
                    self.analyse_variables(get_stmt_typ!(
                        &else_if.props["condition"],
                        StmtValue::Stmt
                    ));
                    self.analyse_variables(get_stmt_typ!(&else_if.props["body"]));
                }

                // else
                if (node.props.contains_key("else")) {
                    self.analyse_variables(get_stmt_typ!(
                        &get_stmt_typ!(&node.props["else"]).props["body"]
                    ));
                }
            }
            StmType::ForStmt => {
                self.scope_stk.push_scope();
                self.analyse_variables(get_stmt_typ!(&node.props["decl"]));
                self.analyse_variables(get_stmt_typ!(&node.props["condition"]));
                self.analyse_variables(get_stmt_typ!(&node.props["action"]));
                self.analyse_variables(get_stmt_typ!(&node.props["body"]));
                self.scope_stk.pop_scope();
            }
            StmType::WhileStmt => {
                self.analyse_variables(get_stmt_typ!(&node.props["condition"]));
                self.analyse_variables(get_stmt_typ!(&node.props["body"]));
            }
            StmType::FuncDeclaration => {
                self.scope_stk.push_symb(SymbData::func(
                    get_stmt_typ!(
                        &get_stmt_typ!(&node.props["name"]).props["name"],
                        StmtValue::Str
                    )
                    .clone(),
                ));
                self.scope_stk.push_scope();

                if node.props.contains_key("arglist") {
                    let list = get_stmt_typ!(
                        &get_stmt_typ!(&node.props["arglist"]).props["list"],
                        StmtValue::Arr
                    );

                    for stmt in list {
                        match stmt.typ {
                            StmType::VariableAssignment => {
                                self.scope_stk.push_symb(SymbData::variable(
                                    get_stmt_typ!(
                                        &get_stmt_typ!(&stmt.props["ident"]).props["name"],
                                        StmtValue::Str
                                    )
                                    .clone(),
                                    self.get_val_typ(get_stmt_typ!(&node.props["val"])),
                                ));
                            }
                            StmType::Ident => {
                                self.scope_stk.push_symb(SymbData::variable(
                                    get_stmt_typ!(&stmt.props["name"], StmtValue::Str).clone(),
                                    SymbType::Null,
                                ));
                            }
                            _ => unreachable!(),
                        }
                    }
                }

                self.analyse_variables(get_stmt_typ!(&node.props["body"]));

                self.scope_stk.pop_scope();
            }
            StmType::ArgList => {
                let list = get_stmt_typ!(&node.props["list"], StmtValue::Arr);
                for stmt in list {
                    self.analyse_variables(stmt);
                }
            }
            StmType::FuncCall => {
                if node.props.contains_key("arglsit") {
                    self.analyse_variables(get_stmt_typ!(&node.props["arglist"]));
                }

                self.analyse_variables(get_stmt_typ!(&node.props["name"]));
                let func_name = get_stmt_typ!(
                    &get_stmt_typ!(&node.props["name"]).props["name"],
                    StmtValue::Str
                );
                if (self.scope_stk.get_symb(func_name).is_func) {
                    println!("variable '{}' is not callable", func_name);
                }
            }

            StmType::StmtBlock => {
                self.scope_stk.push_scope();
                let body: &Vec<Stmt> = get_stmt_typ!(&node.props["body"], StmtValue::Arr);
                for stmt in body {
                    self.analyse_variables(stmt);
                }
                self.scope_stk.pop_scope();
            }

            StmType::Return => {
                self.analyse_variables(get_stmt_typ!(&node.props["val"]));
            }
            StmType::Arr => {
                let vals = get_stmt_typ!(&node.props["vals"], StmtValue::Arr);
                for val in vals {
                    self.analyse_variables(&val);
                }
            }
            StmType::HashMap => {
                let vals = get_stmt_typ!(&node.props["vals"], StmtValue::HashMap);
                for val in vals {
                    self.analyse_variables(&val[1]);
                }
            }

            _ => {
                println!("{:?}", node);
                unimplemented!()
            }
        }
    }
}
