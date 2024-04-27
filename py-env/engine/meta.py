import pygame
from engine.rect import Rect
from engine.engine import engine
from engine.dotdict import *


Components = DotDict()
Components["Position"] = "Position"
Components["Size"] =     "Size"
Components["Color"] =    "Color"
Components["Storage"] =  "Storage"

Keys = DotDict()
Keys["Left"] =  pygame.K_LEFT
Keys["Right"] = pygame.K_RIGHT
Keys["Up"] =    pygame.K_UP
Keys["Down"] =  pygame.K_DOWN


def sys__create_entity(on_init,on_update,x = 0, y = 0 , w = 0, h = 0,r = 0,g = 0,b = 0,id = None, storage = []):
    entity = Rect(on_init,on_update,x,y,w,h,r,g,b,id,storage)
    engine.entities[entity.uuid] = entity
def sys__remove_entity(ID):
    del engine.entities[ID]
def sys__get_entity_by_id(ID):
    for uuid in engine.frame_entities:
        if engine.frame_entities[uuid].id == ID:
            return uuid
    print(f"entity with id {ID} not found")
    return None

def sys__get_component(ID,comp_typ):
    return engine.frame_entities[ID].get_component(comp_typ)

def sys__is_pressed(key):
    return pygame.key.get_pressed()[key]


# general
def sys__AABB(x1,y1,w1,h1,x2,y2,w2,h2):
    return x1 + w1 > x2 and y1 + h1 > y2 and x2 + w2 > x1 and y2 + h2 > y1

def sys__clear_entities():
    engine.entities.clear()
def sys__restart():
    engine.restart = True
    
    
    