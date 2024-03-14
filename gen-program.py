
from engine import *
import pygame
import ecs.ecs_component as  ecs_component
import ecs.ecs_system as  ecs_system
import esper
from op_lang.build.lookup_tables import *
from op_lang.build.functions import *

# generated funcs
def on_input(ID):
	pos = get_component(ID,Components["Position"])
	if (is_pressed(Keys["Left"])):
		pos.x = pos.x - 1
	if (is_pressed(Keys["Right"])):
		pos.x = pos.x + 1


# generated code
def init():
	rect0 =  esper.create_entity()
	esper.add_component(rect0,ecs_component.Position(10,10))
	esper.add_component(rect0,ecs_component.Size(20,20))
	esper.add_component(rect0,ecs_component.Color(255,0,0))
	esper.add_component(rect0,ecs_component.RectShape())
	esper.add_component(rect0,ecs_component.InputCallback(callback = on_input))
	esper.add_processor(ecs_system.RectRendererSystem())
	esper.add_processor(ecs_system.KeyboardInputSyatem())


def process():
    esper.process()
engine.init_callback = init 
engine.process_callback = process 
engine.init()
engine.run()
