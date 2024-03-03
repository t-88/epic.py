import op_parser
import op_transpiler
src = open("main.op","r").read()


parser =  op_parser.OPParser()
parser.parse(src)
# print(parser.program)
# parser.print_tree()

transpiler = op_transpiler.OPTraspiler()
src = transpiler.transpile(parser.program)

src = f"""
# imports
from lookup_tables import *
from functions import *

# transpiled code
{src}
"""
with open("build/output.py","w") as f:
    f.write(src)