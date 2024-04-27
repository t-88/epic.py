import pygame


def empty():
    pass
class Engine:
    def __init__(self):
        self.is_running = True
        self.display = None
        
        self.width = 600
        self.height = 400
        self.background_color = (0,0,0)
        
        pygame.init()
        self.clock = pygame.time.Clock()
        self.frame_entities = {}
        self.entities = {}
        self.pre_init = empty
        self.restart = False
                
    def init(self):
        if self.display == None:
            self.display = pygame.display.set_mode((self.width,self.height))
        self.pre_init()
        self.frame_entities = self.entities.copy()
        for uuid in self.frame_entities:
            self.frame_entities[uuid].init(uuid)
        
            
    def update(self):
        self.frame_entities = self.entities.copy()
        for uuid in self.frame_entities:
            self.frame_entities[uuid].update(uuid)
        
        if self.restart:
            engine.entities.clear()
            self.init()
            self.restart = False
            
    def render(self):
        for uuid in self.entities:
            self.entities[uuid].render()
        
        
    def run(self):
        while self.is_running:
            self.clock.tick(60)
            for event in pygame.event.get():
                if event.type == pygame.QUIT:
                    self.is_running = False
                elif event.type == pygame.KEYDOWN:
                    if event.key == pygame.K_ESCAPE:
                        self.is_running = False
        
            self.update()
            self.render()
            pygame.display.flip()     
            self.display.fill(self.background_color)
            
engine = Engine()
