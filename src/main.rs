mod lexer;

fn main() {
    let mut lexer : lexer::Lexer = lexer::Lexer::new(); 
    lexer.tokenize("
        {
            '(5 == 3)'
            asd > 2
        }
    ");


    if(lexer.errs.len() > 0) {
        println!("Lexer found {} errs",lexer.errs.len());
        for lex_err in lexer.errs  {
            println!("{}",lex_err.error);
        }
        return;
    }
    for tkn in lexer.tknz  {
        println!("{:?}",tkn);
    }    
}
