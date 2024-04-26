
from engine.engine import * 
from engine.dotdict import *
import uuid
import pygame

def empty(ID):
    pass

class Rect:
    def __init__(self,init = empty,update = empty,x = 0 , y = 0 , w = 0, h = 0 , r = 0 , g = 0 , b = 0,id = None, storage = []):
        self.uuid = str(uuid.uuid4())


        self.pos = DotDict()
        self.pos.x = x
        self.pos.y = y
        self.size = DotDict()
        self.size.w = w
        self.size.h = h
        self.color = DotDict()
        self.color.r = r
        self.color.g = g
        self.color.b = b
        if not id: id = self.uuid
        self.id = id
        
        
        self.storage = DotDict()
        for i in range(len(storage)):
            self.storage[storage[i]["key"]] = float(storage[i]["val"])
        
        if init == None: init = empty
        if update == None: update = empty
        self.update = lambda ID: update(ID)
        self.init = lambda ID: init(ID)
        
    
    def get_component(self,comp_typ):
        if comp_typ == "Position": return self.pos
        elif comp_typ == "Size":   return self.size
        elif comp_typ == "Color":  return self.color
        elif comp_typ == "Storage": return self.storage
    def render(self):
        pygame.draw.rect(engine.display,[self.color.r,self.color.g,self.color.b],pygame.Rect(self.pos.x,self.pos.y,self.size.w,self.size.h))