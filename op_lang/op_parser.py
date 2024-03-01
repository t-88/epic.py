from op_base_parser import *
from op_lexer import *
import enum




StatementType = enum.Enum("StatementType",[
    # base types
    "Number", "String",
    "Identifier",
    
    
    "ArthOp",    
    "BooleanOp",    
    "VarAssigment",
    "Para",
    "Block", "Conditional",
    
    
    # key nodes
    "Program",
    "FuncCall",
    "FuncDeclaration",
])

OP_MAP = {
    "+" : "Add",
    "-" : "Sub",
    "*" : "Mul",
    "/" : "Div",
    ">" : "Bigger",
    "<" : "Lesser",
    ">=" : "Bigger Or Equal",
    "=<" : "Lesser Or Equal",
    "==" : "Equal",
}

class Statement:
    def __init__(self,typ,val = ""):
        self.type = typ
        self.val = val
        
    def __str__(self):
        return f"<ExprAdd type={self.type} val={self.val}>"
    def __repr__(self):
        return self.__str__()  

class ExprProgram(Statement):
    def __init__(self) -> None:
        super().__init__(StatementType.Program)
        self.statements = []
    def __str__(self):
        return f"<ExprProgram statements={self.statements}>"        
        
class ExprArithOp(Statement):
    def __init__(self,op,left,right):
        super().__init__(StatementType.ArthOp)
        self.left = left                
        self.right = right    
        self.op = op    
    def __str__(self):
        return f"<ExprArithOp op={self.op} left={self.left} right={self.right}>"
       
class ExprBoolOp(Statement):
    def __init__(self,op,left,right):
        super().__init__(StatementType.BooleanOp)
        self.left = left                
        self.right = right    
        self.op = op    
    def __str__(self):
        return f"<ExprBoolOp op='{self.op}' left={self.left} right={self.right}>"

class ExprPara(Statement):
    def __init__(self,expr):
        super().__init__(StatementType.Para)
        self.expr = expr                
    def __str__(self):
        return f"<ExprPara expr={self.expr}>"

class StatBlock(Statement):
    def __init__(self,block):
        super().__init__(StatementType.Block)
        self.block = block                
    def __str__(self):
        return f"<StatBlock block={self.block}>"


class StatVarAssigment(Statement):
    def __init__(self,name,val):
        super().__init__(StatementType.VarAssigment)
        self.name = name                
        self.val = val    
    def __str__(self):
        return f"<StatVarAssigment name={self.name} val={self.val}>"
class StatConditional(Statement):
    def __init__(self,condition,block):
        super().__init__(StatementType.Conditional)
        self.condition = condition                
        self.block = block    
    def __str__(self):
        return f"<StatConditional condition={self.condition} block={self.block}>"


class StatDefaulFunc(Statement):
    def __init__(self,name,body):
        super().__init__(StatementType.FuncCall)
        self.name = name                
        self.body = body    
    def __str__(self):
        return f"<StatDefaulFunc name={self.name} body={self.body}>"

class StatFuncDeclartion(Statement):
    def __init__(self,name,body):
        super().__init__(StatementType.FuncDeclaration)
        self.name = name                
        self.body = body    
    def __str__(self):
        return f"<StatFuncDeclartion name={self.name} body={self.body}>"



