from engine import *
import pygame
import ecs.ecs_component as  ecs_component
import ecs.ecs_system as  ecs_system
import esper


def rect_input_callback(ent):
    pos =  esper.component_for_entity(ent,ecs_component.Position)
    if pygame.key.get_pressed()[pygame.K_d]:
        pos.x += 1
    elif pygame.key.get_pressed()[pygame.K_a]:
        pos.x -= 1


def init():
    rect =  esper.create_entity()
    esper.add_component(rect,ecs_component.Position(100,100))
    esper.add_component(rect,ecs_component.Size(100,100))
    esper.add_component(rect,ecs_component.Color(255,0,0))
    esper.add_component(rect,ecs_component.RectShape())
    esper.add_component(rect,ecs_component.InputCallback(callback = rect_input_callback))
    
    esper.add_processor(ecs_system.RectRendererSystem())
    esper.add_processor(ecs_system.KeyboardInputSyatem())



def process():
    esper.process()
engine.init_callback = init 
engine.process_callback = process 
engine.init()
engine.run()