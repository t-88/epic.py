import op_lang.op_parser as op_parser
import op_lang.op_transpiler as op_transpiler




src_json = {
    "entities" : [{
        "pos" : {"x" : 10, "y" : 10},
        "size" : {"w" : 20, "h" : 20},
        "script" : """
                func on_input(ID) {
                    pos = get_component(ID,Components.Position);    
                    if(is_pressed(Keys.Left)) {
                        pos.x = pos.x - 1;
                    }
                    if(is_pressed(Keys.Right)) {
                        pos.x = pos.x + 1;
                    }    
                }        
        """,
    }]
}


parser =  op_parser.OPParser()



identation = "\t"

gen = "" 
funcs = ""
for (idx , entity) in enumerate(src_json["entities"]):
    gen += identation + f"rect{idx} =  esper.create_entity()\n"
    gen += identation + f"esper.add_component(rect{idx},ecs_component.Position({entity['pos']['x']},{entity['pos']['y']}))\n"
    gen += identation + f"esper.add_component(rect{idx},ecs_component.Size({entity['size']['w']},{entity['size']['h']}))\n"
    gen += identation + f"esper.add_component(rect{idx},ecs_component.Color(255,0,0))\n" 
    gen += identation + f"esper.add_component(rect{idx},ecs_component.RectShape())\n"
    gen += identation + f"esper.add_component(rect{idx},ecs_component.InputCallback(callback = on_input))\n"
    
    
    parser.parse(entity["script"])
    transpiler = op_transpiler.OPTraspiler()
    funcs += transpiler.transpile(parser.program)
    
    
    
gen += identation + "esper.add_processor(ecs_system.RectRendererSystem())\n"
gen += identation + "esper.add_processor(ecs_system.KeyboardInputSyatem())\n"


src = f"""
from engine import *
import pygame
import ecs.ecs_component as  ecs_component
import ecs.ecs_system as  ecs_system
import esper
from op_lang.build.lookup_tables import *
from op_lang.build.functions import *

# generated funcs
{funcs}


# generated code
def init():
{gen}

def process():
    esper.process()
engine.init_callback = init 
engine.process_callback = process 
engine.init()
engine.run()
"""



with open("gen-program.py","w") as f:
    f.write(src)
