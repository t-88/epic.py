
from engine import *
import pygame
import ecs.ecs_component as  ecs_component
import ecs.ecs_system as  ecs_system
import esper
from op_lang.build.lookup_tables import *
from op_lang.build.functions import *
from random import *
from math import *

entities = {}


# generated funcs
def _0_on_update(ID):
	pos = get_component(ID,Components["Position"])
	size = get_component(ID,Components["Size"])
	data = get_component(ID,Components["Storage"])
	scene_id = get_entity_by_id("Main-Scene")
	scene_size = get_component(scene_id,Components["Size"])
	ball_id = get_entity_by_id("ball")
	ball_size = get_component(ball_id,Components["Size"])
	ball_pos = get_component(ball_id,Components["Position"])
	ball_data = get_component(ball_id,Components["Storage"])
	if (pos.x > 0):
		if (is_pressed(Keys["Left"])):
			pos.x = pos.x - data.speed
			if (AABB(pos.x,pos.y,size.w,size.h,ball_pos.x,ball_pos.y,ball_size.w,ball_size.h)):
				ball_pos.x = pos.x - ball_size.w
				ball_data.x_dir = -1
	if (pos.x < scene_size.w - size.w):
		if (is_pressed(Keys["Right"])):
			pos.x = pos.x + data.speed
			if (AABB(pos.x,pos.y,size.w,size.h,ball_pos.x,ball_pos.y,ball_size.w,ball_size.h)):
				ball_pos.x = pos.x + size.w
				ball_data.x_dir = 1
				log(ball_data)
def _1_on_update(ID):
	pos = get_component(ID,Components["Position"])
	size = get_component(ID,Components["Size"])
	data = get_component(ID,Components["Storage"])
	data.collided = 0
	scene_id = get_entity_by_id("Main-Scene")
	scene_size = get_component(scene_id,Components["Size"])
	if (pos.x < 0):
		r_factor = randint(-2,2)
		r_factor = r_factor / 100
		data.x_dir = data.x_dir * -1 + r_factor
	if (pos.x > scene_size.w - size.w):
		r_factor = randint(-2,2)
		r_factor = r_factor / 100
		data.x_dir = data.x_dir * -1 + r_factor
	if (pos.y < 0):
		r_factor = randint(-2,2)
		r_factor = r_factor / 100
		data.y_dir = data.y_dir * -1 + r_factor
	if (pos.y > scene_size.h - size.h):
		r_factor = randint(-2,2)
		r_factor = r_factor / 100
		data.y_dir = data.y_dir * -1 + r_factor
		clear_entities()
		init()
	player_id = get_entity_by_id("player")
	player_pos = get_component(player_id,Components["Position"])
	player_size = get_component(player_id,Components["Size"])
	sqrted = sqrt(data.x_dir * data.x_dir + data.y_dir * data.y_dir)
	data.vel_x = (data.x_dir / sqrted) * data.speed
	data.vel_y = (data.y_dir / sqrted) * data.speed
	if (AABB(player_pos.x,player_pos.y,player_size.w,player_size.h,pos.x + data.vel_x,pos.y + data.vel_y,size.w,size.h)):
		x_collide = 1
		if (AABB(player_pos.x,player_pos.y,player_size.w,player_size.h,pos.x,pos.y + data.vel_y,size.w,size.h)):
			if (data.vel_y > 0):
				pos.y = player_pos.y - size.h
			if (data.vel_y < 0):
				pos.y = player_pos.y + player_size.h
			data.y_dir = data.y_dir * -1
			x_collide = 0
		if (x_collide):
			if (AABB(player_pos.x,player_pos.y,player_size.w,player_size.h,pos.x + data.vel_x,pos.y,size.w,size.h)):
				if (data.vel_x > 0):
					pos.x = player_pos.x - size.w
				if (data.vel_x < 0):
					pos.x = player_pos.x + player_size.w
				data.x_dir = data.x_dir * -1
	data.vel_x = (data.x_dir / sqrted) * data.speed
	data.vel_y = (data.y_dir / sqrted) * data.speed
	pos.x = pos.x + data.vel_x
	pos.y = pos.y + data.vel_y
