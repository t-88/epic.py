from ecs.ecs_component import *
import pygame

Components = {
    "Position" : Position,
    "Size" : Size,
    "Color" : Color,
    "Storage" : Storage,
    "UpdateCallback" : UpdateCallback,
    
    "Rect" : RectShape,
}
Keys = {
    "Left" : pygame.K_LEFT,
    "Right" : pygame.K_RIGHT,
    "Up" : pygame.K_UP,
    "Down" : pygame.K_DOWN,
    
}