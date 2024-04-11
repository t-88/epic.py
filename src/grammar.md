# Grammer Language Syntax
- * : none or more
- | : or
- , : and
- .. : spread
- ; : end of line
- () : optional

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



- IfStmt = IfKeyword , ( , Expr , ) , StmtBlock ;
- ForStmt = ForKeyword , ( , VariableDeclaration , SemiCol , BooleanExpr , SemiCol , Expr  , ) , StmtBlock ;
- WhileStmt = WhileKeyword , ( , BooleanExpr, ) , StmtBlock ; 
- StmtBlock = { , StmtList , };


- Ident = letter , char*;
- Keyword = ForKeyword | IfKeyword | WhileKeyword;
- ForKeyword = "for";
- IfKeyword = "if";
- WhileKeyword = "while";
- LetKeyword = "let";
- TrueKeyword = "true";
- FalseKeyword = "false";


- String = " , char* , "; 
- Number = digit | digit  , . , digit*;
- ArhOp = "+" | "-" | "*" | "/";
- BooleanOp = "&&" | "||" | ">" | "<" | ">=" | "<=" | "==";
- SemiCol = ";";
- Equal = "=";
- digit = 0 | .. | 9;
- char = letter | digit;
- letter = a | .. | z | A | .. | Z;



