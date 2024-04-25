use std::collections::HashMap;

use crate::*;
use self::parser::*;
use self::lexer::*;
use crate::transpiler::Transpiler;


impl Transpiler {
    
    pub fn fill_py_prebuilds(&mut self) {
        let mut py_functions : HashMap<String,String> = HashMap::new();
        
        py_functions.insert("log".to_string(), "print".to_string());
        py_functions.insert("sqrt".to_string(), "math.sqrt".to_string());
        py_functions.insert("randint".to_string(), "random.randint".to_string());
        self.same_name_meta(&mut py_functions, "get_component");
        self.same_name_meta(&mut py_functions, "get_entity_by_id");
        self.same_name_meta(&mut py_functions, "clear_entities");
        self.same_name_meta(&mut py_functions, "clear_entities");
        self.same_name_meta(&mut py_functions, "init");
        self.same_name_meta(&mut py_functions, "AABB");
        self.same_name_meta(&mut py_functions, "is_pressed");
        self.same_name_meta(&mut py_functions, "remove_entity");
        self.same_name_meta(&mut py_functions, "create_entity");

        self.py_functions = py_functions;
    }

pub fn py_transpiler(&self, node: &Stmt, depth: usize, add_semi: &mut bool, func_prefix: &String) -> String {
    let mut src = "".to_string();
    let spacing: String = "  ".to_string();

    match node.typ {
        StmType::Program => {
            if let StmtValue::Arr(body) = &node.props["body"] {
                for node in body {
                    src += &self.py_transpiler(node, depth, &mut true,func_prefix);
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
            if *get_stmt_typ!(&node.props["val"], StmtValue::Bool) {
                src = format!(
                    "{}True",
                    spacing.repeat(depth),
                );
            } else {
                src = format!(
                    "{}False",
                    spacing.repeat(depth),
                );
            }
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
                    src += &self.py_transpiler(&vals[i], 0, &mut false,func_prefix);
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
            let val = self.py_transpiler(
                get_stmt_typ!(&node.props["val"], StmtValue::Stmt),
                0,
                &mut false,
                func_prefix
            );
            src = format!("{}({val})", spacing.repeat(depth));
            if (*add_semi) {
                src += ";";
            }
            return src;
        }

        StmType::VariableDeclaration => {
            let ident = self.py_transpiler(get_stmt_typ!(&node.props["ident"]), 0, &mut false,func_prefix);
            let val = self.py_transpiler(get_stmt_typ!(&node.props["val"]), 0, &mut false,func_prefix);
            src = format!("{}{} = {}", spacing.repeat(depth), ident, val);
            if (*add_semi) {
                src += ";";
            }
            return src;
        }
        StmType::VariableAssignment => {
            let ident = self.py_transpiler(get_stmt_typ!(&node.props["ident"]), 0, &mut false,func_prefix);
            let val = self.py_transpiler(get_stmt_typ!(&node.props["val"]), 0, &mut false,func_prefix);
            src = format!("{}{} = {}", spacing.repeat(depth), ident, val);
            if (*add_semi) {
                src += ";";
            }
            return src;
        }

        // ops
        StmType::ArthExpr => {
            let op = get_stmt_typ!(&node.props["op"], StmtValue::Str);
            let lhs = self.py_transpiler(get_stmt_typ!(&node.props["lhs"]), 0, &mut false,func_prefix);
            let rhs = self.py_transpiler(get_stmt_typ!(&node.props["rhs"]), 0, &mut false,func_prefix);
            src = format!("{}{} {} {}", spacing.repeat(depth), lhs, op, rhs);
            if (*add_semi) {
                src += ";";
            }
            return src;
        }
        StmType::BooleanExpr => {
            let mut op = get_stmt_typ!(&node.props["op"], StmtValue::Str).to_string();
            match op.as_str() {
                "&&" => op = "and".to_string(),
                "||" => op = "or".to_string(),
                _ => {

                }
            }

            let lhs = self.py_transpiler(get_stmt_typ!(&node.props["lhs"]), 0, &mut false,func_prefix);
            let rhs = self.py_transpiler(get_stmt_typ!(&node.props["rhs"]), 0, &mut false,func_prefix);
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
                        src = self.py_transpiler(rhs, depth, &mut false,func_prefix);
                    }
                    _ => {
                        unreachable!();
                    }
                }
            } else {
                let lhs = self.py_transpiler(get_stmt_typ!(&node.props["lhs"]), 0, &mut false,func_prefix);
                let rhs = self.py_transpiler(get_stmt_typ!(&node.props["rhs"]), 0, &mut false,func_prefix);
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
                self.py_transpiler(get_stmt_typ!(&node.props["condition"]), 0, &mut false,func_prefix);
            let body =
                self.py_transpiler(get_stmt_typ!(&node.props["body"]), depth + 1, &mut false,func_prefix);

            // else if
            let else_ifs = get_stmt_typ!(&node.props["else_ifs"], StmtValue::Arr);

            let mut else_ifs_src = "".to_string();

            if (!else_ifs.is_empty()) {
                for else_if in else_ifs {
                    let condition = self.py_transpiler(
                        get_stmt_typ!(&else_if.props["condition"]),
                        0,
                        &mut false,
                    func_prefix
                    );
                    let body = self.py_transpiler(
                        get_stmt_typ!(&else_if.props["body"]),
                        depth + 1,
                        &mut false,
                    func_prefix
                    );
                    else_ifs_src +=
                        &format!("{}elif({condition}):{body}", spacing.repeat(depth));
                }
            }

            // else
            let mut else_src = "".to_string();
            if (node.props.contains_key("else")) {
                let body = self.py_transpiler(
                    get_stmt_typ!(&get_stmt_typ!(&node.props["else"]).props["body"]),
                    depth + 1,
                    &mut false,
                func_prefix
                );
                else_src += &format!("{}else:{body}", spacing.repeat(depth));
            }

            src = format!(
                "{}if({condition}):{body}{else_ifs_src}{else_src}",
                spacing.repeat(depth)
            );
            return src;
        }
        StmType::ForStmt => {
            let decl = self.py_transpiler(get_stmt_typ!(&node.props["decl"]), 0, &mut false,func_prefix);
            let condition =
                self.py_transpiler(get_stmt_typ!(&node.props["condition"]), 0, &mut false,func_prefix);
            let action =
                self.py_transpiler(get_stmt_typ!(&node.props["action"]), 0, &mut false,func_prefix);
            let body =
                self.py_transpiler(get_stmt_typ!(&node.props["body"]), depth, &mut false,func_prefix);

            src = format!("{}{decl}\n",spacing.repeat(depth));
            src += &format!("{}while({condition}):{body}",spacing.repeat(depth));
            src += &format!("{}{action}",spacing.repeat(depth + 1));
            return src;
        }
        StmType::WhileStmt => {
            let condition =
                self.py_transpiler(get_stmt_typ!(&node.props["condition"]), 0, &mut false,func_prefix);
            let body =
                self.py_transpiler(get_stmt_typ!(&node.props["body"]), depth + 1, &mut false,func_prefix);
            src = format!("while({condition}):{body}");
            return src;
        }

        StmType::FuncDeclaration => {
            let name = self.py_transpiler(get_stmt_typ!(&node.props["name"]), 0, &mut false,func_prefix);
            let mut args = "".to_string();
            if node.props.contains_key("arglist") {
                args += &self.py_transpiler(
                    get_stmt_typ!(&node.props["arglist"]),
                    depth + 1,
                    &mut false,
                func_prefix
                );
            }


            let mut body =
                self.py_transpiler(get_stmt_typ!(&node.props["body"]), depth + 1, &mut false,func_prefix);
            
            src = format!("def {func_prefix}{name}({args}):{body}");
            return src;
        }

        StmType::FuncCall => {
            let mut name =
                self.py_transpiler(get_stmt_typ!(&node.props["name"]), 0, &mut false,func_prefix);
            
            let mut is_sys = false;
            if (name.starts_with("$.")) {
                is_sys = true;
                name = self
                    .py_functions
                    .get(&name.split_off(2))
                    .unwrap()
                    .to_string();
            }
            let mut args: String = "".to_string();
            if node.props.contains_key("arglist") {
                args = self.py_transpiler(get_stmt_typ!(&node.props["arglist"]), 0, &mut false,func_prefix);
            }

            if !is_sys {
                src = format!("{}{func_prefix}{name}({args})", spacing.repeat(depth));
            } else {
                src = format!("{}{name}({args})", spacing.repeat(depth));
            }
            if (*add_semi) {
                src += ";";
            }


            return src;
        }

        StmType::ArgList => {
            let args = get_stmt_typ!(&node.props["list"], StmtValue::Arr);
            let mut not_required_args = vec![];

            for i in 0..args.len() {
                match args[i].typ {
                    StmType::VariableAssignment => {
                        not_required_args.push(&args[i]);
                    }
                    _ => {
                        src += &self.py_transpiler(&args[i], 0, &mut false,func_prefix);
                        if i != args.len() - 1 {
                            src += ",";
                        }
                    }
                }
            }
            if not_required_args.len() != 0 {
                if src.len() > 0 && src.chars().last().unwrap() != ',' {
                    src += ",";
                }
                for i in 0..not_required_args.len() {
                    src += &self.py_transpiler(&not_required_args[i], 0, &mut false,func_prefix);
                    if i != not_required_args.len() - 1 {
                        src += ",";
                    }
                }
            }

            return src;
        }

        StmType::Return => {
            let val = self.py_transpiler(get_stmt_typ!(&node.props["val"]), 0, &mut false,func_prefix);
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
                    let key = self.py_transpiler(&vals[i][0], 0, &mut false,func_prefix);
                    let val = self.py_transpiler(&vals[i][1], 0, &mut false,func_prefix);
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
            src = "\n".to_string();
            match &node.props["body"] {
                StmtValue::Arr(block) => {
                    if (block.len() == 0) {
                        src += &format!("{}pass",spacing.repeat(depth));
                    } else {
                        for stmt in block {
                            src += &self.py_transpiler(stmt, depth + 1, &mut true,func_prefix);
                            src += "\n";
                        }
                    }
                }
                _ => unreachable!(),
            }
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
