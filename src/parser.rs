use std::{
    any::Any,
    collections::{btree_map::Values, HashMap},
    fmt::format,
    vec,
};

use crate::lexer::*;

#[derive(Debug, PartialEq, Eq)]
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
    DotExpr,
    BooleanExpr,
    IfStmt,
    ElseIfStmt,
    ElseStmt,
    ForStmt,
    WhileStmt,
    FuncDeclaration,
    FuncCall,
    SysIdent,
    ArgList,
    Return,
    Empty,
    Arr,
    HashMap,
    GroupExpr,
}
#[derive(Debug)]
pub enum StmtValue {
    Stmt(Stmt),
    Str(String),
    Ident(String),
    Int(i64),
    Float(f64),
    Bool(bool),
    Arr(Vec<Stmt>),
    HashMap(Vec<Vec<Stmt>>),
}

#[derive(Debug)]
pub struct Stmt {
    pub line: u64,
    pub typ: StmType,
    pub props: HashMap<String, StmtValue>,
}

impl Stmt {
    fn new() -> Self {
        Stmt {
            line: 0,
            typ: StmType::Empty,
            props: HashMap::new(),
        }
    }
    fn from(line: u64, typ: StmType, props: HashMap<String, StmtValue>) -> Self {
        Stmt {
            line: line,
            typ: typ,
            props: props,
        }
    }
}

#[macro_export]
macro_rules! get_stmt_typ {
    ($value: expr , $typ: path) => {
        match $value {
            $typ(x) => x,
            _ => unreachable!(),
        }
    };
    ($value: expr) => {
        match $value {
            StmtValue::Stmt(x) => x,
            _ => unreachable!(),
        }
    };
}
macro_rules! panic_check {
    ($value: expr,$out: expr) => {
        match $value {
            Ok(x) => {
                $out = x;
            }
            Err(err) => return Err(err),
        }
    };
    ($value: expr) => {
        match $value {
            Ok(x) => {}
            Err(err) => return Err(err),
        }
    };
}

macro_rules! expect_expr {
    ($self: ident,$value: expr,$out: expr) => {
        match $value {
            Ok(x) => {
                $out = x;
            }
            Err(err) => return Err($self.err_expected_expr()),
        }
    };
}

macro_rules! expect_semi_colon {
    ($self: ident,$expect_semi: ident) => {
        if (*$expect_semi) {
            *$expect_semi = false;
            panic_check!($self.expect(TknType::SemiCol));
            $self.next();
        }
    };
}
#[derive(Debug)]
pub struct SyntaxError {
    pub msg: String,
}

pub struct Parser {
    pub program: Stmt,
    pub errs: Vec<SyntaxError>,
    pub src: Vec<String>,
    lexer: Lexer,
    idx: u64,
}

impl Parser {
    fn err_expected_char(self: &mut Self, chr: char) -> SyntaxError {
        return SyntaxError {
            msg: format!(
                "line {}: missing '{}'\n -> {}",
                self.get(-1).line,
                chr,
                self.src[(self.get(-1).line - 1) as usize]
            ),
        };
    }
    fn err_expected(self: &mut Self, exp_typ: TknType) -> SyntaxError {
        if self.get(0).typ == TknType::EOF {
            return SyntaxError {
                msg: format!(
                    "line {}: unexpected end of file\n -> {}",
                    self.get(-1).line,
                    self.src[self.src.len() - 1]
                ),
            };
        } else {
            return SyntaxError {
                msg: format!(
                    "line {}: expected {:?} but found {:?} with value {:?}\n -> {}",
                    self.get(-1).line,
                    exp_typ,
                    self.get(0).typ,
                    self.get(0).val,
                    self.src[(self.get(-1).line - 1) as usize]
                ),
            };
        }
    }

    fn err_expected_expr(self: &mut Self) -> SyntaxError {
        return SyntaxError {
            msg: format!(
                "line {}: expected an experssion\n -> {}",
                self.get(-1).line,
                self.src[(self.get(-1).line - 1) as usize]
            ),
        };
    }

