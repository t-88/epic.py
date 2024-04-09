import esper
import pygame
from .lookup_tables import *
from ecs.ecs_system import *

def get_component(ID,comp_typ):
    if comp_typ.__name__ == "Storage":
        return esper.component_for_entity(ID,comp_typ).data

    return esper.component_for_entity(ID,comp_typ)

def remove_entity(ID):
    esper.delete_entity(ID)


def clear_entities():
    esper.clear_database()
    esper.remove_processor(UpdateCallbackSystem)
    esper.remove_processor(RectRendererSystem)
        
def get_entity_by_id(ID):
    for ent , (idComp, ) in esper.get_components(IdComponent):
        if idComp.id == ID:
            return ent
    return None


def AABB(x1,y1,w1,h1,x2,y2,w2,h2):
    return x1 + w1 > x2 and y1 + h1 > y2 and x2 + w2 > x1 and y2 + h2 > y1
    


def is_pressed(key):
    return pygame.key.get_pressed()[key]


def create_entity(x = -1, y = -1 , w = -1, h = -1,r = -1,g = -1,b = -1,on_init = None,on_update = None):
    id = esper.create_entity()
    
    # position
    if x != -1 or y != -1:
        if x == -1: x = 0
        if y == -1: y = 0
        esper.add_component(id,Components["Position"](x,y))    

    # size
    if w != -1 or h != -1:
        if w == -1: w = 0
        if h == -1: h = 0
        esper.add_component(id,Components["Size"](w,h))    
        
    # color
    if r != -1 or g != -1 or b != -1:
        if r == -1: r = 0
        if g == -1: g = 0
        if b == -1: b = 0
        esper.add_component(id,Components["Color"](r,g,b))    
        
        
    esper.add_component(id,Components["Rect"]())    
        

    # on update
    if on_update:
        esper.add_component(id,Components["UpdateCallback"](on_update))
    
    return id