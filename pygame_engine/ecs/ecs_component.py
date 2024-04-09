from .types import *

class Position:
    def __init__(self,x = 0 , y = 0):
        self.x = x
        self.y = y
class Size:
    def __init__(self,w = 0 , h = 0):
        self.w = w
        self.h = h
class Color:
    def __init__(self,r = 255 , g = 255, b = 255):
        self.r = r
        self.g = g        
        self.b = b  
class Button:
    def __init__(self,callback = None):
        self.callback = callback
        self.clicked = False
        
class Storage:
    def __init__(self,data = []):
        self.data = DotDict()
        for i in data:
            self.data[i['key']] = float(i['val'])

        
class RectShape:
    def __init__(self):
        pass
                
class UpdateCallback:
    def __init__(self,callback = None):
        self.callback = callback

class InitCallback:
    def __init__(self,callback = None):
        self.called = False
        self.callback = callback



class IdComponent:
    def __init__(self,id = None):
        self.id = id


from pwn import *