    fn expect(self: &mut Self, typ: TknType) -> Result<Tkn, SyntaxError> {
        let tkn: Tkn = self.get(0);
        if (tkn.typ != typ) {
            match typ {
                TknType::SemiCol => return Err(self.err_expected_char(';')),
                TknType::CCurl => return Err(self.err_expected_char('}')),
                TknType::OCurl => return Err(self.err_expected_char('{')),
                TknType::OSqr => return Err(self.err_expected_char('[')),
                TknType::CSqr => return Err(self.err_expected_char(']')),
                _ => return Err(self.err_expected(typ)),
            }
        }

        let val = self.get(0);

        return Ok(val);
    }
}

impl Parser {
    pub fn new() -> Parser {
        let mut props: HashMap<String, StmtValue> = HashMap::new();
        props.insert("body".to_string(), StmtValue::Arr(vec![]));
        return Parser {
            lexer: Lexer::new(),
            program: Stmt {
                line: 0,
                typ: StmType::Program,
                props: props,
            },
            errs: vec![],
            idx: 0,
            src: vec![],
        };
    }
    fn is_empty(self: &Self, offset: u64) -> bool {
        return self.idx + offset >= self.lexer.tknz.len() as u64;
    }
    fn get(self: &Self, offset: i64) -> Tkn {
        return self.lexer.tknz[(self.idx as i64 + offset) as usize].clone();
    }
    fn next(self: &mut Self) -> Tkn {
        let tkn: Tkn = self.get(0);
        self.idx += 1;
        tkn
    }

    fn panic_mode(self: &mut Self, err: SyntaxError) {
        self.errs.push(err);

        let keywords = [
            TknType::Keyword(TknKeyword::Return),
            TknType::Keyword(TknKeyword::Elseif),
            TknType::Keyword(TknKeyword::Else),
            TknType::Keyword(TknKeyword::For),
            TknType::Keyword(TknKeyword::Let),
            TknType::Keyword(TknKeyword::If),
        ];

        while ![TknType::EOF, TknType::CCurl, TknType::SemiCol].contains(&self.get(0).typ)
            && !keywords.contains(&self.get(0).typ)
        {
            self.next();
        }

        if self.get(0).typ != TknType::EOF && !keywords.contains(&self.get(0).typ) {
            self.next();
        }
    }
}

impl Parser {
    pub fn parse(self: &mut Self, src: &String) {
        self.idx = 0;
        self.lexer.tokenize(&src);

        self.src = src
            .split("\n")
            .collect::<Vec<&str>>()
            .iter()
            .map(|s| s.to_string())
            .collect();

        let mut body: Vec<Stmt> = vec![];
        while self.get(0).typ != TknType::EOF {
            let out = self.parse_statement();
            match out {
                Ok(stmt) => body.push(stmt),
                Err(e) => self.panic_mode(e),
            }
        }

        body.push(Stmt {
            line: self.get(0).line,
            typ: StmType::EOP,
            props: HashMap::new(),
        });

        self.program
            .props
            .insert("body".to_string(), StmtValue::Arr(body));
    }