class OPParser(BaseParser):
    def __init__(self):
        super().__init__()

        self.program = None
        self.lexer = Lexer()
        self.src = []
        
        
    def expect(self,typ):
        if self.cur().type != typ:
            print(f"[Parser Expect Error] expected {typ} got {self.cur().type}")
            exit(69)
            
    
    def parse(self,src):
        self.src = self.lexer.tokenize(src)
        self.idx = 0
        
        self.program = ExprProgram()
        
        while self.idx < len(self.src):
            self.program.statements.append(self.parse_statements())
        
        return self.program
    
    def parse_statements(self):
        if self.idx < len(self.src) and self.cur().type == TokenType.OCurl:
            return self.parse_block()
        elif self.idx < len(self.src) and self.cur().type == TokenType._if:
            return self.parse_conditional()
        
        elif self.idx < len(self.src) and self.cur().type == TokenType.Func:
            return self.parse_func_declartion()
        
        else:
            expr =  self.parse_func_call()
            self.expect(TokenType.SemiColon)
            self.next()            
            return expr
    
    def parse_block(self):
        self.next()
        block = []
        while self.cur().type != TokenType.CCurl:
            block.append(self.parse_statements())

        self.next()        
        return StatBlock(block)
        
    def parse_conditional(self):
        self.next()
        condition = self.parse_boolean_ops()
        block = self.parse_block()
        return StatConditional(condition,block)     
    
    
    def parse_func_call(self):
        if (self.cur().type == TokenType.Identifier and self.cur(1).type == TokenType.OPara) or (self.cur().type == TokenType.Outer_Func):
            name = self.next().val

            self.expect(TokenType.OPara)
            self.next()
            body = self.parse_boolean_ops() 
            self.expect(TokenType.CPara)
            self.next()            
            return StatDefaulFunc(name,body)     
            
        return self.parse_var_assigment()
        
       
    
    
    def parse_func_declartion(self):
        self.next()
        name =  self.next().val
        self.expect(TokenType.OPara)
        self.next()
        self.expect(TokenType.CPara)
        self.next()
        
        body = self.parse_statements()
        return StatFuncDeclartion(name,body)
        
        
        
            
    def parse_var_assigment(self):
        if self.cur().type == TokenType.Identifier and self.cur(1).type == TokenType.Equal:
            name = self.next().val
            self.next()
            val = self.parse_var_assigment()

            return StatVarAssigment(name,val)
        return self.parse_boolean_ops()
    

    

        
    def parse_boolean_ops(self):
        left = self.parse_addition_subtraction()
        
        while self.idx < len(self.src) and self.cur().type in [TokenType.Equality,TokenType.Bigger, TokenType.BiggerOrEqual,TokenType.Lesser,TokenType.LesserOrEqual]:
            op = self.next().val
            right = self.parse_addition_subtraction()
            left =  ExprBoolOp(op,left,right)        


        return left

           
    def parse_addition_subtraction(self):
        left = self.parse_muliplication()
        
        while self.idx < len(self.src) and self.cur().type in  [TokenType.Add , TokenType.Minus]:
            op = self.next().val
            right = self.parse_muliplication()
            left =  ExprArithOp(op,left,right)        


        return left
   
    def parse_muliplication(self):
        left = self.parse_para()

        while self.idx < len(self.src) and (self.cur().type == TokenType.Mult or self.cur().type == TokenType.Div):
            op = self.next().val
            right = self.parse_para()
            left =  ExprArithOp(op,left,right)
        return left
    
    
    def parse_para(self):
        if self.idx < len(self.src) and self.cur().type == TokenType.OPara:
            self.next()
            expr = self.parse_boolean_ops()
            self.expect(TokenType.CPara)
            self.next()
            return ExprPara(expr)
        return self.parse_literal()
    
    def parse_literal(self):       
        tkn = self.next()
        if tkn.type == TokenType.String:
            return Statement(StatementType.String,tkn.val)
        elif tkn.type == TokenType.Number:
            return Statement(StatementType.Number,tkn.val)
        elif tkn.type == TokenType.Identifier:
            return Statement(StatementType.Identifier,tkn.val)
        elif tkn.type == TokenType.CCurl or tkn.type == TokenType.CPara:
            self.idx -= 1
            return None
        else:
            print(f"[Parser Error] Unexpected literal '{tkn}'")
            exit(69)
            
    def print_tree(self,node = None,depth = 0):
        if node == None: 
            self.print_tree(self.program)
            return 
        sep = "  " * depth
        
        if node.type == StatementType.Program:
            print("program")
            for statement in node.statements:
                self.print_tree(statement,depth + 1)
        elif node.type == StatementType.String:
            print(sep +  "string " + node.val)
        elif node.type == StatementType.Number:
            print(sep + "number " + node.val)
        elif node.type == StatementType.Identifier:
            print(sep +  "identifier " + node.val)
        elif node.type == StatementType.ArthOp:
            print(sep + OP_MAP[node.op])
            self.print_tree(node.left , depth + 1)
            self.print_tree(node.right, depth + 1)
        elif node.type == StatementType.BooleanOp:
            print(sep + OP_MAP[node.op])
            self.print_tree(node.left , depth + 1)
            self.print_tree(node.right, depth + 1)            
        elif node.type == StatementType.VarAssigment:
            print(sep + "Var Assign")
            print(sep + node.name)
            self.print_tree(node.val , depth + 1)
        elif node.type == StatementType.Para:
            print(sep + "Para")
            self.print_tree(node.expr , depth + 1)            
        elif node.type == StatementType.Block:
            print(sep + "Block")
            for statement in node.block:
                self.print_tree(statement , depth + 1)            
        elif node.type == StatementType.Conditional:
            print(sep + "Conditional")
            self.print_tree(node.condition , depth + 1)                     
            self.print_tree(node.block , depth + 1)                     
        elif node.type == StatementType.FuncCall:
            print(sep + "FuncCall")
            print(sep + node.name)
            if node.body != None:
                self.print_tree(node.body , depth + 1)   
        elif node.type == StatementType.FuncDeclaration:
            print(sep + "FuncDeclaration")
            print(sep + node.name)
            self.print_tree(node.body , depth + 1)                     
                              
        else:
            print(f"[Parser Error] Unexpected node to be printed '{node}'")
            exit(69)
            