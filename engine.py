import pygame
import esper

class Engine:
    def __init__(self):
        self.is_running = True
        self.display = None
        self.init_callback = None
        self.process_callback = None
        
        self.width = 600
        self.height = 400
        
    def init(self):
        pygame.init()
        self.display = pygame.display.set_mode((self.width,self.height))
        
        if self.init_callback: self.init_callback()
    
        
    def update(self):
        pass
        
    def run(self):
        while self.is_running:
            for event in pygame.event.get():
                if event.type == pygame.QUIT:
                    self.is_running = False
                elif event.type == pygame.KEYDOWN:
                    if event.key == pygame.K_ESCAPE:
                        self.is_running = False
        
            self.update()
            if self.process_callback: self.process_callback()

            pygame.display.flip()     
            self.display.fill((0,0,0))
            
engine = Engine()
