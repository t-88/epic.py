
# imports
from lookup_tables import *
from functions import *

# transpiled code
def on_input():
	pos = get_component(Components["Position"])
	if (is_pressed(Keys["Left"])):
		pos.x = pos.x + 1
	if (is_pressed(Keys["Right"])):
		pos.x = pos.x - 1