    fn parse_statement(self: &mut Self) -> Result<Stmt, SyntaxError> {
        match self.get(0).typ {
            TknType::Keyword(TknKeyword::Let) => self.parse_variable_declaration(true),
            TknType::Keyword(TknKeyword::If) => self.parse_conditional(),
            TknType::Keyword(TknKeyword::For) => self.parse_for_stmt(),
            TknType::Keyword(TknKeyword::While) => self.parse_while_stmt(),
            TknType::Keyword(TknKeyword::Func) => self.parse_func_declaration_stmt(),
            TknType::Keyword(TknKeyword::Return) => self.parse_return_stmt(),
            _ => self.parse_fun_call(&mut true),
        }
    }
    fn parse_conditional(self: &mut Self) -> Result<Stmt, SyntaxError> {
        let mut condtion: Stmt;
        let mut block: Stmt;
        let mut else_ifs: Vec<Stmt> = vec![];
        let mut elses: Stmt = Stmt::new();

        panic_check!(self.expect(TknType::Keyword(TknKeyword::If)));
        self.next();
        panic_check!(self.expect(TknType::OPara));
        self.next();
        panic_check!(self.parse_fun_call(&mut false), condtion);
        panic_check!(self.expect(TknType::CPara));
        self.next();
        panic_check!(self.parse_stmt_block(), block);

        while !self.is_empty(0) && self.get(0).typ == TknType::Keyword(TknKeyword::Elseif) {
            let condtion: Stmt;
            let block: Stmt;

            self.next();
            panic_check!(self.expect(TknType::OPara));
            self.next();
            panic_check!(self.parse_boolean_op(&mut false), condtion);
            panic_check!(self.expect(TknType::CPara));
            self.next();
            panic_check!(self.parse_stmt_block(), block);

            else_ifs.push(Stmt::from(self.get(-1).line, StmType::ElseIfStmt, {
                let mut props: HashMap<String, StmtValue> = HashMap::new();
                props.insert("condition".to_string(), StmtValue::Stmt(condtion));
                props.insert("body".to_string(), StmtValue::Stmt(block));
                props
            }));
        }

        let mut elses_exist = false;
        if !self.is_empty(0) && self.get(0).typ == TknType::Keyword(TknKeyword::Else) {
            elses_exist = true;
            let block: Stmt;
            self.next();
            panic_check!(self.parse_stmt_block(), block);
            elses
                .props
                .insert("body".to_string(), StmtValue::Stmt(block));
        }

        return Ok(Stmt::from(self.get(-1).line, StmType::IfStmt, {
            let mut props: HashMap<String, StmtValue> = HashMap::new();
            props.insert("condition".to_string(), StmtValue::Stmt(condtion));
            props.insert("body".to_string(), StmtValue::Stmt(block));
            if elses_exist {
                props.insert("else".to_string(), StmtValue::Stmt(elses));
            }
            props.insert("else_ifs".to_string(), StmtValue::Arr(else_ifs));
            props
        }));
    }
    fn parse_for_stmt(self: &mut Self) -> Result<Stmt, SyntaxError> {
        let mut decl: Stmt;
        let mut condition: Stmt;
        let mut action: Stmt;
        let mut body: Stmt;

        panic_check!(self.expect(TknType::Keyword(TknKeyword::For)));
        self.next();
        panic_check!(self.expect(TknType::OPara));
        self.next();
        panic_check!(self.parse_variable_declaration(false), decl);
        panic_check!(self.expect(TknType::SemiCol));
        self.next();
        panic_check!(self.parse_boolean_op(&mut false), condition);
        panic_check!(self.expect(TknType::SemiCol));
        self.next();
        panic_check!(self.parse_variable_assignment(&mut false), action);
        panic_check!(self.expect(TknType::CPara));
        self.next();
        panic_check!(self.parse_stmt_block(), body);

        return Ok(Stmt::from(self.get(-1).line, StmType::ForStmt, {
            let mut props: HashMap<String, StmtValue> = HashMap::new();
            props.insert("decl".to_string(), StmtValue::Stmt(decl));
            props.insert("condition".to_string(), StmtValue::Stmt(condition));
            props.insert("action".to_string(), StmtValue::Stmt(action));
            props.insert("body".to_string(), StmtValue::Stmt(body));
            props
        }));
    }
    fn parse_while_stmt(self: &mut Self) -> Result<Stmt, SyntaxError> {
        let mut condition: Stmt;
        let mut body: Stmt;

        panic_check!(self.expect(TknType::Keyword(TknKeyword::While)));
        self.next();
        panic_check!(self.expect(TknType::OPara));
        self.next();
        panic_check!(self.parse_boolean_op(&mut false), condition);
        panic_check!(self.expect(TknType::CPara));
        self.next();
        panic_check!(self.parse_stmt_block(), body);

        return Ok(Stmt::from(self.get(-1).line, StmType::WhileStmt, {
            let mut props: HashMap<String, StmtValue> = HashMap::new();
            props.insert("condition".to_string(), StmtValue::Stmt(condition));
            props.insert("body".to_string(), StmtValue::Stmt(body));
            props
        }));
    }
    fn parse_func_declaration_stmt(self: &mut Self) -> Result<Stmt, SyntaxError> {
        let mut name: Stmt;
        let mut arglist: Stmt = Stmt::new();
        let mut arglist_exist = false;
        let mut body: Stmt;

        panic_check!(self.expect(TknType::Keyword(TknKeyword::Func)));
        self.next();
        panic_check!(self.parse_literal(), name);
        panic_check!(self.expect(TknType::OPara));
        self.next();
        if (self.get(0).typ != TknType::CPara) {
            arglist_exist = true;
            panic_check!(self.parse_arglist(), arglist);
        }
        panic_check!(self.expect(TknType::CPara));
        self.next();
        panic_check!(self.parse_stmt_block(), body);

        return Ok(Stmt::from(self.get(-1).line, StmType::FuncDeclaration, {
            let mut props: HashMap<String, StmtValue> = HashMap::new();
            props.insert("name".to_string(), StmtValue::Stmt(name));
            props.insert("body".to_string(), StmtValue::Stmt(body));
            if arglist_exist {
                props.insert("arglist".to_string(), StmtValue::Stmt(arglist));
            };
            props
        }));
    }
    fn parse_arglist(self: &mut Self) -> Result<Stmt, SyntaxError> {
        let mut args: Vec<Stmt> = vec![];

        loop {
            let arg: Stmt;
            panic_check!(self.parse_variable_assignment(&mut false), arg);
            args.push(arg);
            if (self.get(0).typ == TknType::Comma) {
                self.next();
            } else {
                break;
            }
        }

        return Ok(Stmt::from(self.get(-1).line, StmType::ArgList, {
            let mut props: HashMap<String, StmtValue> = HashMap::new();
            props.insert("list".to_string(), StmtValue::Arr(args));
            props
        }));
    }
    fn parse_return_stmt(self: &mut Self) -> Result<Stmt, SyntaxError> {
        let mut val = Stmt::new();
        let mut val_exists = false;

        panic_check!(self.expect(TknType::Keyword(TknKeyword::Return)));
        self.next();
        if self.get(0).typ != TknType::SemiCol {
            val_exists = true;
            expect_expr!(self, self.parse_arth_op_add(&mut false), val);
        }
        panic_check!(self.expect(TknType::SemiCol));
        self.next();

        return Ok(Stmt::from(self.get(-1).line, StmType::Return, {
            let mut props: HashMap<String, StmtValue> = HashMap::new();
            if val_exists {
                props.insert("val".to_string(), StmtValue::Stmt(val));
            }
            props
        }));
    }

