use std::{collections::HashMap, fmt::format, vec};

use crate::lexer::*;

struct ParserError {
    pub msg: String,
}

#[derive(Debug)]
pub enum StmType {
    Program,
    EOP,
    Ident,
    StringLiteral,
    IntLiteral,
    FloatLiteral,
    VariableDeclaration,
    VariableAssignment,
    ArthExpr,
}
#[derive(Debug)]
pub enum StmtValue {
    Stmt(Stmt),
    Str(String),
    Int(i64),
    Float(f64),
    Arr(Vec<Stmt>),
}

#[derive(Debug)]
pub struct Stmt {
    typ: StmType,
    props: HashMap<String, StmtValue>,
}

pub struct Parser {
    pub program: Stmt,
    pub errs: Vec<ParserError>,
    lexer: Lexer,
    idx: u64,
}

impl Parser {
    pub fn new() -> Parser {
        let mut props: HashMap<String, StmtValue> = HashMap::new();
        props.insert("body".to_string(), StmtValue::Arr(vec![]));
        return Parser {
            lexer: Lexer::new(),
            program: Stmt {
                typ: StmType::Program,
                props: props,
            },
            errs: vec![],
            idx: 0,
        };
    }
    fn push_err(self: &mut Self, msg: String) {
        self.errs.push(ParserError { msg: msg });
    }
    fn is_empty(self: &Self, offset: u64) -> bool {
        return self.idx + offset >= self.lexer.tknz.len() as u64;
    }
    fn get(self: &Self, offset: u64) -> Tkn {
        return self.lexer.tknz[(self.idx + offset) as usize].clone();
    }
    fn next(self: &mut Self) -> Tkn {
        let tkn: Tkn = self.get(0);
        self.idx += 1;
        tkn
    }
    fn expect(self: &mut Self, typ: TknType) {
        let tkn: Tkn = self.get(0);
        if (tkn.typ != typ) {
            self.push_err(format!(
                "{}:{} Unexpcted tkn, expected {:?} but got {:?}",
                tkn.line, tkn.col, typ, tkn.typ
            ));
            println!("{}", self.errs.last().unwrap().msg);
            todo!();
        }
        self.next();
    }
}

impl Parser {
    pub fn parse(self: &mut Self, src: &String) {
        self.idx = 0;
        self.lexer.tokenize(&src);

        let mut body: Vec<Stmt> = vec![];
        while !self.is_empty(0) {
            let stmt: Stmt = self.prase_variable_declaration();
            body.push(stmt);
        }
        body.push(Stmt {
            typ: StmType::EOP,
            props: HashMap::new(),
        });
        self.program
            .props
            .insert("body".to_string(), StmtValue::Arr(body));
    }

    fn prase_variable_declaration(self: &mut Self) -> Stmt {
        if self.get(0).typ == TknType::Keyword(TknKeyword::Let) {
            self.next();
            let ident: Stmt = self.parse_literal();
            self.expect(TknType::Equal);
            let val = self.parse_arth_op_add(&mut false);
            self.expect(TknType::SemiCol);

            let mut props: HashMap<String, StmtValue> = HashMap::new();
            props.insert(String::from("ident"), StmtValue::Stmt(ident));
            props.insert(String::from("val"), StmtValue::Stmt(val));

            return Stmt {
                typ: StmType::VariableDeclaration,
                props: props,
            };
        };
        return self.parse_variable_assignment();
    }
    fn parse_variable_assignment(self: &mut Self) -> Stmt {
        if self.get(0).typ == TknType::Ident
            && !self.is_empty(1)
            && self.get(1).typ == TknType::Equal
        {
            let ident: Stmt = self.parse_literal();
            self.expect(TknType::Equal);
            let val = self.parse_arth_op_add(&mut false);
            self.expect(TknType::SemiCol);

            let mut props: HashMap<String, StmtValue> = HashMap::new();
            props.insert(String::from("ident"), StmtValue::Stmt(ident));
            props.insert(String::from("val"), StmtValue::Stmt(val));

            return Stmt {
                typ: StmType::VariableAssignment,
                props: props,
            };
        }

        return self.parse_arth_op_add(&mut true);
    }

