# Grammer Language Syntax
- * : none or more
- | : or
- , : and
- .. : spread
- ; : end of line
- () : optional
- $  : empty , nothing

# Op Language Synatx 
- Program = StmtList;
- StmtList = StmtList stmt | stmt;
- Stmt = Expr 
        | IfStmt 
        | ForStmt
        | WhileStmt;
- Expr =  Number 
        | String
        | AssigmentExpr
        | ArthExpr;


- AssigmentDeclaration = Ident , = , {Number , String , Ident} , SemiCol; 
- VariableDeclaration = LetKeyword , Ident = , {Number , String , Ident} , SemiCol; 
- ArthExpr =  Number , ArthOp , Number 
            | ArthExpr  , ArthOp , Number
            | (ArthExpr);

- BooleanExpr =   BooleanExpr  , BooleanOp , BooleanExpr
                | (BooleanExpr)
                | ArthExpr
                | Ident
                | Number
                | TrueKeywrod 
                | FalseKeyword; 



- StmtBlock = { , StmtList , };
- IfStmt = IfKeyword , ( , Expr , ) , StmtBlock ;
- ForStmt = ForKeyword , ( , VariableDeclaration , SemiCol , BooleanExpr , SemiCol , Expr  , ) , StmtBlock ;
- WhileStmt = WhileKeyword , ( , BooleanExpr, ) , StmtBlock ; 
- FuncDeclaration = FuncKeyword , Ident , (  , Arglist , ) , StmtBlock;
- Arglist = | Arg , (Comma , Arglist)*
- Arg =   RequiredArg
        | OptionalArg;
- RequiredArg = Iden; 
- OptionalArg = AssigmentExpr; 



- Ident = letter , char*;
- Keyword = ForKeyword | IfKeyword | WhileKeyword | FuncKeyword | TrueKeyword | FalseKeyword;
- ForKeyword = "for";
- IfKeyword = "if";
- WhileKeyword = "while";
- LetKeyword = "let";
- TrueKeyword = "true";
- FalseKeyword = "false";
- FuncKeyword = "func";

- String = " , char* , "; 
- Number = digit | digit  , . , digit*;
- ArhOp = "+" | "-" | "*" | "/";
- BooleanOp = "&&" | "||" | ">" | "<" | ">=" | "<=" | "==";
- SemiCol = ";";
- Comma = ",";
- Equal = "=";
- digit = 0 | .. | 9;
- char = letter | digit;
- letter = a | .. | z | A | .. | Z;



