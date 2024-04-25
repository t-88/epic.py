import esper
import pygame
from ecs.ecs_system import *
from ecs.ecs_component import *

Components = DotDict()
Components["Position"] = Position
Components["Size"] = Size
Components["Color"] = Color
Components["Storage"] = Storage
Components["UpdateCallback"] = UpdateCallback
Components["Rect"] = RectShape

Keys = DotDict()
Keys["Left"] = pygame.K_LEFT
Keys["Right"] = pygame.K_RIGHT
Keys["Up"] = pygame.K_UP
Keys["Down"] = pygame.K_DOWN

def sys__get_component(ID,comp_typ):
    if comp_typ.__name__ == "Storage":
        return esper.component_for_entity(ID,comp_typ).data

    return esper.component_for_entity(ID,comp_typ)

def sys__remove_entity(ID):
    esper.delete_entity(ID)


def sys__clear_entities():
    esper.clear_database()
    esper.remove_processor(UpdateCallbackSystem)
    esper.remove_processor(RectRendererSystem)
        
def sys__get_entity_by_id(ID):
    for ent , (idComp, ) in esper.get_components(IdComponent):
        if idComp.id == ID:
            return ent
    return None


def sys__AABB(x1,y1,w1,h1,x2,y2,w2,h2):
    return x1 + w1 > x2 and y1 + h1 > y2 and x2 + w2 > x1 and y2 + h2 > y1
    


def sys__is_pressed(key):
    return pygame.key.get_pressed()[key]


def sys__create_entity(on_init,on_update,x = -1, y = -1 , w = -1, h = -1,r = -1,g = -1,b = -1):
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

def sys__init():
    pass