try:
    from op_parser import *
except ImportError:
    from .op_parser import *


class OPTraspiler:
    def __init__(self):
        self.src  = ""
        self.prefix_functions = ""
        self.funcs = {}
    def transpile(self,node,iden = 0,prefix_functions = ""):
        src = "" 
        identation = iden * "\t"
        
        if node.type == StatementType.Program:
            self.prefix_functions = prefix_functions
            self.funcs = {}
            for (idx,statement) in enumerate(node.statements) :
                src += self.transpile(statement)
                src += "\n"
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
            src = identation +  node.name  + " = " +  self.transpile(node.val)
        elif node.type == StatementType.Para:
            src =  f"({self.transpile(node.expr)})"
        elif node.type == StatementType.Block:
            for (idx , statement) in enumerate(node.block):
                src += self.transpile(statement,iden)
                if idx != len(node.block) - 1 : src += "\n"
            
            if len(node.block) == 0:
                src = identation + "pass"
            
        elif node.type == StatementType.Conditional:
            src = identation + "if " + self.transpile(node.condition) + ":\n"
            src += self.transpile(node.block,iden + 1)
        elif node.type == StatementType.FuncCall:
            src = identation + node.name + "("
            for (i,arg) in enumerate(node.args):
               src += self.transpile(arg)
               if i < len(node.args) - 1:
                   src += ","
            src += ")"
        elif node.type == StatementType.ForIteration:
            src = identation + f"for {self.transpile(node.var)} in range({self.transpile(node.start)},{self.transpile(node.end)}):\n"
            src += self.transpile(node.block , iden + 1)                     
            
        elif node.type == StatementType.FuncDeclaration:
            src = "def " + self.prefix_functions + node.name + "("
            for (i,arg) in enumerate(node.args):
               src += arg
               if i < len(node.args) - 1:
                   src += ","
                
            src += "):\n" 
            if len(node.body.block) == 0:
                src += (iden + 1) * "\t" + "pass"
            else:
                src += self.transpile(node.body,iden + 1)
                
            self.funcs[self.prefix_functions + node.name] = src
        elif node.type == StatementType.TableLookup:
            src += node.table + "["
            src += f'"{node.key}"'
            src += "]"
        else:
            print(f"[Transpiler Error] Unexpected node to be transpiled '{node}'")
            exit(69)
            
        
        return src
            