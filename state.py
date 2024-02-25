import pygame

display = None


def init():
    global display

    pygame.init()
    display = pygame.display.set_mode((600,400))

