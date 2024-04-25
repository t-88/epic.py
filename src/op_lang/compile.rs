use crate::{lexer::Lexer, parser::Parser, sym_analyzer::SymenticAnal, transpiler::{TranspileLang, Transpiler}};

pub fn compile(lang:  TranspileLang,src: String, func_prefix: &str) -> (i32, Vec<String>) {
    let mut lexer: Lexer = Lexer::new();
    lexer.tokenize(&src);
    if (lexer.errs.len() > 0) {
        return (1, lexer.errs);
    }

    let mut parser: Parser = Parser::new();
    parser.parse(&src);
    if (parser.errs.len() > 0) {
        return (2, parser.errs);
    }

    let mut analyzer: SymenticAnal = SymenticAnal::new();
    analyzer.analyse(&parser.program);
    if (analyzer.errs.len() > 0) {
        return (3, analyzer.errs);
    }

    let transpiler: Transpiler = Transpiler::new();
    let mut src : String = "".to_string();
    match lang {
        TranspileLang::Py => {
            src = transpiler.py_transpiler(&parser.program, 0, &mut true, &func_prefix.to_string());
        }
        TranspileLang::Js => {
            src = transpiler.js_transpiler(&parser.program, 0, &mut true, &func_prefix.to_string());
        }
        _ => {
            unreachable!("unknown language to transpile to");
        }
    }
    return (0, vec![src]);
}
