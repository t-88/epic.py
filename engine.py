import pygame
import esper

class Engine:
    def __init__(self):
        self.is_running = True
        self.display = None
        self.process_callback = None
        
    def init(self):
        pygame.init()
        self.display = pygame.display.set_mode((600,400))
        
    def run(self):
        while self.is_running:
            for event in pygame.event.get():
                if event.type == pygame.QUIT:
                    self.is_running = False
                elif event.type == pygame.KEYDOWN:
                    if event.key == pygame.K_ESCAPE:
                        self.is_running = False
        
            
            if self.process_callback: self.process_callback()

            self.display.fill((0,0,0))
            pygame.display.flip()     
            
engine = Engine()
