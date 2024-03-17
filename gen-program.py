
from engine import *
import pygame
import ecs.ecs_component as  ecs_component
import ecs.ecs_system as  ecs_system
import esper
from op_lang.build.lookup_tables import *
from op_lang.build.functions import *

from random import *
from math import *

# generated funcs
def _0_on_update(ID):
	pos = get_component(ID,Components["Position"])
	if (pos.x > 0):
		if (is_pressed(Keys["Left"])):
			pos.x = pos.x - 1 / 2
	if (pos.x < 340):
		if (is_pressed(Keys["Right"])):
			pos.x = pos.x + 1 / 2
	if (is_pressed(Keys["Down"])):
		pos.y = pos.y + 1 / 2
	if (is_pressed(Keys["Up"])):
		pos.y = pos.y - 1 / 2
def _1_on_update(ID):
	pos = get_component(ID,Components["Position"])
	data = get_component(ID,Components["Storage"])
	data.collided = 0
	if (pos.x < 0):
		r_factor = randint(-2,2)
		r_factor = r_factor / 100
		data.x_dir = data.x_dir * -1 + r_factor
	if (pos.x > 400 - 15):
		r_factor = randint(-2,2)
		r_factor = r_factor / 100
		data.x_dir = data.x_dir * -1 + r_factor
	if (pos.y < 0):
		r_factor = randint(-2,2)
		r_factor = r_factor / 100
		data.y_dir = data.y_dir * -1 + r_factor
	if (pos.y > 600 - 15):
		r_factor = randint(-2,2)
		r_factor = r_factor / 100
		data.y_dir = data.y_dir * -1 + r_factor
		clear_entities()
		init()
	player_id = get_entity_by_id("player")
	player_pos = get_component(player_id,Components["Position"])
	player_size = get_component(player_id,Components["Size"])
	size = get_component(ID,Components["Size"])
	if (AABB(player_pos.x,player_pos.y,player_size.w,player_size.h,pos.x + data.x_dir,pos.y + data.y_dir,size.w,size.h)):
		done = 0
		if (pos.x + data.x_dir < player_pos.x):
			pos.x = player_pos.x - size.w
			data.x_dir = data.x_dir * -1
			done = 1
		if (done == 0):
			if (pos.x + data.x_dir + size.w > player_pos.x + player_size.w):
				pos.x = player_pos.x + player_size.w
				data.x_dir = data.x_dir * -1
				done = 1
		data.y_dir = data.y_dir * -1
	sqrted = sqrt(data.x_dir * data.x_dir + data.y_dir * data.y_dir)
	pos.x = pos.x + (data.x_dir / sqrted) * 1 / 3
	pos.y = pos.y + (data.y_dir / sqrted) * 1 / 3
def app_on_block_update(ID):
	pos = get_component(ID,Components["Position"])
	size = get_component(ID,Components["Size"])
	ball_id = get_entity_by_id("ball")
	ball_pos = get_component(ball_id,Components["Position"])
	ball_size = get_component(ball_id,Components["Size"])
	ball_data = get_component(ball_id,Components["Storage"])
	if (ball_data.collided == 0):
		if (AABB(ball_pos.x,ball_pos.y,ball_size.w,ball_size.h,pos.x,pos.y,size.w,size.h)):
			ball_data.y_dir = ball_data.y_dir * -1
			ball_data.collided = 1
			remove_entity(ID)
def app_on_init():
	for i in range(0,10):
		for j in range(0,5):
			id = create_entity(x = i * 30 + 60,y = j * 30 + 10,w = 29,h = 29,r = 20,g = 120,b = 255,shape = "Rect",on_update = app_on_block_update)
def app_on_update():
	pass



# generated code
def init():
	app_on_init()
	rect0 =  esper.create_entity()
	esper.add_component(rect0,ecs_component.Position(180,580))
	esper.add_component(rect0,ecs_component.Size(60,15))
	esper.add_component(rect0,ecs_component.Color(255,125,0))
	esper.add_component(rect0,ecs_component.Storage({}))
	esper.add_component(rect0,ecs_component.IdComponent('player'))
	esper.add_component(rect0,ecs_component.RectShape())
	esper.add_component(rect0,ecs_component.UpdateCallback(callback = _0_on_update))
	rect1 =  esper.create_entity()
	esper.add_component(rect1,ecs_component.Position(200,300))
	esper.add_component(rect1,ecs_component.Size(15,15))
	esper.add_component(rect1,ecs_component.Color(255,50,50))
	esper.add_component(rect1,ecs_component.Storage({'x_dir': 0.2, 'y_dir': 0.2, 'collided': 0}))
	esper.add_component(rect1,ecs_component.IdComponent('ball'))
	esper.add_component(rect1,ecs_component.RectShape())
	esper.add_component(rect1,ecs_component.UpdateCallback(callback = _1_on_update))
	esper.add_processor(ecs_system.RectRendererSystem())
	esper.add_processor(ecs_system.UpdateCallbackSystem())


def process():
    esper.process()
    

engine.width  = 400
engine.height  = 600
engine.update = app_on_update

engine.init_callback = init 
engine.process_callback = process 
engine.init()


engine.run()
