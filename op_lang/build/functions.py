import esper
import pygame

def get_component(ID,comp_typ):
    return esper.component_for_entity(ID,comp_typ)

def is_pressed(key):
    return pygame.key.get_pressed()[key]
