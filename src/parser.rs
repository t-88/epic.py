use std::{collections::HashMap, fmt::format};

use crate::lexer::*;

struct ParserError {
    pub msg: String,
}

#[derive(Debug)]
pub enum StmType {
    Ident,
    StringLiteral,
    IntLiteral,
    FloatLiteral,
    VariableAssignment,
}
#[derive(Debug)]
pub enum StmtValue {
    Stmt(Stmt),
    Str(String),
    Int(i64),
    Float(f64),
}

#[derive(Debug)]
pub struct Stmt {
    typ: StmType,
    props: HashMap<String, StmtValue>,
}

pub struct Parser {
    pub program: Vec<Stmt>,
    pub errs: Vec<ParserError>,
    lexer: Lexer,
    idx: u64,
}

impl Parser {
    fn push_err(self: &mut Self, msg: String) {
        self.errs.push(ParserError { msg: msg });
    }
    fn is_empty(self: &Self) -> bool {
        return self.idx >= self.lexer.tknz.len() as u64;
    }
    fn get(self: &Self) -> Tkn {
        return self.lexer.tknz[self.idx as usize].clone();
    }
    fn next(self: &mut Self) -> Tkn {
        let tkn: Tkn = self.get();
        self.idx += 1;
        tkn
    }
    fn expect(self: &mut Self, typ: TknType) {
        let tkn: Tkn = self.get();
        if (tkn.typ != typ) {
            self.push_err(format!(
                "{}:{} Unexpcted tkn, expected {:?} but got {:?}",
                tkn.line, tkn.col, typ, tkn.typ
            ));
            todo!();
        }
        self.next();
    }
}

impl Parser {
    pub fn new() -> Parser {
        return Parser {
            lexer: Lexer::new(),
            program: vec![],
            errs: vec![],
            idx: 0,
        };
    }

    pub fn parse(self: &mut Self, src: &String) {
        self.idx = 0;
        self.program = vec![];
        self.lexer.tokenize(&src);

        while !self.is_empty() {
            let stmt: Stmt = self.prase_variable_assignment();
            self.program.push(stmt);
        }
    }

    fn prase_variable_assignment(self: &mut Self) -> Stmt {
        match self.get().typ {
            TknType::Keyword(TknKeyword::Let) => {
                self.next();
                let ident: Stmt = self.parse_literal();

                self.expect(TknType::Equal);
                let val = self.parse_literal();

                let mut props: HashMap<String, StmtValue> = HashMap::new();
                props.insert(String::from("ident"), StmtValue::Stmt(ident));
                props.insert(String::from("val"), StmtValue::Stmt(val));

                self.expect(TknType::SemiCol);

                return Stmt {
                    typ: StmType::VariableAssignment,
                    props: props,
                }
            }
            _ =>  { 
                unreachable!() 
            }
        }
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
                println!("{}",self.errs.last().unwrap().msg);
                unreachable!();
            }
        };
    }
}
