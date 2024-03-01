from op_parser import *


class OPTraspiler:
    def __init__(self):
        self.src  = ""
    
    
    def transpile(self,node,iden = 0):
        src = "" 
        identation = iden * "\t"
        
        if node.type == StatementType.Program:
            for (idx,statement) in enumerate(node.statements) :
                src += self.transpile(statement)
                if idx != len(node.statements) - 1 : src += "\n"
        elif node.type == StatementType.String:
            src = f'"{node.val}"'
        elif node.type == StatementType.Number:
            src = f'{node.val}'  
        elif node.type == StatementType.Identifier:
            src = node.val                
        elif node.type == StatementType.ArthOp:
            src =  self.transpile(node.left) + " " +  node.op + " " +  self.transpile(node.right)
        elif node.type == StatementType.BooleanOp:
            src =  self.transpile(node.left) + " " +  node.op + " " +  self.transpile(node.right)
        elif node.type == StatementType.VarAssigment:
            src =  node.name  + " = " +  self.transpile(node.val)
        elif node.type == StatementType.Para:
            src =  f"({self.transpile(node.expr)})"
        elif node.type == StatementType.Block:
            for (idx , statement) in enumerate(node.block):
                src += identation + self.transpile(statement)
                if idx != len(node.block) - 1 : src += "\n"
                
        elif node.type == StatementType.Conditional:
            src = "if " + self.transpile(node.condition) + ":\n"
            src += self.transpile(node.block,iden + 1)
        elif node.type == StatementType.FuncCall:
            src = node.name + "("
            if node.body != None:
                src += self.transpile(node.body,0)
            src += ")\n"

             
        elif node.type == StatementType.FuncDeclaration:
            src = "def " + node.name + "():\n" 
            if len(node.body.block) == 0:
                src += (iden + 1) * "\t" + "pass"
            else:
                src += self.transpile(node.body,iden + 1)
        else:
            print(f"[Transpiler Error] Unexpected node to be transpiled '{node}'")
            exit(69)
            
        
        return src
            