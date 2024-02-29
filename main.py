import op_parser


src = """
    func mamajo() {
        print("asdasdasdasd");
    }
    func toot() {
        print("taki is cool");
    }    
    
    toot();
    mamajo();
"""

parser =  op_parser.Parser()
parser.parse(src)

transpiler = op_parser.Transpiler()



src = transpiler.transpile(parser.nodes)
with open("output.py","w") as f:
    f.write(src)


