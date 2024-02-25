import pygame
import esper
import state
from ecs_component import *

class RectRendererSystem(esper.Processor):
    def process(self):
        for ent , (pos,size,color) in esper.get_components(Position,Size,Color):
            pygame.draw.rect(state.display,[color.r,color.g,color.b],pygame.Rect(pos.x,pos.y,size.w,size.h))

class ButtonSystem(esper.Processor):
    def process(self):        
        mouse_rect = pygame.Rect(pygame.mouse.get_pos()[0],pygame.mouse.get_pos()[1],1,1)
        
        for ent , (button) in esper.get_component(Button):
            if not button.callback :
                continue
            
            if not button.clicked and pygame.mouse.get_pressed()[0]: 
                button.clicked = True

                size =  esper.component_for_entity(ent,Size)
                pos  =  esper.component_for_entity(ent,Position)

                if mouse_rect.colliderect(pos.x,pos.y,size.w,size.h):
                    button.callback()
            elif not pygame.mouse.get_pressed()[0]:
                button.clicked = False
                