    fn parse_arth_op_add(self: &mut Self, expect_semi: &mut bool) -> Stmt {
        let mut stmt: Stmt = self.parse_arth_op_mult(&mut false);

        if !self.is_empty(0)
            && (self.get(0).typ == TknType::Plus || self.get(0).typ == TknType::Minus)
        {
            while !self.is_empty(0)
                && (self.get(0).typ == TknType::Plus || self.get(0).typ == TknType::Minus)
            {
                let op: String = self.next().val;
                let rhs: Stmt = self.parse_arth_op_mult(&mut false);
                stmt =  Stmt {
                    typ: StmType::ArthExpr,
                    props: {
                        let mut props: HashMap<String, StmtValue> = HashMap::new();
                        props.insert(String::from("op"), StmtValue::Str(op));
                        props.insert(String::from("lhs"), StmtValue::Stmt(stmt));
                        props.insert(String::from("rhs"), StmtValue::Stmt(rhs));
                        props
                    },
                };
            }

        }

        if *expect_semi {
            *expect_semi = false;
            self.expect(TknType::SemiCol)
        }


        return stmt;
    }
    fn parse_arth_op_mult(self: &mut Self, expect_semi: &mut bool) -> Stmt {
        let mut stmt: Stmt = self.parse_literal();

        if !self.is_empty(0)
            && (self.get(0).typ == TknType::Mult || self.get(0).typ == TknType::Div)
        {
            while !self.is_empty(0)
                && (self.get(0).typ == TknType::Mult || self.get(0).typ == TknType::Div)
            {
                let op: String = self.next().val;
                let rhs : Stmt = self.parse_literal();
                
                stmt =  Stmt {
                    typ: StmType::ArthExpr,
                    props: {
                        let mut props: HashMap<String, StmtValue> = HashMap::new();
                        props.insert(String::from("op"), StmtValue::Str(op));
                        props.insert(String::from("lhs"), StmtValue::Stmt(stmt));
                        props.insert(String::from("rhs"), StmtValue::Stmt(rhs));
                        props
                    },
                };
            }
        }

        if *expect_semi {
            *expect_semi = false;
            self.expect(TknType::SemiCol)
        }        
        return stmt;
    }

    fn parse_literal(self: &mut Self) -> Stmt {
        let tkn: Tkn = self.next();
        let mut props: HashMap<String, StmtValue> = HashMap::new();

        return match tkn.typ {
            TknType::Ident => {
                props.insert(String::from("name"), StmtValue::Str(tkn.val));
                Stmt {
                    typ: StmType::Ident,
                    props: props,
                }
            }
            TknType::String => {
                props.insert(String::from("val"), StmtValue::Str(tkn.val));
                Stmt {
                    typ: StmType::StringLiteral,
                    props: props,
                }
            }
            TknType::Number => {
                if (tkn.val.contains(".")) {
                    props.insert(
                        String::from("val"),
                        StmtValue::Float(tkn.val.parse().unwrap()),
                    );
                    return Stmt {
                        typ: StmType::FloatLiteral,
                        props: props,
                    };
                }
                props.insert(
                    String::from("val"),
                    StmtValue::Int(tkn.val.parse().unwrap()),
                );
                return Stmt {
                    typ: StmType::IntLiteral,
                    props: props,
                };
            }
            _ => {
                self.push_err(format!(
                    "{}:{} Unexpected literal {:?} ",
                    tkn.line, tkn.col, tkn.typ
                ));
                println!("{}", self.errs.last().unwrap().msg);
                unreachable!();
            }
        };
    }
}

impl Parser {
    pub fn print_tree(self: &Self, node: &Stmt, depth: u64) {
        print!("{}", "   ".repeat(depth as usize));
        match node.typ {
            StmType::Program => {
                println!("program\n");
                if let StmtValue::Arr(body) = &node.props["body"] {
                    for node in body {
                        self.print_tree(&node, depth);
                    }
                } else {
                    unreachable!();
                }
            }
            StmType::EOP => {
                println!("\nend");
            }
            StmType::FloatLiteral => println!("float: {:?}", node.props["val"]),
            StmType::IntLiteral => println!("int: {:?}", node.props["val"]),
            StmType::StringLiteral => println!("string: {:?}", node.props["val"]),
            StmType::Ident => println!("ident: {:?}", node.props["name"]),
            StmType::VariableAssignment => {
                println!("variable assignment");
                match &node.props["ident"] {
                    StmtValue::Stmt(ident) => self.print_tree(&ident, depth + 1),
                    _ => unreachable!(),
                };
                match &node.props["val"] {
                    StmtValue::Stmt(val) => self.print_tree(&val, depth + 1),
                    _ => unreachable!(),
                };
            }
            StmType::VariableDeclaration => {
                println!("variable declaration");
                match &node.props["ident"] {
                    StmtValue::Stmt(ident) => self.print_tree(&ident, depth + 1),
                    _ => unreachable!(),
                };
                match &node.props["val"] {
                    StmtValue::Stmt(val) => self.print_tree(&val, depth + 1),
                    _ => unreachable!(),
                };
            }
            StmType::ArthExpr => {
                print!("arth expr");
                match &node.props["op"] {
                    StmtValue::Str(op) => println!(" {}", op),
                    _ => unreachable!(),
                };

                match &node.props["lhs"] {
                    StmtValue::Stmt(lhs) => self.print_tree(&lhs, depth + 1),
                    _ => unreachable!(),
                };
                match &node.props["rhs"] {
                    StmtValue::Stmt(rhs) => self.print_tree(&rhs, depth + 1),
                    _ => unreachable!(),
                };
            }
            _ => unreachable!(),
        }
    }
}
