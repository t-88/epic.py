import op_parser

src = """
    func mamajo(asd) {
        print(asd);
    }
    func toot() {
        print("holla");
    }    
    mamajo("Hello its meeeeeee");
    mamajo(toot());
"""

parser =  op_parser.Parser()
parser.parse(src)
parser.print_tree()

transpiler = op_parser.Transpiler()


src = transpiler.transpile(parser.nodes)
print(src)
with open("output.py","w") as f:
    f.write(src)


