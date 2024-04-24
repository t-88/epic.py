use std::{fmt::format, vec};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TknKeyword {
    For,
    While,
    If,
    Elseif,
    Else,
    Let,
    Func,
    True,
    False,
    Return,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TknType {
    Number,
    String,
    Keyword(TknKeyword),
    Ident,
    OPara,
    CPara,
    OCurl,
    CCurl,
    OSqr,
    CSqr,
    AndBool,
    OrBool,
    Bigger,
    BiggerEq,
    Less,
    LessEq,
    Equalily,
    NotEqualily,
    And,
    Or,
    Plus,
    Minus,
    Mult,
    Div,
    Colon,
    Equal,
    Comma,
    SemiCol,
    Dot,
    Sys,
    EOF,
}

#[derive(Debug, Clone)]
pub struct Tkn {
    pub typ: TknType,
    pub val: String,
    pub line: u64,
    col: u64,
}
impl Tkn {
    pub fn new(typ: TknType, val: &str, line: u64, col: u64) -> Tkn {
        Tkn {
            typ: typ,
            val: val.to_string(),
            line: line,
            col: col,
        }
    }
}

const SKIPPABLES: &[char] = &['\n', '\t', '\r', ' '];
const KEYWORDS: &[&str] = &[
    "for", "while", "if", "else", "elseif", "let", "true", "false", "func", "return",
];
const ARTH_OPS: &[char] = &['+', '-', '*', '/'];
const SINGLE_CHAR_BOOL_OPS: &[&str] = &[">", "<"];
const TWO_CHAR_BOOL_OPS: &[&str] = &["&&", "||", ">=", "<=", "==", "!="];
const SINGLE_CHAR_SYMBS: &[char] = &[
    '(', ')', '{', '}', '[', ']', '&', '|', '=', ',', ':', '.', '$',
];

pub struct Lexer {
    pub src: String,
    pub tknz: Vec<Tkn>,
    pub errs: Vec<String>,
    idx: u64,
    line: u64,
    col: u64,
}

impl Lexer {
    pub fn new() -> Lexer {
        Lexer {
            src: "".to_string(),
            tknz: vec![],
            errs: vec![],
            idx: 0,
            col: 0,
            line: 1,
        }
    }

    fn is_empty(self: &Self) -> bool {
        return self.idx >= self.src.len() as u64;
    }
    fn next(self: &mut Self) -> char {
        let chr: char = self.get();
        self.idx += 1;
        return chr;
    }
    fn get(self: &mut Self) -> char {
        return self.src.chars().nth(self.idx as usize).unwrap();
    }
    fn push_tkn(self: &mut Self, typ: TknType, val: String) {
        self.tknz.push(Tkn {
            typ: typ,
            val: val,
            col: self.col,
            line: self.line,
        });
    }

    fn push_err(self: &mut Self, msg: String) {
        self.errs.push(format!("line {}: {}", self.line, msg));
    }
    fn map_keyword(self: &Self, keyword: &String) -> Option<TknKeyword> {
        assert!(KEYWORDS.contains(&keyword.as_str()));
        match keyword.as_str() {
            "for" => Some(TknKeyword::For),
            "while" => Some(TknKeyword::While),
            "if" => Some(TknKeyword::If),
            "else" => Some(TknKeyword::Else),
            "elseif" => Some(TknKeyword::Elseif),
            "let" => Some(TknKeyword::Let),
            "true" => Some(TknKeyword::True),
            "false" => Some(TknKeyword::False),
            "func" => Some(TknKeyword::Func),
            "return" => Some(TknKeyword::Return),
            _ => unreachable!(),
        }
    }
    fn map_arthop(self: &Self, chr: &char) -> Option<TknType> {
        assert!(ARTH_OPS.contains(chr));
        match chr {
            '+' => Some(TknType::Plus),
            '-' => Some(TknType::Minus),
            '*' => Some(TknType::Mult),
            '/' => Some(TknType::Div),
            _ => unreachable!(),
        }
    }
    fn map_booleanop(self: &Self, word: &str) -> Option<TknType> {
        assert!(SINGLE_CHAR_BOOL_OPS.contains(&word) || TWO_CHAR_BOOL_OPS.contains(&word));
        match word {
            "&&" => Some(TknType::AndBool),
            "||" => Some(TknType::OrBool),
            ">" => Some(TknType::Bigger),
            "<" => Some(TknType::Less),
            ">=" => Some(TknType::BiggerEq),
            "<=" => Some(TknType::LessEq),
            "==" => Some(TknType::Equalily),
            "!=" => Some(TknType::NotEqualily),
            _ => None,
        }
    }
    fn map_symbs(self: &Self, chr: &char) -> Option<TknType> {
        assert!(SINGLE_CHAR_SYMBS.contains(&chr));
        match chr {
            '=' => Some(TknType::Equal),
            '&' => Some(TknType::And),
            '|' => Some(TknType::Or),
            '(' => Some(TknType::OPara),
            ')' => Some(TknType::CPara),
            '{' => Some(TknType::OCurl),
            '}' => Some(TknType::CCurl),
            ']' => Some(TknType::CSqr),
            '[' => Some(TknType::OSqr),
            ',' => Some(TknType::Comma),
            ':' => Some(TknType::Colon),
            '.' => Some(TknType::Dot),
            '$' => Some(TknType::Sys),
            _ => None,
        }
    }

    pub fn tokenize(self: &mut Self, src: &str) {
        self.idx = 0;
        self.tknz = vec![];
        self.src = src.to_string();

        while (!self.is_empty()) {
            let mut chr: char = self.next();

            if (SKIPPABLES.contains(&chr)) {
                if (chr == '\n') {
                    self.line += 1;
                    self.col = 0;
                } else {
                    self.col += 1;
                }
            } else if chr == '/' && !self.is_empty() && self.get() == '/' {
                self.next();
                while !self.is_empty() && self.get() != '\n' {
                    self.next();
                }
            } else if chr.is_alphabetic() {
                let mut word: String = String::from(chr);
                let mut col_offset = 1;
                while !self.is_empty() && (self.get().is_alphanumeric() || self.get() == '_') {
                    chr = self.next();
                    word.push(chr);
                    col_offset += 1;
                }
                if (KEYWORDS.contains(&word.as_str())) {
                    self.push_tkn(TknType::Keyword((self).map_keyword(&word).unwrap()), word);
                } else {
                    self.push_tkn(TknType::Ident, word);
                }
                self.col += col_offset;
            } else if chr.is_numeric()
                || (['-', '+'].contains(&chr) && !self.is_empty() && self.get().is_numeric())
            {
                let mut word: String = String::from(chr);
                let mut col_offset = 1;
                while !self.is_empty() && self.get().is_numeric() {
                    chr = self.next();
                    word.push(chr);
                    col_offset += 1;
                }
                if !self.is_empty() && self.get() == '.' {
                    word.push(self.next());
                    col_offset += 1;
                    while !self.is_empty() && self.get().is_numeric() {
                        chr = self.next();
                        word.push(chr);
                        col_offset += 1;
                    }
                }
                self.push_tkn(TknType::Number, word);
                self.col += col_offset;
            } else if chr == '"' || chr == '\'' {
                let string_start: char = chr;
                let mut string: String = String::from("");
                let mut col_offset = 1;
                while !self.is_empty() && self.get() != string_start {
                    string.push(self.next());
                    col_offset += 1;
                }
                if (!self.is_empty() && self.get() == string_start) {
                    self.next();
                    col_offset += 1;
                    self.push_tkn(TknType::String, string);
                    self.col += col_offset;
                } else {
                    self.push_err(String::from("Expected closure string but not found"));
                    self.col += col_offset;
                }
            } else if chr == ';' {
                self.push_tkn(TknType::SemiCol, String::from(chr));
                self.col += 1;
            } else if ARTH_OPS.contains(&chr) {
                self.push_tkn(self.map_arthop(&chr).unwrap(), String::from(chr));
            } else if !self.is_empty()
                && TWO_CHAR_BOOL_OPS
                    .contains(&(String::from(chr) + &String::from(self.get())).as_str())
            {
                let mut op: String = String::from("");
                op.push(chr);
                op.push(self.get());

                self.push_tkn(self.map_booleanop(&op).unwrap(), op);
                self.col += 2;
                self.idx += 1;
            } else if SINGLE_CHAR_BOOL_OPS.contains(&String::from(chr).as_str()) {
                self.push_tkn(
                    self.map_booleanop(&chr.to_string().as_str()).unwrap(),
                    String::from(chr),
                );
                self.col += 1;
            } else if SINGLE_CHAR_SYMBS.contains(&chr) {
                self.push_tkn(self.map_symbs(&chr).unwrap(), String::from(chr));
                self.col += 1;
            } else {
                self.push_err(format!("unexpected character '{}'", chr));
            }
        }

        self.tknz.push(Tkn {
            col: self.col,
            line: self.line,
            typ: TknType::EOF,
            val: "".to_string(),
        });
    }
}
