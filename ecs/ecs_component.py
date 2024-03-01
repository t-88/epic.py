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
        