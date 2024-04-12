use std::collections::HashMap;

use crate::parser::*;

macro_rules! get_stmt_typ {
    ($value: expr , $typ: path) => {
        match $value {
            $typ(x) => x,
            _ => unreachable!(),
        }
    };
}

#[derive(Debug)]
pub struct Scope {
    idents: HashMap<String, bool>,
}
impl Scope {
    fn new() -> Self {
        Scope {
            idents: HashMap::new(),
        }
    }
}

#[derive(Debug)]
pub struct ScopeStack {
    cur_scope: i64,
    scopes: Vec<Scope>,
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
        self.scopes.push(Scope::new());
    }

    fn pop_scope(self: &mut Self) {
        self.cur_scope -= 1;
        self.scopes.pop();
    }
    fn check_ident(self: &mut Self, name: &str) -> bool {
        let saved_scope = self.cur_scope;
        let mut found = false;
        loop {
            if (self.scopes[self.cur_scope as usize]
                .idents
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
    fn push_ident(self: &mut Self, ident: String) {
        self.scopes[self.cur_scope as usize]
            .idents
            .insert(ident, true);
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
    pub fn analyse(self: &mut Self, program: Stmt) {
        assert!(program.typ == StmType::Program);
        self.analyse_variables(&program);
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
                if (!self.scope_stk.check_ident(ident_name)) {
                    println!("{:?}", self.scope_stk);
                    println!("variables '{}' not declared", ident_name);
                }
            }
            StmType::VariableDeclaration => {
                let ident_name = get_stmt_typ!(
                    &get_stmt_typ!(&node.props["ident"], StmtValue::Stmt).props["name"],
                    StmtValue::Str
                );

                self.scope_stk.push_ident((*ident_name).clone());

                self.analyse_variables(get_stmt_typ!(&node.props["val"], StmtValue::Stmt));
            }
            StmType::VariableAssignment => {
                self.analyse_variables(get_stmt_typ!(&node.props["ident"], StmtValue::Stmt));
                self.analyse_variables(get_stmt_typ!(&node.props["val"], StmtValue::Stmt));
            }
            StmType::ArthExpr => {
                self.analyse_variables(get_stmt_typ!(&node.props["lhs"], StmtValue::Stmt));
                self.analyse_variables(get_stmt_typ!(&node.props["rhs"], StmtValue::Stmt));
            }
            StmType::BooleanExpr => {
                self.analyse_variables(get_stmt_typ!(&node.props["lhs"], StmtValue::Stmt));
                self.analyse_variables(get_stmt_typ!(&node.props["rhs"], StmtValue::Stmt));
            }

            StmType::IfStmt => {
                self.analyse_variables(get_stmt_typ!(&node.props["condition"], StmtValue::Stmt));
                self.analyse_variables(get_stmt_typ!(&node.props["block"], StmtValue::Stmt));

                let else_ifs = get_stmt_typ!(&node.props["else_ifs"], StmtValue::Arr);
                for else_if in else_ifs {
                    self.analyse_variables(get_stmt_typ!(
                        &else_if.props["condition"],
                        StmtValue::Stmt
                    ));
                    self.analyse_variables(get_stmt_typ!(&else_if.props["block"], StmtValue::Stmt));
                }

                let elses = get_stmt_typ!(&node.props["else"], StmtValue::Stmt);
                if (elses.props.contains_key("block")) {
                    self.analyse_variables(get_stmt_typ!(&elses.props["block"], StmtValue::Stmt));
                }
            }
            StmType::ForStmt => {
                self.scope_stk.push_scope();
                self.analyse_variables(get_stmt_typ!(&node.props["decl"], StmtValue::Stmt));
                self.analyse_variables(get_stmt_typ!(&node.props["condition"], StmtValue::Stmt));
                self.analyse_variables(get_stmt_typ!(&node.props["action"], StmtValue::Stmt));
                self.analyse_variables(get_stmt_typ!(&node.props["body"], StmtValue::Stmt));
                self.scope_stk.pop_scope();
            }
            StmType::WhileStmt => {
                self.analyse_variables(get_stmt_typ!(&node.props["condition"], StmtValue::Stmt));
                self.analyse_variables(get_stmt_typ!(&node.props["body"], StmtValue::Stmt));
            }
            StmType::FuncDeclaration => {
                self.scope_stk.push_ident(
                    get_stmt_typ!(
                        &get_stmt_typ!(&node.props["name"], StmtValue::Stmt).props["name"],
                        StmtValue::Str
                    )
                    .clone(),
                );
                self.scope_stk.push_scope();

                if node.props.contains_key("arglist") {
                    let list = get_stmt_typ!(
                        &get_stmt_typ!(&node.props["arglist"], StmtValue::Stmt).props["list"],
                        StmtValue::Arr
                    );

                    for stmt in list {
                        match stmt.typ {
                            StmType::VariableAssignment => {
                                self.scope_stk.push_ident(
                                    get_stmt_typ!(
                                        &get_stmt_typ!(&stmt.props["ident"], StmtValue::Stmt).props
                                            ["name"],
                                        StmtValue::Str
                                    )
                                    .clone(),
                                );
                            }
                            StmType::Ident => {
                                self.scope_stk.push_ident(
                                    get_stmt_typ!(&stmt.props["name"], StmtValue::Str).clone(),
                                );
                            }
                            _ => unreachable!(),
                        }
                    }
                }

                self.analyse_variables(get_stmt_typ!(&node.props["body"], StmtValue::Stmt));

                self.scope_stk.pop_scope();
            }
            StmType::ArgList => {
                let list = get_stmt_typ!(&node.props["list"], StmtValue::Arr);
                for stmt in list {
                    self.analyse_variables(stmt);
                }
            }
            StmType::FuncCall => {
                self.analyse_variables(get_stmt_typ!(&node.props["name"], StmtValue::Stmt));
                self.analyse_variables(get_stmt_typ!(&node.props["arglist"], StmtValue::Stmt));
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
                self.analyse_variables(get_stmt_typ!(&node.props["val"], StmtValue::Stmt));
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
