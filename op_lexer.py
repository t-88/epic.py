import enum
from op_base_parser import *

TokenType = enum.Enum(
    "TokenType",
    [
        # single tokens
        "OPara", "CPara",
        "OCurl", "CCurl",
        "Dot", "Comma",
        "Equal", "SemiColon",
        "Add", "Minus", 
        "Mult", "Div",
        "Bigger", "Lesser",

        
        # two or more tokens
        "Equality", 
        "BiggerOrEqual", "LesserOrEqual",
        
        
        # literals
        "Identifier", "String",
        "Number",
        
        # keywords
        "_if" , 
        "And",  "Or", 
        "Func", "Return",
        "false" , "true",
        
        # function
        "Outer_Func",
    ]
)


SINGLE_TOKENS_MAP = {
    "(" : TokenType.OPara     ,
    ")" : TokenType.CPara     ,
    "{" : TokenType.OCurl     ,
    "}" : TokenType.CCurl     ,
    "." : TokenType.Dot       ,
    "," : TokenType.Comma     ,
    "=" : TokenType.Equal     ,
    ";" : TokenType.SemiColon ,
    "+" :  TokenType.Add      ,
    "-" :  TokenType.Minus    ,
    "*" :  TokenType.Mult     ,
    "/" :  TokenType.Div      ,
    ">" : TokenType.Bigger,
    "<" : TokenType.Lesser,
}
SINGLE_TOKENS = SINGLE_TOKENS_MAP.keys()

DOUBLE_OR_MORE_TOKENS_MAP = {
    "==" : TokenType.Equality,
    ">=" : TokenType.BiggerOrEqual,
    "=<" : TokenType.LesserOrEqual,
}
DOUBLE_OR_MORE_TOKENS = DOUBLE_OR_MORE_TOKENS_MAP.keys()

KEYWORDS_MAP = {
    "func"   : TokenType.Func,
    "return" : TokenType.Return,
    "false"  : TokenType.false,
    "true"   : TokenType.true,
    "if"   : TokenType._if,
} 
KEYWORDS = KEYWORDS_MAP.keys()


FUNCTIONS = ["print"]

SKIPABLE_TOKENS = [" ", "\n", "\t"]

class Token:
    def __init__(self,type,val):
        self.type =  type
        self.val = val
    def __str__(self):
        return f"<Token type='{self.type}' val='{self.val}'>"
    def __repr__(self):
        return self.__str__()

class Lexer(BaseParser):
    def __init__(self):
        super().__init__()
        self.tokens = []
        
        
    def tokenize(self,src):
        self.idx = 0
        self.src = src
        self.tokens = []
        while self.idx < len(self.src):
            chr = self.next()
            if chr in SINGLE_TOKENS:
                if self.idx < len(self.src) and  chr + self.cur() in DOUBLE_OR_MORE_TOKENS: 
                    self.tokens.append(Token(DOUBLE_OR_MORE_TOKENS_MAP[chr + self.cur()],chr + self.cur()))
                    self.next()
                else:
                    self.tokens.append(Token(SINGLE_TOKENS_MAP[chr],chr))
            elif chr == '"' or chr == "'":
                stop = chr
                word = ""
                while self.idx < len(self.src) and self.cur() != stop:
                    word += self.next()
                
                self.next()
                self.tokens.append(Token(TokenType.String,word))
            elif chr.isalpha():
                word = chr
                while self.idx < len(self.src) and self.cur().isalpha():
                    word +=  self.next()
                
                typ = TokenType.Identifier
                if word in KEYWORDS:
                    typ = KEYWORDS_MAP[word] 
                elif word in FUNCTIONS:
                    typ = TokenType.Outer_Func 
                self.tokens.append(Token(typ,word))
            elif chr.isnumeric():
                word = chr
                while self.idx < len(self.src) and self.cur().isnumeric():
                    word +=  self.next()
                self.tokens.append(Token(TokenType.Number,word))
            elif chr in SKIPABLE_TOKENS:
                pass
            else:
                print(f"[Lexer Error] Unexpected token '{chr}'")
                exit(69)
        return self.tokens