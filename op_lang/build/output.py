
# imports
from lookup_tables import *
from functions import *

# transpiled code
def on_input(ID):
	pos = get_component(ID,Components["Position"])
	if (is_pressed(Keys["Left"])):
		pos.x = pos.x + 1
	if (is_pressed(Keys["Right"])):
		pos.x = pos.x - 1
	for i in range(1,10):
		create_entity(x = 1,y = 1,w = 1,h = 1)