    fn parse_stmt_block(self: &mut Self) -> Result<Stmt, SyntaxError> {
        let mut stmts: Vec<Stmt> = vec![];
        let mut stmt: Stmt;

        panic_check!(self.expect(TknType::OCurl));
        self.next();
        while !self.is_empty(0) && self.get(0).typ != TknType::CCurl {
            panic_check!(self.parse_statement(), stmt);
            stmts.push(stmt);
        }

        panic_check!(self.expect(TknType::CCurl));
        self.next();

        return Ok(Stmt::from(self.get(-1).line, StmType::StmtBlock, {
            let mut props: HashMap<String, StmtValue> = HashMap::new();
            props.insert("body".to_string(), StmtValue::Arr(stmts));
            props
        }));

        panic_check!(self.expect(TknType::CCurl));
        self.next();
    }
    fn parse_variable_declaration(self: &mut Self, expect_semi: bool) -> Result<Stmt, SyntaxError> {
        let mut ident: Stmt;
        let mut val: Stmt;

        panic_check!(self.expect(TknType::Keyword(TknKeyword::Let)));
        self.next();
        panic_check!(self.expect(TknType::Ident));
        panic_check!(self.parse_literal(), ident);
        panic_check!(self.expect(TknType::Equal));
        self.next();
        panic_check!(self.parse_fun_call(&mut false), val);

        if (expect_semi) {
            panic_check!(self.expect(TknType::SemiCol));
            self.next();
        }
        return Ok(Stmt::from(
            self.get(0).line,
            StmType::VariableDeclaration,
            {
                let mut props: HashMap<String, StmtValue> = HashMap::new();
                props.insert(String::from("ident"), StmtValue::Stmt(ident));
                props.insert(String::from("val"), StmtValue::Stmt(val));
                props
            },
        ));
    }
    fn parse_fun_call(self: &mut Self, expect_semi: &mut bool) -> Result<Stmt, SyntaxError> {
        if self.get(0).typ == TknType::Ident
            && !self.is_empty(0)
            && self.get(1).typ == TknType::OPara
        {
            let mut name: Stmt;
            let mut arglist_exist = false;
            let mut arglist = Stmt::new();

            panic_check!(self.parse_literal(), name);
            panic_check!(self.expect(TknType::OPara));
            self.next();
            if self.get(0).typ != TknType::CPara {
                arglist_exist = true;
                panic_check!(self.parse_arglist(), arglist);
            }
            panic_check!(self.expect(TknType::CPara));
            self.next();

            if *expect_semi {
                *expect_semi = false;
                panic_check!(self.expect(TknType::SemiCol));
                self.next();
            }

            return Ok(Stmt::from(self.get(-1).line, StmType::FuncCall, {
                let mut props: HashMap<String, StmtValue> = HashMap::new();
                props.insert("name".to_string(), StmtValue::Stmt(name));
                if arglist_exist {
                    props.insert("arglist".to_string(), StmtValue::Stmt(arglist));
                }
                props
            }));
        }
        return self.parse_variable_assignment(expect_semi);
    }
    fn parse_variable_assignment(
        self: &mut Self,
        expect_semi: &mut bool,
    ) -> Result<Stmt, SyntaxError> {
        let mut stmt;
        panic_check!(self.parse_arth_op_add(&mut false), stmt);

        if (!self.is_empty(0) && self.get(0).typ == TknType::Equal) {
            self.next();

            let mut val: Stmt;
            panic_check!(self.parse_fun_call(&mut false), val);

            stmt = Stmt::from(self.get(-1).line, StmType::VariableAssignment, {
                let mut props: HashMap<String, StmtValue> = HashMap::new();
                props.insert(String::from("ident"), StmtValue::Stmt(stmt));
                props.insert(String::from("val"), StmtValue::Stmt(val));
                props
            });
        }

        expect_semi_colon!(self, expect_semi);
        return Ok(stmt);
    }
    fn parse_arth_op_add(self: &mut Self, expect_semi: &mut bool) -> Result<Stmt, SyntaxError> {
        let mut stmt: Stmt;

        expect_expr!(self, self.parse_arth_op_mult(&mut false), stmt);

        if !self.is_empty(0)
            && (self.get(0).typ == TknType::Plus || self.get(0).typ == TknType::Minus)
        {
            while !self.is_empty(0)
                && (self.get(0).typ == TknType::Plus || self.get(0).typ == TknType::Minus)
            {
                let rhs: Stmt;
                let op: String = self.next().val;

                expect_expr!(self, self.parse_arth_op_mult(&mut false), rhs);

                stmt = Stmt::from(self.get(-1).line, StmType::ArthExpr, {
                    let mut props: HashMap<String, StmtValue> = HashMap::new();
                    props.insert(String::from("op"), StmtValue::Str(op));
                    props.insert(String::from("lhs"), StmtValue::Stmt(stmt));
                    props.insert(String::from("rhs"), StmtValue::Stmt(rhs));
                    props
                });
            }
        }

        if *expect_semi {
            *expect_semi = false;
            panic_check!(self.expect(TknType::SemiCol));
            self.next();
        }

        return Ok(stmt);
    }
    fn parse_arth_op_mult(self: &mut Self, expect_semi: &mut bool) -> Result<Stmt, SyntaxError> {
        let mut stmt: Stmt;

        panic_check!(self.parse_boolean_op(&mut false), stmt);

        if !self.is_empty(0)
            && (self.get(0).typ == TknType::Mult || self.get(0).typ == TknType::Div)
        {
            while !self.is_empty(0)
                && (self.get(0).typ == TknType::Mult || self.get(0).typ == TknType::Div)
            {
                let op: String = self.next().val;
                let rhs: Stmt;
                panic_check!(self.parse_boolean_op(&mut false), rhs);

                stmt = Stmt::from(self.get(-1).line, StmType::ArthExpr, {
                    let mut props: HashMap<String, StmtValue> = HashMap::new();
                    props.insert(String::from("op"), StmtValue::Str(op));
                    props.insert(String::from("lhs"), StmtValue::Stmt(stmt));
                    props.insert(String::from("rhs"), StmtValue::Stmt(rhs));
                    props
                });
            }
        }

        if *expect_semi {
            *expect_semi = false;
            panic_check!(self.expect(TknType::SemiCol));
            self.next();
        }
        return Ok(stmt);
    }
    fn parse_boolean_op(self: &mut Self, expect_semi: &mut bool) -> Result<Stmt, SyntaxError> {
        let mut stmt: Stmt;
        panic_check!(self.parse_dot_op(expect_semi), stmt);

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
                let rhs: Stmt;
                panic_check!(self.parse_dot_op(expect_semi), rhs);

                stmt = Stmt::from(self.get(-1).line, StmType::BooleanExpr, {
                    let mut props: HashMap<String, StmtValue> = HashMap::new();
                    props.insert(String::from("op"), StmtValue::Str(op));
                    props.insert(String::from("lhs"), StmtValue::Stmt(stmt));
                    props.insert(String::from("rhs"), StmtValue::Stmt(rhs));
                    props
                });
            }
        }

        if *expect_semi {
            *expect_semi = false;
            panic_check!(self.expect(TknType::SemiCol));
            self.next();
        }
        return Ok(stmt);
    }

    fn parse_dot_op(self: &mut Self, expect_semi: &mut bool) -> Result<Stmt, SyntaxError> {
        let mut stmt: Stmt;
        panic_check!(self.parse_grouping(), stmt);
        if (self.get(0).typ == TknType::Dot) {
            while !self.is_empty(0) && self.get(0).typ == TknType::Dot {
                self.next();

                let rhs: Stmt;
                panic_check!(self.parse_fun_call(expect_semi), rhs);

                stmt = Stmt::from(self.get(-1).line, StmType::DotExpr, {
                    let mut props: HashMap<String, StmtValue> = HashMap::new();
                    props.insert(String::from("lhs"), StmtValue::Stmt(stmt));
                    props.insert(String::from("rhs"), StmtValue::Stmt(rhs));
                    props
                });
            }
        }

        if *expect_semi {
            *expect_semi = false;
            panic_check!(self.expect(TknType::SemiCol));
            self.next();
        }

        return Ok(stmt);
    }

    fn parse_grouping(self: &mut Self) -> Result<Stmt, SyntaxError> {
        if self.get(0).typ == TknType::OPara {
            let mut  stmt: Stmt;

            self.next();
            panic_check!(self.parse_arth_op_add(&mut false), stmt);
            panic_check!(self.expect(TknType::CPara));
            self.next();

            stmt = Stmt::from(self.get(-1).line, StmType::GroupExpr, {
                let mut props: HashMap<String, StmtValue> = HashMap::new();
                props.insert(String::from("val"), StmtValue::Stmt(stmt));
                props
            });
            return  Ok(stmt);
        }

        return self.parse_array_notation();
    }
    fn parse_array_notation(self: &mut Self) -> Result<Stmt, SyntaxError> {
        if self.get(0).typ == TknType::OSqr {
            let mut vals: Vec<Stmt> = vec![];
            let mut val: Stmt;

            self.next();
            if self.get(0).typ != TknType::CSqr {
                loop {
                    panic_check!(self.parse_arth_op_add(&mut false), val);
                    vals.push(val);
                    if self.get(0).typ == TknType::Comma {
                        self.next();
                    } else {
                        break;
                    }
                }
            }
            panic_check!(self.expect(TknType::CSqr));
            self.next();
            return Ok(Stmt::from(self.get(-1).line, StmType::Arr, {
                let mut props: HashMap<String, StmtValue> = HashMap::new();
                props.insert("vals".to_string(), StmtValue::Arr(vals));
                props
            }));
        }

        return self.parse_hashtable_notation();
    }
    fn parse_hashtable_notation(self: &mut Self) -> Result<Stmt, SyntaxError> {
        if self.get(0).typ == TknType::OCurl {
            let mut vals: Vec<Vec<Stmt>> = vec![];
            let mut elem: Stmt;

            self.next();
            if self.get(0).typ != TknType::CCurl {
                loop {
                    let mut elems: Vec<Stmt> = vec![];
                    panic_check!(self.parse_arth_op_add(&mut false), elem);
                    elems.push(elem);
                    panic_check!(self.expect(TknType::Colon));
                    self.next();
                    panic_check!(self.parse_arth_op_add(&mut false), elem);
                    elems.push(elem);

                    vals.push(elems);
                    if self.get(0).typ == TknType::Comma {
                        self.next();
                    } else {
                        break;
                    }
                }
            }
            panic_check!(self.expect(TknType::CCurl));
            self.next();

            return Ok(Stmt::from(self.get(-1).line, StmType::HashMap, {
                let mut props: HashMap<String, StmtValue> = HashMap::new();
                props.insert("vals".to_string(), StmtValue::HashMap(vals));
                props
            }));
        }

        return self.parse_literal();
    }
    fn parse_literal(self: &mut Self) -> Result<Stmt, SyntaxError> {
        let tkn: Tkn = self.next();
        let mut props: HashMap<String, StmtValue> = HashMap::new();

        return match tkn.typ {
            TknType::Ident => {
                props.insert(String::from("name"), StmtValue::Str(tkn.val));
                Ok(Stmt::from(self.get(-1).line, StmType::Ident, props))
            }
            TknType::String => {
                props.insert(String::from("val"), StmtValue::Str(tkn.val));
                Ok(Stmt::from(self.get(-1).line, StmType::StringLiteral, props))
            }
            TknType::Number => {
                if (tkn.val.contains(".")) {
                    props.insert(
                        String::from("val"),
                        StmtValue::Float(tkn.val.parse().unwrap()),
                    );
                    return Ok(Stmt::from(self.get(-1).line, StmType::FloatLiteral, props));
                }
                props.insert(
                    String::from("val"),
                    StmtValue::Int(tkn.val.parse().unwrap()),
                );
                Ok(Stmt::from(self.get(-1).line, StmType::IntLiteral, props))
            }
            TknType::Keyword(TknKeyword::True) => {
                props.insert(String::from("val"), StmtValue::Bool(true));
                Ok(Stmt::from(self.get(-1).line, StmType::BooleanLiteral, props))
            }
            TknType::Keyword(TknKeyword::False) => {
                props.insert(String::from("val"), StmtValue::Bool(false));
                Ok(Stmt::from(self.get(-1).line, StmType::BooleanLiteral, props))
            }
            TknType::Sys => Ok(Stmt::from(
                self.get(0).line,
                StmType::SysIdent,
                HashMap::new(),
            )),
            _ => {
                return Err(SyntaxError {
                    msg: format!("{} Unexpected literal {:?} ", tkn.line, tkn.typ),
                });
            }
        };
    }
}

