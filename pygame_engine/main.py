import json
from op_lang import *

src = json.load(open("src.json","r"))

parser =  op_parser.OPParser()
transpiler = op_transpiler.OPTraspiler()
identation = "\t"
gen = "" 
funcs = ""
config = ""


# generate entities
for (idx , entity) in enumerate(src["children"]):
    gen += identation + f"rect{idx} =  esper.create_entity()\n"
    
    comps = entity['comps']
    gen += identation + f"esper.add_component(rect{idx},ecs_component.Position({comps['pos']['x']},{comps['pos']['y']}))\n"
    gen += identation + f"esper.add_component(rect{idx},ecs_component.Size({comps['size']['w']},{comps['size']['h']}))\n"
    gen += identation + f"esper.add_component(rect{idx},ecs_component.Color({comps['color']['r']},{comps['color']['g']},{comps['color']['b']}))\n"
    gen += identation + f"esper.add_component(rect{idx},ecs_component.Storage({comps['storage']}))\n"
    gen += identation + f"esper.add_component(rect{idx},ecs_component.RectShape())\n"
    if "id" in comps:  gen += identation + f"esper.add_component(rect{idx},ecs_component.IdComponent('{comps['id']}'))\n"
    
    parser.parse(comps["script"])
    funcs += transpiler.transpile(parser.program,prefix_functions=f"_{idx}_")
    for func in transpiler.funcs:
        if "on_update" in func:
           gen += identation + f"esper.add_component(rect{idx},ecs_component.UpdateCallback(callback = _{idx}_on_update))\n"
        elif "on_init" in func:
            gen += identation + f"esper.add_component(rect{idx},ecs_component.InitCallback(callback = _{idx}_on_init))\n"
            

gen += identation + f"scene =  esper.create_entity()\n"
comps = src['comps']
gen += identation + f"esper.add_component(scene,ecs_component.Size({comps['size']['w']},{comps['size']['h']}))\n"
gen += identation + f"esper.add_component(scene,ecs_component.Color({comps['color']['r']},{comps['color']['g']},{comps['color']['b']}))\n"
gen += identation + f"esper.add_component(scene,ecs_component.RectShape())\n"
if "id" in comps:  gen += identation + f"esper.add_component(scene,ecs_component.IdComponent('{comps['id']}'))\n"

parser.parse(comps["script"])
funcs += transpiler.transpile(parser.program,prefix_functions=f"")
for func in transpiler.funcs:
    if "on_update" in func:
       gen += identation + f"esper.add_component(scene,ecs_component.UpdateCallback(callback = on_update))\n"
    elif "on_init" in func:
        gen += identation + f"esper.add_component(scene,ecs_component.InitCallback(callback = on_init))\n"



app_configs = f"""
engine.width  = {src["comps"]['size']['w']}
engine.height  = {src["comps"]['size']['h']}
engine.background_color = {(src["comps"]['color']['r'],src["comps"]['color']['g'],src["comps"]['color']['b'])}
"""

# combine code
src = f"""
from engine import *
import pygame
import ecs.ecs_component as  ecs_component
import ecs.ecs_system as  ecs_system
import esper
from op_lang.build.lookup_tables import *
from op_lang.build.functions import *
from random import *
from math import *

entities = {"{}"}


# generated funcs
{funcs}


# generated code
def init():
\tesper.add_processor(ecs_system.RectRendererSystem())  
\tesper.add_processor(ecs_system.InitCallbackSystem())    
\tesper.add_processor(ecs_system.UpdateCallbackSystem())    
{gen}


def process():
    esper.process()
    
{app_configs}
engine.init_callback = init 
engine.process_callback = process 
engine.init()


engine.run()
"""


with open("gen-program.py","w") as f:
    f.write(src)

import os
os.system("python3 gen-program.py")








