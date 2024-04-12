use std::{collections::HashMap, fmt::format, vec};

use crate::lexer::*;

struct ParserError {
    pub msg: String,
}

#[derive(Debug)]
pub enum StmType {
    Program,
    StmtBlock,
    EOP,
    Ident,
    StringLiteral,
    IntLiteral,
    FloatLiteral,
    BooleanLiteral,
    VariableDeclaration,
    VariableAssignment,
    ArthExpr,
    BooleanExpr,
    IfStmt,
    ElseIfStmt,
    ElseStmt,
    ForStmt,
    WhileStmt,
    FuncDeclaration,
    ArgList,
    Return,
    Empty,
}
#[derive(Debug)]
pub enum StmtValue {
    Stmt(Stmt),
    Str(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    Arr(Vec<Stmt>),
}

#[derive(Debug)]
pub struct Stmt {
    typ: StmType,
    props: HashMap<String, StmtValue>,
}

impl Stmt {
    fn new() -> Self {
        Stmt {
            typ: StmType::Empty,
            props: HashMap::new(),
        }
    }
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
                "{}:{} Unexpcted tkn, expected {:?} but got {:?} with val '{}'",
                tkn.line, tkn.col, typ, tkn.typ, tkn.val
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
        while self.get(0).typ != TknType::EOF {
            let stmt: Stmt = self.parse_statement();
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

    fn parse_statement(self: &mut Self) -> Stmt {
        match self.get(0).typ {
            TknType::Keyword(TknKeyword::Let) => self.prase_variable_declaration(true),
            TknType::Keyword(TknKeyword::If) => self.parse_conditional(),
            TknType::Keyword(TknKeyword::For) => self.parse_for_stmt(),
            TknType::Keyword(TknKeyword::While) => self.parse_while_stmt(),
            TknType::Keyword(TknKeyword::Func) => self.parse_func_declaration_stmt(),
            TknType::Keyword(TknKeyword::Return) => self.parse_return_stmt(),
            _ => self.parse_variable_assignment(&mut true),
        }
    }
    fn parse_conditional(self: &mut Self) -> Stmt {
        self.expect(TknType::Keyword(TknKeyword::If));
        self.expect(TknType::OPara);
        let condtion: Stmt = self.parse_booean_op(&mut false);
        self.expect(TknType::CPara);

        let block: Stmt = self.parse_stmt_block();

        let mut else_ifs: Vec<Stmt> = vec![];
        while !self.is_empty(0) && self.get(0).typ == TknType::Keyword(TknKeyword::Elseif) {
            self.next();
            self.expect(TknType::OPara);
            let condtion: Stmt = self.parse_booean_op(&mut false);
            self.expect(TknType::CPara);
            let block: Stmt = self.parse_stmt_block();

            else_ifs.push(Stmt {
                typ: StmType::ElseIfStmt,
                props: {
                    let mut props: HashMap<String, StmtValue> = HashMap::new();
                    props.insert("condition".to_string(), StmtValue::Stmt(condtion));
                    props.insert("block".to_string(), StmtValue::Stmt(block));
                    props
                },
            });
        }

        let mut elses: Stmt = Stmt {
            typ: StmType::ElseIfStmt,
            props: {
                let mut props: HashMap<String, StmtValue> = HashMap::new();
                props
            },
        };
        if !self.is_empty(0) && self.get(0).typ == TknType::Keyword(TknKeyword::Else) {
            self.next();
            let block: Stmt = self.parse_stmt_block();
            elses
                .props
                .insert("block".to_string(), StmtValue::Stmt(block));
        }

        return Stmt {
            typ: StmType::IfStmt,
            props: {
                let mut props: HashMap<String, StmtValue> = HashMap::new();
                props.insert("condition".to_string(), StmtValue::Stmt(condtion));
                props.insert("block".to_string(), StmtValue::Stmt(block));
                props.insert("else".to_string(), StmtValue::Stmt(elses));
                props.insert("else_ifs".to_string(), StmtValue::Arr(else_ifs));
                props
            },
        };
    }
    fn parse_for_stmt(self: &mut Self) -> Stmt {
        self.expect(TknType::Keyword(TknKeyword::For));
        self.expect(TknType::OPara);

        let decl = self.prase_variable_declaration( false);
        self.expect(TknType::SemiCol);
        let condition = self.parse_booean_op( &mut false);
        self.expect(TknType::SemiCol);
        let action = self.parse_variable_assignment(&mut false);
        self.expect(TknType::CPara);

        let body = self.parse_stmt_block(); 

        return  Stmt {
            typ: StmType::ForStmt,
            props: {
                let mut props : HashMap<String,StmtValue> = HashMap::new();
                props.insert("decl".to_string(), StmtValue::Stmt(decl));
                props.insert("condition".to_string(), StmtValue::Stmt(condition));
                props.insert("action".to_string(), StmtValue::Stmt(action));
                props.insert("body".to_string(), StmtValue::Stmt(body));
                props
            }
        };
    }
    fn parse_while_stmt(self: &mut Self) -> Stmt {
        self.expect(TknType::Keyword(TknKeyword::While));
        self.expect(TknType::OPara);
        let condition = self.parse_booean_op( &mut false);
        self.expect(TknType::CPara);
        let body = self.parse_stmt_block(); 

        return  Stmt {
            typ: StmType::WhileStmt,
            props: {
                let mut props : HashMap<String,StmtValue> = HashMap::new();
                props.insert("condition".to_string(), StmtValue::Stmt(condition));
                props.insert("body".to_string(), StmtValue::Stmt(body));
                props
            }
        };
    }
    fn parse_func_declaration_stmt(self: &mut Self) -> Stmt {
        self.expect(TknType::Keyword(TknKeyword::Func));
        let name = self.parse_literal();
        self.expect(TknType::OPara);


        let mut arglist_exist = false;

        let mut arglist: Stmt = Stmt::new();
        if(self.get(0).typ != TknType::CPara) {
            arglist_exist = true;
            arglist = self.parse_arglist();
        }
        self.expect(TknType::CPara);
        let body = self.parse_stmt_block(); 

        return  Stmt {
            typ: StmType::FuncDeclaration,
            props: {
                let mut props : HashMap<String,StmtValue> = HashMap::new();
                props.insert("name".to_string(), StmtValue::Stmt(name));
                props.insert("body".to_string(), StmtValue::Stmt(body));
                if arglist_exist { props.insert("arglist".to_string(), StmtValue::Stmt(arglist)); };
                props
            }
        };        
    }
    fn parse_arglist(self: &mut Self) -> Stmt {

        let mut args : Vec<Stmt> = vec![];

        loop {
            args.push(
                self.parse_variable_assignment(&mut false),
            );
            if(self.get(0).typ == TknType::Comma) {
                self.next();
            } else {
                break;
            }
        }

        return  Stmt {
            typ: StmType::ArgList,
            props: {
                let mut props : HashMap<String,StmtValue> = HashMap::new();
                props.insert("list".to_string(), StmtValue::Arr(args));
                props
            }
        };  
    }

    fn parse_return_stmt(self: &mut Self) -> Stmt {
        self.expect(TknType::Keyword(TknKeyword::Return));



        let mut val_exists = false;
        let mut val = Stmt::new();
        if self.get(0).typ != TknType::SemiCol {
            val  = self.parse_arth_op_add(&mut false);
            val_exists=  true;
        }
        self.expect(TknType::SemiCol);
        return  Stmt {
            typ: StmType::Return,
            props: { 
                let mut props: HashMap<String, StmtValue> = HashMap::new();
                if val_exists { props.insert("val".to_string(), StmtValue::Stmt(val)); }
                props
            }
        };
    }

    fn parse_stmt_block(self: &mut Self) -> Stmt {
        self.expect(TknType::OCurl);

        let mut stmts: Vec<Stmt> = vec![];

        while !self.is_empty(0) && self.get(0).typ != TknType::CCurl {
            stmts.push(self.parse_statement());
        }

        self.expect(TknType::CCurl);

        return Stmt {
            typ: StmType::StmtBlock,
            props: {
                let mut props: HashMap<String, StmtValue> = HashMap::new();
                props.insert("body".to_string(), StmtValue::Arr(stmts));
                props
            },
        };

        self.expect(TknType::CCurl);
    }

    fn prase_variable_declaration(self: &mut Self,expect_semi: bool) -> Stmt {
        self.expect(TknType::Keyword(TknKeyword::Let));

        let ident: Stmt = self.parse_literal();
        self.expect(TknType::Equal);
        let val = self.parse_arth_op_add(&mut false);


        if(expect_semi) {
            self.expect(TknType::SemiCol);
        }

        return Stmt {
            typ: StmType::VariableDeclaration,
            props: {
                let mut props: HashMap<String, StmtValue> = HashMap::new();
                props.insert(String::from("ident"), StmtValue::Stmt(ident));
                props.insert(String::from("val"), StmtValue::Stmt(val));
                props
            },
        };
    }
    fn parse_variable_assignment(self: &mut Self,expect_semi: &mut bool) -> Stmt {
        if self.get(0).typ == TknType::Ident
            && !self.is_empty(1)
            && self.get(1).typ == TknType::Equal
        {
            let ident: Stmt = self.parse_literal();
            self.expect(TknType::Equal);
            let val = self.parse_arth_op_add(&mut false);

            if(*expect_semi) {
                *expect_semi = false;
                self.expect(TknType::SemiCol);
            }

            let mut props: HashMap<String, StmtValue> = HashMap::new();
            props.insert(String::from("ident"), StmtValue::Stmt(ident));
            props.insert(String::from("val"), StmtValue::Stmt(val));

            return Stmt {
                typ: StmType::VariableAssignment,
                props: props,
            };
        }

        return self.parse_arth_op_add(expect_semi);
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
                stmt = Stmt {
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
            self.expect(TknType::SemiCol);
        }

        return stmt;
    }
    fn parse_arth_op_mult(self: &mut Self, expect_semi: &mut bool) -> Stmt {
        let mut stmt: Stmt = self.parse_booean_op(&mut false);

        if !self.is_empty(0)
            && (self.get(0).typ == TknType::Mult || self.get(0).typ == TknType::Div)
        {
            while !self.is_empty(0)
                && (self.get(0).typ == TknType::Mult || self.get(0).typ == TknType::Div)
            {
                let op: String = self.next().val;
                let rhs: Stmt = self.parse_booean_op(&mut false);

                stmt = Stmt {
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

    fn parse_booean_op(self: &mut Self, expect_semi: &mut bool) -> Stmt {
        let mut stmt = self.parse_grouping();

        if !self.is_empty(0)
            && [
                TknType::AndBool,
                TknType::OrBool,
                TknType::Bigger,
                TknType::BiggerEq,
                TknType::Less,
                TknType::LessEq,
                TknType::Equalily,
                TknType::NotEqualily,
            ]
            .contains(&self.get(0).typ)
        {
            while !self.is_empty(0)
                && [
                    TknType::AndBool,
                    TknType::OrBool,
                    TknType::Bigger,
                    TknType::BiggerEq,
                    TknType::Less,
                    TknType::LessEq,
                    TknType::Equalily,
                    TknType::NotEqualily,
                ]
                .contains(&self.get(0).typ)
            {
                let op: String = self.next().val;
                let rhs: Stmt = self.parse_grouping();

                stmt = Stmt {
                    typ: StmType::BooleanExpr,
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
            self.expect(TknType::SemiCol);
        }
        return stmt;
    }

    fn parse_grouping(self: &mut Self) -> Stmt {
        if self.get(0).typ == TknType::OPara {
            self.next();
            let stmt: Stmt = self.parse_arth_op_add(&mut false);
            self.expect(TknType::CPara);
            return stmt;
        }

        return self.parse_literal();
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
            TknType::Keyword(TknKeyword::True) => {
                props.insert(String::from("val"), StmtValue::Bool(true));
                Stmt {
                    typ: StmType::BooleanLiteral,
                    props: props,
                }
            }
            TknType::Keyword(TknKeyword::False) => {
                props.insert(String::from("val"), StmtValue::Bool(false));
                Stmt {
                    typ: StmType::BooleanLiteral,
                    props: props,
                }
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
        let space: &str = "   ";
        let seprator: String = space.repeat(depth as usize);
        print!("{}", seprator);

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
            StmType::BooleanLiteral => println!("boolean: {:?}", node.props["val"]),
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
            StmType::BooleanExpr => {
                print!("boolean expr");
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

            StmType::IfStmt => {
                println!("if stmt");
                match &node.props["condition"] {
                    StmtValue::Stmt(condition) => {
                        println!("{}condition", space.repeat((depth + 1) as usize));
                        self.print_tree(condition, depth + 2);
                    }
                    _ => unreachable!(),
                };
                match &node.props["block"] {
                    StmtValue::Stmt(block) => {
                        self.print_tree(block, depth + 1);
                    }
                    _ => unreachable!(),
                }

                match &node.props["else_ifs"] {
                    StmtValue::Arr(else_ifs) => {
                        if (!else_ifs.is_empty()) {
                            for else_if in else_ifs {
                                println!("elseif stmt");
                                match &else_if.props["condition"] {
                                    StmtValue::Stmt(condition) => {
                                        println!("{}condition", space.repeat((depth + 1) as usize));
                                        self.print_tree(condition, depth + 2);
                                    }
                                    _ => unreachable!(),
                                };
                                match &else_if.props["block"] {
                                    StmtValue::Stmt(block) => {
                                        self.print_tree(block, depth + 1);
                                    }
                                    _ => unreachable!(),
                                }
                            }
                        }
                    }
                    _ => unreachable!(),
                };

                match &node.props["else"] {
                    StmtValue::Stmt(elses) => {
                        if (elses.props.contains_key("block")) {
                            println!("else stmt");
                            match &elses.props["block"] {
                                StmtValue::Stmt(block) => {
                                    self.print_tree(block, depth + 1);
                                }
                                _ => unreachable!(),
                            }
                        }
                    }
                    _ => unreachable!(),
                }
            }
            StmType::ForStmt => {
                println!("for stmt");
                match &node.props["decl"] {
                    StmtValue::Stmt(decl) => {
                        println!("{}decl", space.repeat((depth + 1) as usize));
                        self.print_tree(decl, depth + 2);
                    }
                    _ => unreachable!(),
                }



                match &node.props["condition"] {
                    StmtValue::Stmt(condition) => {
                        println!("{}condition", space.repeat((depth + 1) as usize));
                        self.print_tree(condition, depth + 2);
                    }
                    _ => unreachable!(),
                }
                
                match &node.props["action"] {
                    StmtValue::Stmt(action) => {
                        println!("{}action", space.repeat((depth + 1) as usize));
                        self.print_tree(action, depth + 2);
                    }
                    _ => unreachable!(),
                }

                match &node.props["body"] {
                    StmtValue::Stmt(body) => {
                        println!("{}body", space.repeat((depth + 1) as usize));
                        self.print_tree(body, depth + 2);
                    }
                    _ => unreachable!(),
                }

            }   
            StmType::WhileStmt => {
                println!("while stmt");

                match &node.props["condition"] {
                    StmtValue::Stmt(condition) => {
                        println!("{}condition", space.repeat((depth + 1) as usize));
                        self.print_tree(condition, depth + 2);
                    }
                    _ => unreachable!(),
                }

                match &node.props["body"] {
                    StmtValue::Stmt(body) => {
                        println!("{}body", space.repeat((depth + 1) as usize));
                        self.print_tree(body, depth + 2);
                    }
                    _ => unreachable!(),
                }

            }  

            StmType::FuncDeclaration => {
                println!("func decl");
                match &node.props["name"] {
                    StmtValue::Stmt(name) => {
                        println!("{}name", space.repeat((depth + 1) as usize));
                        self.print_tree(name, depth + 2);
                    }
                    _ => unreachable!(),
                }
                if node.props.contains_key("arglist") {
                    match &node.props["arglist"] {
                        StmtValue::Stmt(arglist) => {
                            self.print_tree(arglist, depth + 1);
                        }
                        _ => unreachable!(),
                    }                
                }
                match &node.props["body"] {
                    StmtValue::Stmt(body) => {
                        println!("{}body", space.repeat((depth + 1) as usize));
                        self.print_tree(body, depth + 2);
                    }
                    _ => unreachable!(),
                }                
            }

            StmType::ArgList => {
                match &node.props["list"] {
                    StmtValue::Arr(list) => {
                        println!("arglist");
                        for stmt in list {
                            self.print_tree(stmt, depth + 1);
                        }

                    }
                    _ => unreachable!(),
                }                
            }
            StmType::Return => {
                println!("return stmt");
                
                if node.props.contains_key("val") {
                    match &node.props["val"] {
                        StmtValue::Stmt(val) => {
                            self.print_tree(val, depth + 1);
                        }
                        _ => unreachable!(),
                    }                
                }
            }            
            StmType::StmtBlock => {
                println!("stmt block");
                match &node.props["body"] {
                    StmtValue::Arr(block) => {
                        if (block.len() == 0) {
                            println!("{}empty", space.repeat((depth + 1) as usize))
                        } else {
                            for stmt in block {
                                self.print_tree(stmt, depth + 1);
                            }
                        }
                    }
                    _ => unreachable!(),
                }
            }
            _ => unimplemented!(),
        }
    }
}
