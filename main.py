import pygame
import state
from ecs_component import *
from ecs_system import *
import esper
import op_parser


state.init()

src = """
    button( pos(50 50) size(90 90) color($.color.r $.color.g $.color.b))
"""
    
context = {
    "color" : {
        "r" : 200,
        "g" : 100,
        "b" : 50,
    },
}
parser =  op_parser.Parser()
parser.parse(src,context)

generator = op_parser.Generator()
generator.generate(parser.nodes)

esper.add_processor(RectRendererSystem())
esper.add_processor(ButtonSystem())
running = True
while running:
    for event in pygame.event.get():
        if event.type == pygame.QUIT:
            running = False
        elif event.type == pygame.KEYDOWN:
            if event.key == pygame.K_ESCAPE:
                running = False

    state.display.fill((0,0,0))
    
    esper.process()
    pygame.display.flip()



