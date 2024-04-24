from ecs.ecs_component import *
import pygame

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