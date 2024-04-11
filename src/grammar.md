# Grammer Language Syntax
- * : none or more
- | : or
- , : and
- .. : spread
- ; : end of line
- () : optional

# Op Language Synatx 
- Program = stmtList;
- stmtList = stmtList stmt | stmt;
- stmt = AssigmentExpr;

- AssigmentExpr = Ident , = , {Number , String , Ident} , SemiCol; 
- VariableDeclaration = LetKeyword , Ident = , {Number , String , Ident} , SemiCol; 
- ArthExpr =  Number , ArthOp , Number 
            | ArthExpr  , ArthOp , Number;

- Ident = letter , char*;

- Keyword = ForKeyword | IfKeyword | WhileKeyword;
- ForKeyword = "for";
- IfKeyword = "if";
- WhileKeyword = "while";
- LetKeyword = "let";


- String = " , char* , "; 
- Number = digit | digit  , . , digit*;
- ArhOp = "+" | "-" | "*" | "/";
- BooleanOp = "&&" | "||" | ">" | "<" | ">=" | "<=" | "==";
- SemiCol = ";";
- Equal = "=";
- digit = 0 | .. | 9;
- char = letter | digit;
- letter = a | .. | z | A | .. | Z;