def on_block_init(ID):
	pass
def on_block_update(ID):
	pos = get_component(ID,Components["Position"])
	size = get_component(ID,Components["Size"])
	ball_id = get_entity_by_id("ball")
	ball_pos = get_component(ball_id,Components["Position"])
	ball_size = get_component(ball_id,Components["Size"])
	ball_data = get_component(ball_id,Components["Storage"])
	if (ball_data.collided == 0):
		if (AABB(ball_pos.x + ball_data.vel_x,ball_pos.y + ball_data.vel_y,ball_size.w,ball_size.h,pos.x,pos.y,size.w,size.h)):
			if (AABB(ball_pos.x + ball_data.vel_x,ball_pos.y,ball_size.w,ball_size.h,pos.x,pos.y,size.w,size.h)):
				ball_data.collided = 1
				ball_data.x_dir = ball_data.x_dir * -1
			if (ball_data.collided == 0):
				if (AABB(ball_pos.x,ball_pos.y + ball_data.vel_y,ball_size.w,ball_size.h,pos.x,pos.y,size.w,size.h)):
					ball_data.collided = 1
					ball_data.y_dir = ball_data.y_dir * -1
			remove_entity(ID)
def on_init(ID):
	for i in range(0,10):
		for j in range(0,5):
			id = create_entity(i * 30 + 60,j * 30 + 10,29,29,20,120,255,on_block_init,on_block_update)



# generated code
def init():
	esper.add_processor(ecs_system.RectRendererSystem())  
	esper.add_processor(ecs_system.InitCallbackSystem())    
	esper.add_processor(ecs_system.UpdateCallbackSystem())    
	rect0 =  esper.create_entity()
	esper.add_component(rect0,ecs_component.Position(156,473))
	esper.add_component(rect0,ecs_component.Size(80,20))
	esper.add_component(rect0,ecs_component.Color(125,10,50))
	esper.add_component(rect0,ecs_component.Storage([{'key': 'speed', 'val': '5'}]))
	esper.add_component(rect0,ecs_component.RectShape())
	esper.add_component(rect0,ecs_component.IdComponent('player'))
	esper.add_component(rect0,ecs_component.UpdateCallback(callback = _0_on_update))
	rect1 =  esper.create_entity()
	esper.add_component(rect1,ecs_component.Position(191,327))
	esper.add_component(rect1,ecs_component.Size(20,20))
	esper.add_component(rect1,ecs_component.Color(100,50,255))
	esper.add_component(rect1,ecs_component.Storage([{'key': 'collided', 'val': '0'}, {'key': 'x_dir', 'val': '0.5'}, {'key': 'y_dir', 'val': '-0.5'}, {'key': 'speed', 'val': '5'}, {'key': 'vel_x', 'val': '0'}, {'key': 'vel_y', 'val': '0'}]))
	esper.add_component(rect1,ecs_component.RectShape())
	esper.add_component(rect1,ecs_component.IdComponent('ball'))
	esper.add_component(rect1,ecs_component.UpdateCallback(callback = _1_on_update))
	scene =  esper.create_entity()
	esper.add_component(scene,ecs_component.Size(400,600))
	esper.add_component(scene,ecs_component.Color(255,255,255))
	esper.add_component(scene,ecs_component.RectShape())
	esper.add_component(scene,ecs_component.IdComponent('Main-Scene'))
	esper.add_component(scene,ecs_component.InitCallback(callback = on_init))



def process():
    esper.process()
    

engine.width  = 400
engine.height  = 600
engine.background_color = (255, 255, 255)

engine.init_callback = init 
engine.process_callback = process 
engine.init()


engine.run()
