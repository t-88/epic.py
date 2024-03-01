import op_parser
import op_transpiler
src = open("main.op","r").read()


parser =  op_parser.OPParser()
parser.parse(src)
print(parser.program)
parser.print_tree()



print()
transpiler = op_transpiler.OPTraspiler()
src = transpiler.transpile(parser.program)

print(src)
with open("build/output.py","w") as f:
    f.write(src)