impl Parser {
    pub fn print_tree(self: &Self, node: &Stmt, depth: u64) {
        let space: &str = "   ";
        let seprator: String = space.repeat(depth as usize);

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
            StmType::EOP => println!("\nend"),

            StmType::FloatLiteral => println!(
                "{}float: {}",
                seprator.clone(),
                get_stmt_typ!(node.props["val"], StmtValue::Float)
            ),
            StmType::IntLiteral => {
                println!(
                    "{}int: {}",
                    seprator.clone(),
                    get_stmt_typ!(node.props["val"], StmtValue::Int)
                )
            }
            StmType::StringLiteral => {
                println!(
                    "{}str: {}",
                    seprator.clone(),
                    get_stmt_typ!(&node.props["val"], StmtValue::Str)
                )
            }
            StmType::BooleanLiteral => println!(
                "{}bool: {}",
                seprator.clone(),
                get_stmt_typ!(&node.props["val"], StmtValue::Bool)
            ),
            StmType::Ident => println!(
                "{}ident: {}",
                seprator.clone(),
                get_stmt_typ!(&node.props["name"], StmtValue::Str)
            ),
            StmType::SysIdent => println!("{}sys ident: $", seprator.clone(),),
            StmType::VariableAssignment => {
                println!("{}variable assignment", seprator);
                self.print_tree(get_stmt_typ!(&node.props["ident"]), depth + 1);
                self.print_tree(get_stmt_typ!(&node.props["val"]), depth + 1);
            }
            StmType::VariableDeclaration => {
                println!("{}variable declaration", seprator);
                self.print_tree(get_stmt_typ!(&node.props["ident"]), depth + 1);
                self.print_tree(get_stmt_typ!(&node.props["val"]), depth + 1);
            }
            StmType::ArthExpr => {
                print!("{}arth expr", seprator);
                println!(" {}", get_stmt_typ!(&node.props["op"], StmtValue::Str));
                self.print_tree(get_stmt_typ!(&node.props["lhs"]), depth + 1);
                self.print_tree(get_stmt_typ!(&node.props["rhs"]), depth + 1);
            }
            StmType::BooleanExpr => {
                print!("{}boolean expr", seprator);
                println!(" {}", get_stmt_typ!(&node.props["op"], StmtValue::Str));
                self.print_tree(get_stmt_typ!(&node.props["lhs"]), depth + 1);
                self.print_tree(get_stmt_typ!(&node.props["rhs"]), depth + 1);
            }
            StmType::DotExpr => {
                println!("{}dot expr", seprator);
                self.print_tree(get_stmt_typ!(&node.props["lhs"]), depth + 1);
                self.print_tree(get_stmt_typ!(&node.props["rhs"]), depth + 1);
            }
            StmType::IfStmt => {
                println!("{}if stmt", seprator.clone());

                // if
                println!("{}condition", seprator.clone());
                self.print_tree(get_stmt_typ!(&node.props["condition"]), depth + 2);
                self.print_tree(get_stmt_typ!(&node.props["body"]), depth + 1);

                // else if
                let else_ifs = get_stmt_typ!(&node.props["else_ifs"], StmtValue::Arr);
                if (!else_ifs.is_empty()) {
                    for else_if in else_ifs {
                        println!("{}elseif stmt", seprator.clone());
                        println!("{}condition", seprator.clone() + space);
                        self.print_tree(get_stmt_typ!(&else_if.props["condition"]), depth + 2);
                        self.print_tree(get_stmt_typ!(&else_if.props["body"]), depth + 1);
                    }
                }

                // else
                if (node.props.contains_key("else")) {
                    println!("{}else stmt", seprator.clone());
                    self.print_tree(
                        get_stmt_typ!(&get_stmt_typ!(&node.props["else"]).props["body"]),
                        depth + 1,
                    );
                }
            }
            StmType::ForStmt => {
                println!("{}for stmt", seprator.clone());

                println!("{}decl", seprator.clone() + space);
                self.print_tree(get_stmt_typ!(&node.props["decl"]), depth + 2);

                println!("{}condition", seprator.clone() + space);
                self.print_tree(get_stmt_typ!(&node.props["condition"]), depth + 2);

                println!("{}action", seprator.clone() + space);
                self.print_tree(get_stmt_typ!(&node.props["action"]), depth + 2);

                println!("{}body", seprator.clone() + space);
                self.print_tree(get_stmt_typ!(&node.props["body"]), depth + 2);
            }
            StmType::WhileStmt => {
                println!("{}while stmt", seprator.clone());
                println!("{}condition", seprator.clone() + space);
                self.print_tree(get_stmt_typ!(&node.props["condition"]), depth + 2);
                println!("{}body", seprator.clone() + space);
                self.print_tree(get_stmt_typ!(&node.props["body"]), depth + 2);
            }
            StmType::FuncDeclaration => {
                println!("{}func decl", seprator.clone());

                println!("{}name", seprator.clone() + space);
                self.print_tree(get_stmt_typ!(&node.props["name"]), depth + 2);

                if node.props.contains_key("arglist") {
                    println!("{}arglist", seprator.clone());
                    self.print_tree(get_stmt_typ!(&node.props["arglist"]), depth + 1);
                }
                println!("{}body", seprator.clone() + space);
                self.print_tree(get_stmt_typ!(&node.props["body"]), depth + 2);
            }
            StmType::FuncCall => {
                println!("{}func call", seprator.clone());

                println!("{}name", seprator.clone() + space);
                self.print_tree(get_stmt_typ!(&node.props["name"]), depth + 2);

                if node.props.contains_key("arglist") {
                    println!("{}arglist", seprator.clone());
                    self.print_tree(get_stmt_typ!(&node.props["arglist"]), depth + 1);
                }
            }
            StmType::ArgList => {
                let args = get_stmt_typ!(&node.props["list"], StmtValue::Arr);
                for arg in args {
                    self.print_tree(arg, depth + 1);
                }
            }
            StmType::Return => {
                println!("{}return stmt", seprator.clone());
                self.print_tree(get_stmt_typ!(&node.props["val"]), depth + 1);
            }
            StmType::Arr => {
                println!("{}arr", seprator.clone());
                let vals = get_stmt_typ!(&node.props["vals"], StmtValue::Arr);
                if vals.len() == 0 {
                    println!("{}empty", seprator.clone() + space);
                } else {
                    for val in vals {
                        self.print_tree(val, depth + 1);
                    }
                }
            }
            StmType::HashMap => {
                println!("{}hashmap", seprator.clone());

                let vals = get_stmt_typ!(&node.props["vals"], StmtValue::HashMap);

                if vals.len() == 0 {
                    println!("{}empty", seprator.clone() + space);
                } else {
                    for val in vals {
                        self.print_tree(&val[0], depth + 1);
                        self.print_tree(&val[1], depth + 2);
                    }
                }
            }
            StmType::StmtBlock => {
                println!("{}stmt block", seprator.clone());
                match &node.props["body"] {
                    StmtValue::Arr(block) => {
                        if (block.len() == 0) {
                            println!("{}empty", seprator.clone() + space)
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
