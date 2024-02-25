import enum

Token = enum.Enum("Token",["Word","Number","OPara","CPara","DollarSymb","Dot"])
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

            if cur in ["(",")","$","."]:
                if cur == "(": tokens.append((Token.OPara,cur))
                elif cur == ")": tokens.append((Token.CPara,cur))
                elif cur == "$": tokens.append((Token.DollarSymb,cur))
                elif cur == ".": tokens.append((Token.Dot,cur))
            elif cur.isalpha():
                word = cur
                while self.src[self.idx].isalnum():
                    word += self.src[self.idx]
                    self.idx += 1
                tokens.append((Token.Word,word))
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


Node = enum.Enum("Node",["Widget","SystemLookUp"])
Widget = enum.Enum("Widget",["button"])
widget_list = [e.name for e in Widget]
component_list = ["pos","size","color"]
class Parser:
    def __init__(self) -> None:
        self.tokenizer = Tokenizer()
        self.tokens = []
        self.idx = 0
        
        self.nodes = []
        
        self.lookup = {}
        
    
    def cur_token(self):
        return self.tokens[self.idx]

    def expect(self,token_type):
        if self.cur_token()[0] != token_type:
            print(f"Unexpected token in expect expected '{token_type}' got '{self.cur_token()[0]}'")
            exit(69)
    
    def parse(self,src,lookup = {}):
        self.tokens = self.tokenizer.tokenize(src)
        self.idx = 0
        self.lookup = lookup
        
        while self.idx < len(self.tokens):
            self.nodes.append(self.parse_widget())
    
    def parse_widget(self):
        if self.cur_token()[0] == Token.Word and self.cur_token()[1] in widget_list:
            widget = self.parse_literal()

            self.expect(Token.OPara)
            self.idx += 1
            
            components = []
            while self.cur_token()[0] != Token.CPara:
                if self.cur_token()[1] in component_list:
                    components.append(self.parse_component()) 
                else:
                    print(f"Unexpeted Component '{self.cur_token()[1]}'")
                    exit(69)
                    
            self.expect(Token.CPara)
            self.idx += 1
            
            
            return (Node.Widget,(widget,components)) 

        print(f"Unexpected token '{self.cur_token()}'")
        exit(69)

    def parse_component(self):
        self.expect(Token.Word)
        typ = self.cur_token()[1]
        self.idx += 1
        
        self.expect(Token.OPara)
        self.idx += 1
        
        props = []
        while self.cur_token()[0] != Token.CPara:
            props.append(self.parse_system()) 
            

        self.expect(Token.CPara)
        self.idx += 1
        
        return (typ,props)
    
    def parse_system(self):
        if self.cur_token()[0] == Token.DollarSymb : 
            self.idx += 1
            
            keys = []
            while self.cur_token()[0] == Token.Dot:
                self.idx += 1
                keys.append(self.cur_token()[1])                
                self.idx += 1
                
            lookup = self.lookup
            for key in keys:
                if key in lookup:
                    lookup = lookup[key]
                else:
                    print(f"Failed lookup key='{key}' lookup='{lookup}'")
                    exit(69)
            return lookup
        
        
        return self.parse_literal()
    
    def parse_literal(self):
        token = self.cur_token()
        self.idx += 1
        
        if token[0] == Token.Word:
            return token[1]
        elif token[0] == Token.Number:
            return token[1]
        else:
            print(f"Unexpected literal'{token}'")
            exit(69)    


from ecs_component import *
from ecs_system import *
import esper


class Generator:
    def __init__(self):
        self.widgets = []
    
    
    def generate(self,nodes):
        for node in nodes:
            if node[0] == Node.Widget:
                self.widgets.append(self.create_widget(node))
            else:
                print(f"Unexpcted node '{node}'")
                exit(69)
    
    def create_widget(self,node):
        typ =  node[1][0]

        components = []
        for component in node[1][1]:
            components.append(self.create_component(component))
        
        
        
        widget = None
        if typ == Widget.button.name:
            widget = esper.create_entity()
            esper.add_component(widget,Button(callback= lambda : print("asdasd")))
            for component in components:
                esper.add_component(widget,component)
        
        return widget
    
    def create_component(self,component):
        typ = component[0]
        args = component[1]
        
        if typ == "pos":
            return Position(args[0],args[1])
        elif typ == "size":
            return Size(args[0],args[1])
        elif typ == "color":
            return Color(args[0],args[1],args[2])        
        
        
        
