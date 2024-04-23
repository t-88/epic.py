use std::collections::HashMap;

use crate::*;

pub struct Transpiler {
    js_functions: HashMap<String, String>,
}

impl Transpiler {
    pub fn new() -> Transpiler {
        let mut trans: Transpiler = Transpiler {
            js_functions: HashMap::new(),
        };
        trans.fill_js_prebuilds();
        return trans;
    }

    pub fn fill_js_prebuilds(&mut self) {
        self.js_functions
            .insert("log".to_string(), "console.log".to_string());
        self.js_functions
            .insert("get_component".to_string(), "sys__get_component".to_string());

        self.js_functions.insert(
            "get_entity_by_id".to_string(),
            "get_entity_by_id".to_string(),
        );

        self.js_functions
            .insert("randint".to_string(), "sys__randint".to_string());

        self.js_functions
            .insert("clear_entities".to_string(), "sys__clear_entities".to_string());

        self.js_functions
            .insert("init".to_string(), "sys__init".to_string());

        self.js_functions
            .insert("sqrt".to_string(), "sys__sqrt".to_string());

        self.js_functions
            .insert("AABB".to_string(), "sys__AABB".to_string());
    }
    pub fn js_transpiler(&self, node: &Stmt, depth: usize, add_semi: &mut bool) -> String {
        let mut src = "".to_string();
        let spacing: String = "  ".to_string();

        match node.typ {
            StmType::Program => {
                if let StmtValue::Arr(body) = &node.props["body"] {
                    for node in body {
                        src += &self.js_transpiler(node, depth, &mut true);
                        src += "\n"
                    }
                }
                return src;
            }
            StmType::EOP => {}

            // literals
            StmType::FloatLiteral => {
                src = format!(
                    "{}{}",
                    spacing.repeat(depth),
                    get_stmt_typ!(&node.props["val"], StmtValue::Float)
                );
                if (*add_semi) {
                    src += ";";
                }
                return src;
            }
            StmType::IntLiteral => {
                src = format!(
                    "{}{}",
                    spacing.repeat(depth),
                    get_stmt_typ!(&node.props["val"], StmtValue::Int)
                );
                if (*add_semi) {
                    src += ";";
                }
                return src;
            }
            StmType::BooleanLiteral => {
                src = format!(
                    "{}{}",
                    spacing.repeat(depth),
                    get_stmt_typ!(&node.props["val"], StmtValue::Bool)
                );
                if (*add_semi) {
                    src += ";";
                }
                return src;
            }
            StmType::StringLiteral => {
                src = format!(
                    "{}{:?}",
                    spacing.repeat(depth),
                    get_stmt_typ!(&node.props["val"], StmtValue::Str)
                );
                if (*add_semi) {
                    src += ";";
                }
                return src;
            }
            StmType::Ident => {
                src = format!(
                    "{}{}",
                    spacing.repeat(depth),
                    get_stmt_typ!(&node.props["name"], StmtValue::Str)
                );
                if (*add_semi) {
                    src += ";";
                }
                return src;
            }
            StmType::SysIdent => {
                unreachable!("sys ident in js tranpiler");
                return "".to_string();
            }
            StmType::Arr => {
                let vals = get_stmt_typ!(&node.props["vals"], StmtValue::Arr);
                src += "[";
                if vals.len() != 0 {
                    for i in 0..vals.len() {
                        src += &self.js_transpiler(&vals[i], 0, &mut false);
                        if i != vals.len() - 1 {
                            src += ",";
                        }
                    }
                }
                src += "]";
                if (*add_semi) {
                    src += ";";
                }
                return src;
            }
            StmType::GroupExpr => {
                let val = self.js_transpiler(
                    get_stmt_typ!(&node.props["val"], StmtValue::Stmt),
                    0,
                    &mut false,
                );
                src = format!("{}({val})", spacing.repeat(depth));
                if (*add_semi) {
                    src += ";";
                }
                return src;
            }

            StmType::VariableDeclaration => {
                let ident = self.js_transpiler(get_stmt_typ!(&node.props["ident"]), 0, &mut false);
                let val = self.js_transpiler(get_stmt_typ!(&node.props["val"]), 0, &mut false);
                src = format!("{}let {} = {}", spacing.repeat(depth), ident, val);
                if (*add_semi) {
                    src += ";";
                }
                return src;
            }
            StmType::VariableAssignment => {
                let ident = self.js_transpiler(get_stmt_typ!(&node.props["ident"]), 0, &mut false);
                let val = self.js_transpiler(get_stmt_typ!(&node.props["val"]), 0, &mut false);
                src = format!("{}{} = {}", spacing.repeat(depth), ident, val);
                if (*add_semi) {
                    src += ";";
                }
                return src;
            }

            // ops
            StmType::ArthExpr => {
                let op = get_stmt_typ!(&node.props["op"], StmtValue::Str);
                let lhs = self.js_transpiler(get_stmt_typ!(&node.props["lhs"]), 0, &mut false);
                let rhs = self.js_transpiler(get_stmt_typ!(&node.props["rhs"]), 0, &mut false);
                src = format!("{}{} {} {}", spacing.repeat(depth), lhs, op, rhs);
                if (*add_semi) {
                    src += ";";
                }
                return src;
            }
            StmType::BooleanExpr => {
                let op = get_stmt_typ!(&node.props["op"], StmtValue::Str);
                let lhs = self.js_transpiler(get_stmt_typ!(&node.props["lhs"]), 0, &mut false);
                let rhs = self.js_transpiler(get_stmt_typ!(&node.props["rhs"]), 0, &mut false);
                src = format!("{}{} {} {}", spacing.repeat(depth), lhs, op, rhs);
                if (*add_semi) {
                    src += ";";
                }

                return src;
            }

            StmType::DotExpr => {
                if (get_stmt_typ!(&node.props["lhs"]).typ == StmType::SysIdent) {
                    let rhs = get_stmt_typ!(&node.props["rhs"]);
                    match rhs.typ {
                        StmType::FuncCall => {
                            src = self.js_transpiler(rhs, 0, &mut false);
                        }
                        _ => {
                            unreachable!();
                        }
                    }
                } else {
                    let lhs = self.js_transpiler(get_stmt_typ!(&node.props["lhs"]), 0, &mut false);
                    let rhs = self.js_transpiler(get_stmt_typ!(&node.props["rhs"]), 0, &mut false);
                    src = format!("{}{lhs}.{rhs}", spacing.repeat(depth));
                    if (*add_semi) {
                        src += ";";
                    }
                }
                return src;
            }

            // stmts
            StmType::IfStmt => {
                let condition =
                    self.js_transpiler(get_stmt_typ!(&node.props["condition"]), 0, &mut false);
                let body =
                    self.js_transpiler(get_stmt_typ!(&node.props["body"]), depth + 1, &mut false);

                // else if
                let else_ifs = get_stmt_typ!(&node.props["else_ifs"], StmtValue::Arr);

                let mut else_ifs_src = "".to_string();

                if (!else_ifs.is_empty()) {
                    for else_if in else_ifs {
                        let condition = self.js_transpiler(
                            get_stmt_typ!(&else_if.props["condition"]),
                            0,
                            &mut false,
                        );
                        let body = self.js_transpiler(
                            get_stmt_typ!(&else_if.props["body"]),
                            depth + 1,
                            &mut false,
                        );
                        else_ifs_src +=
                            &format!("{}else if({condition}){body}", spacing.repeat(depth));
                    }
                }

                // else
                let mut else_src = "".to_string();
                if (node.props.contains_key("else")) {
                    let body = self.js_transpiler(
                        get_stmt_typ!(&get_stmt_typ!(&node.props["else"]).props["body"]),
                        depth + 1,
                        &mut false,
                    );
                    else_src += &format!("{}else{body}", spacing.repeat(depth));
                }

                src = format!(
                    "{}if({condition}){body}{else_ifs_src}{else_src}",
                    spacing.repeat(depth)
                );
                return src;
            }
            StmType::ForStmt => {
                let decl = self.js_transpiler(get_stmt_typ!(&node.props["decl"]), 0, &mut false);
                let condition =
                    self.js_transpiler(get_stmt_typ!(&node.props["condition"]), 0, &mut false);
                let action =
                    self.js_transpiler(get_stmt_typ!(&node.props["action"]), 0, &mut false);
                let body =
                    self.js_transpiler(get_stmt_typ!(&node.props["body"]), depth + 1, &mut false);

                src = format!("for({decl};{condition};{action}){body}");
                return src;
            }
            StmType::WhileStmt => {
                let condition =
                    self.js_transpiler(get_stmt_typ!(&node.props["condition"]), 0, &mut false);
                let body =
                    self.js_transpiler(get_stmt_typ!(&node.props["body"]), depth + 1, &mut false);
                src = format!("while({condition}){body}");
                return src;
            }

            StmType::FuncDeclaration => {
                let name = self.js_transpiler(get_stmt_typ!(&node.props["name"]), 0, &mut false);
                let mut args = "".to_string();
                if node.props.contains_key("arglist") {
                    args += &self.js_transpiler(
                        get_stmt_typ!(&node.props["arglist"]),
                        depth + 1,
                        &mut false,
                    );
                }
                let body =
                    self.js_transpiler(get_stmt_typ!(&node.props["body"]), depth + 1, &mut false);
                src = format!("function {name}({args}){body}");
                return src;
            }

            StmType::FuncCall => {
                let mut name =
                    self.js_transpiler(get_stmt_typ!(&node.props["name"]), 0, &mut false);
                if (name.starts_with("$.")) {
                    name = self
                        .js_functions
                        .get(&name.split_off(2))
                        .unwrap()
                        .to_string();
                }
                let mut args: String = "".to_string();
                if node.props.contains_key("arglist") {
                    args = self.js_transpiler(get_stmt_typ!(&node.props["arglist"]), 0, &mut false);
                }

                src = format!("{}{name}({args})", spacing.repeat(depth));
                if (*add_semi) {
                    src += ";";
                }
                return src;
            }

            StmType::ArgList => {
                let args = get_stmt_typ!(&node.props["list"], StmtValue::Arr);

                for i in 0..args.len() {
                    src += &self.js_transpiler(&args[i], 0, &mut false);
                    if i != args.len() - 1 {
                        src += ",";
                    }
                }
                return src;
            }

            StmType::Return => {
                let val = self.js_transpiler(get_stmt_typ!(&node.props["val"]), 0, &mut false);
                src = format!("{}return {val}", spacing.repeat(depth));
                if (*add_semi) {
                    src += ";";
                }
                return src;
            }

            StmType::HashMap => {
                let vals = get_stmt_typ!(&node.props["vals"], StmtValue::HashMap);
                src = "{".to_string();

                if vals.len() != 0 {
                    for i in 0..vals.len() {
                        let key = self.js_transpiler(&vals[i][0], 0, &mut false);
                        let val = self.js_transpiler(&vals[i][1], 0, &mut false);
                        src += format!("{key} : {val}").as_str();
                        if (i != vals.len() - 1) {
                            src += ",";
                        }
                    }
                }
                src += "}";
                if (*add_semi) {
                    src += ";";
                }
                return src;
            }
            StmType::StmtBlock => {
                src += &format!("{{\n");
                match &node.props["body"] {
                    StmtValue::Arr(block) => {
                        if (block.len() == 0) {
                        } else {
                            for stmt in block {
                                src += &self.js_transpiler(stmt, depth + 1, &mut true);
                                src += "\n";
                            }
                        }
                    }
                    _ => unreachable!(),
                }
                src += &format!("{}}}\n", spacing.repeat(depth - 1));
                return src;
            }
            _ => {
                println!("{:?}", node);
                unreachable!();
                return "".to_string();
            }
        }
        return "".to_string();
    }
}
