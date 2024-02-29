import enum

Token = enum.Enum("Token",["Word","FunctionDeclaration","Number","String","OPara","CPara","DollarSymb","Dot","OCurl","CCurl","SemiColon"])
skippables = [" ","\n","\t"]

class Tokenizer:
    def __init__(self):
        self.src = ""
        self.idx = 0

    def tokenize(self,src):
        self.src = src
        self.idx = 0
        tokens = []

        while self.idx < len(self.src):
            cur = self.src[self.idx]
            self.idx += 1

            if cur in ["(",")","$",".","{","}",";"]:
                if cur == "(": tokens.append((Token.OPara,cur))
                elif cur == ")": tokens.append((Token.CPara,cur))
                elif cur == "$": tokens.append((Token.DollarSymb,cur))
                elif cur == ".": tokens.append((Token.Dot,cur))
                elif cur == "{": tokens.append((Token.OCurl,cur))
                elif cur == "}": tokens.append((Token.CCurl,cur))
                elif cur == ";": tokens.append((Token.SemiColon,cur))
            elif cur == '"':
                word = ""
                while self.src[self.idx] != '"':
                    word += self.src[self.idx]
                    self.idx += 1
                self.idx += 1
                tokens.append((Token.String,word))
            elif cur.isalpha():
                word = cur
                while self.src[self.idx].isalnum():
                    word += self.src[self.idx]
                    self.idx += 1
                if word == "func": tokens.append((Token.FunctionDeclaration,word)) 
                else : tokens.append((Token.Word,word))
            elif cur.isdigit():
                word = cur
                while self.src[self.idx].isdigit():
                    word += self.src[self.idx]
                    self.idx += 1
                tokens.append((Token.Number,int(word)))
            elif cur in skippables:
                pass
            else:
                print(f"Unexpected token '{cur}'")
                exit(69)    
        return tokens   
    

Node = enum.Enum("Node",["FuncDec","FuncCall","String","Number"])
Widget = enum.Enum("Widget",["button"])
widget_list = [e.name for e in Widget]
component_list = ["pos","size","color"]
class BaseParser:
    def __init__(self):
        self.tokenizer = Tokenizer()
        self.tokens = []
        self.idx = 0
        self.nodes = []
        self.lookup = {}
        
        
    def cur_token(self,step = 0):
        return self.tokens[self.idx + step]
            
    def expect(self,token_type):
        if self.cur_token()[0] != token_type:
            print(f"Unexpected token in expect expected '{token_type}' got '{self.cur_token()[0]}'")
            exit(69)        

    def parse(self,src,lookup = {}):
        pass
            
class Parser(BaseParser):
    def __init__(self):
        super().__init__()
    def parse(self,src,lookup = {}):
        self.tokens = self.tokenizer.tokenize(src)
        self.idx = 0
        self.lookup = lookup
        while self.idx < len(self.tokens):
            self.nodes.append(self.parse_experssion())
        
    def parse_experssion(self):
        if self.cur_token()[0] == Token.FunctionDeclaration:
            return self.parse_function_dec()
        elif self.cur_token()[0] == Token.Word and self.cur_token(1)[0] == Token.OPara:
            return self.parse_function_call()
        return self.parse_literal()
    
    def parse_function_dec(self):
        self.idx += 1


        self.expect(Token.Word)
        func_name = self.cur_token()[1]
        self.idx += 1
        

        self.expect(Token.OPara)
        self.idx += 1
        self.expect(Token.CPara)
        self.idx += 1

        self.expect(Token.OCurl)
        self.idx += 1

        block = []
        
        while self.cur_token()[0] != Token.CCurl:
            block.append(self.parse_experssion())
        
        self.expect(Token.CCurl)
        self.idx += 1
        
        return (Node.FuncDec,{"func_name" : func_name,"block": block})

    def parse_function_call(self):
        func_name =  self.cur_token()[1]
        self.idx += 1
        
        self.expect(Token.OPara)
        self.idx += 1
        
        args = []
        while self.cur_token()[0] != Token.CPara:
            args.append(self.parse_experssion())

        self.expect(Token.CPara)
        self.idx += 1
        
        self.expect(Token.SemiColon)
        self.idx += 1
        
        return (Node.FuncCall,{"func_name" : func_name,"args": args})
        

    def parse_literal(self):
        token  = self.cur_token()
        self.idx += 1
        if token[0] == Token.String:
            return (Node.String , {"val" : token[1]})
        elif token[0] == Token.Number:
            return (Node.Number , {"val" : token[1]})
        else:
            print(f"Unexpected literal '{token}'")
            exit(69)

    def print_tree(self, nodes = None,depth = 0):
        if nodes == None:
            self.print_tree(self.nodes)
        else:
            if type(nodes) == list:
                for node in nodes:
                    self.print_tree(node,depth + 1)
            else:
                if nodes[0] == Node.FuncDec:
                    print("  " * depth + "[function declaration]")
                    print("  " * depth + "[name] " +  nodes[1]["func_name"])
                    print("  " * depth + "[block]")
                    self.print_tree(nodes[1]["block"],depth + 1)
                elif nodes[0] == Node.FuncCall:
                    print("  " * depth + "[function call]")
                    print("  " * depth + "[name] " +  nodes[1]["func_name"])
                    print("  " * depth + "[args]")
                    self.print_tree(nodes[1]["args"],depth + 1)
                elif nodes[0] == Node.String:
                    print("  " * depth + "[string literal] " + nodes[1]["val"])
                elif nodes[0] == Node.Number:
                    print("  " * depth + "[number literal] " + nodes[1]["val"])
                else:
                    print(f"Unhandled node in print_tree function '{nodes}'")
                    exit(69)
            

class Transpiler:
    def __init__(self):
        self.src = ""
        
    
    def transpile(self,nodes,depth = -1):
        if nodes == None: return
        src = ""
        if type(nodes) == list:
            for node in nodes:
                src += self.transpile(node,depth + 1)
            return src
        else:
            if nodes[0] == Node.FuncDec:
                src =  '\t' * depth + f"def {nodes[1]['func_name']}():\n"
                src += '\t' * (depth + 1) + self.transpile(nodes[1]["block"],depth + 1)
                return  src
            elif nodes[0] == Node.FuncCall:
                src = nodes[1]["func_name"] + "("
                src += self.transpile(nodes[1]["args"],depth + 1)
                src += ")\n"
                return src
            elif nodes[0] == Node.String:
                return '"'+nodes[1]["val"]+'"'
            elif nodes[0] == Node.Number:
                return nodes[1]["val"]
            else:
                print(f"Unhandled node in transpile function '{nodes}'")
                exit(69)
    
    