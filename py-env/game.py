
from engine.engine import engine
from engine.meta import *        
import random
import math

engine.entities = {}

    
# generated funcs

    
    
# generated code
def init():
	sys__create_entity(None,
                              None, 
                              x = 0, 
                              y = 0, 
                              w = 600, 
                              h = 600,
                              r = 255,
                              g = 255,
                              b = 255,
                              id='Scene',
                              storage = [])
	sys__create_entity(None,
                              None, 
                              x = 200, 
                              y = 479, 
                              w = 50, 
                              h = 50,
                              r = 125,
                              g = 125,
                              b = 125,
                              id='Rect',
                              storage = [])


    

engine.width = 600
engine.height = 600
engine.background_color = (255,255,255)


engine.pre_init = init
engine.init()
engine.run()

