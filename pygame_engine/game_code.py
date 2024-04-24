
from engine import *
import pygame
import ecs.ecs_component as  ecs_component
import ecs.ecs_system as  ecs_system
import esper
from op_lang.build.lookup_tables import *
from op_lang.build.functions import *
import random
import math

entities = {}
    
    
# generated funcs
def _0_on_init(ID):
  pass
def _0_on_update(ID):
    pos = sys__get_component(ID,Components.Position);
    size = sys__get_component(ID,Components.Size);
    data = sys__get_component(ID,Components.Storage);
    scene_id = sys__get_entity_by_id("Main-Scene");
    scene_size = sys__get_component(scene_id,Components.Size);
    ball_id = sys__get_entity_by_id("ball");
    ball_size = sys__get_component(ball_id,Components.Size);
    ball_pos = sys__get_component(ball_id,Components.Position);
    ball_data = sys__get_component(ball_id,Components.Storage);
    if(pos.x > 0):
        if(sys__is_pressed(Keys.Left)):
            pos.x = pos.x - data.speed;
            if(sys__AABB(pos.x,pos.y,size.w,size.h,ball_pos.x,ball_pos.y,ball_size.w,ball_size.h)):
                ball_pos.x = pos.x - ball_size.w;
                ball_data.x_dir = -1;



    if(pos.x < scene_size.w - size.w):
        if(sys__is_pressed(Keys.Right)):
            pos.x = pos.x + data.speed;
            if(sys__AABB(pos.x,pos.y,size.w,size.h,ball_pos.x,ball_pos.y,ball_size.w,ball_size.h)):
                ball_pos.x = pos.x + size.w;
                ball_data.x_dir = 1;
                print(ball_data)





def _1_on_init(ID):
  pass
def _1_on_update(ID):
    pos = sys__get_component(ID,Components.Position);
    size = sys__get_component(ID,Components.Size);
    data = sys__get_component(ID,Components.Storage);
    data.collided = 0;
    scene_id = sys__get_entity_by_id("Main-Scene");
    scene_size = sys__get_component(scene_id,Components.Size);
    if(pos.x < 0):
        data.x_dir = data.x_dir * -1 + (random.randint(-2,2) / 100);
        pos.x = 0;

    if(pos.x > scene_size.w - size.w):
        data.x_dir = data.x_dir * -1 + (random.randint(-2,2) / 100);
        pos.x = scene_size.w - size.w;

    if(pos.y < 0):
        data.y_dir = data.y_dir * -1 + (random.randint(-2,2) / 100);
        pos.y = 0;

    if(pos.y > scene_size.h - size.h):
        data.y_dir = 0;
        sys__clear_entities()
        sys__init()

    player_id = sys__get_entity_by_id("player");
    player_pos = sys__get_component(player_id,Components.Position);
    player_size = sys__get_component(player_id,Components.Size);
    sqrted = math.sqrt(data.x_dir * data.x_dir + data.y_dir * data.y_dir);
    data.vel_x = (data.x_dir / sqrted) * data.speed;
    data.vel_y = (data.y_dir / sqrted) * data.speed;
    if(sys__AABB(player_pos.x,player_pos.y,player_size.w,player_size.h,pos.x + data.vel_x,pos.y + data.vel_y,size.w,size.h)):
        if(sys__AABB(player_pos.x,player_pos.y,player_size.w,player_size.h,pos.x,pos.y + data.vel_y,size.w,size.h)):
            if(data.vel_y > 0):
                pos.y = player_pos.y - size.h;

            if(data.vel_y < 0):
                pos.y = player_pos.y + player_size.h;

            data.y_dir = data.y_dir * -1;
        elif(sys__AABB(player_pos.x,player_pos.y,player_size.w,player_size.h,pos.x + data.vel_x,pos.y,size.w,size.h)):
            if(data.vel_x > 0):
                pos.x = player_pos.x - size.w;

            if(data.vel_x < 0):
                pos.x = player_pos.x + player_size.w;

            data.x_dir = data.x_dir * -1;


    data.vel_x = (data.x_dir / sqrted) * data.speed;
    data.vel_y = (data.y_dir / sqrted) * data.speed;
    pos.x = pos.x + data.vel_x;
    pos.y = pos.y + data.vel_y;


def on_block_init(ID):
  pass
def on_block_update(ID):
    pos = sys__get_component(ID,Components.Position);
    size = sys__get_component(ID,Components.Size);
    ball_id = sys__get_entity_by_id("ball");
    ball_pos = sys__get_component(ball_id,Components.Position);
    ball_size = sys__get_component(ball_id,Components.Size);
    ball_data = sys__get_component(ball_id,Components.Storage);
    if(ball_data.collided == 0):
        if(sys__AABB(ball_pos.x + ball_data.vel_x,ball_pos.y + ball_data.vel_y,ball_size.w,ball_size.h,pos.x,pos.y,size.w,size.h)):
            if(sys__AABB(ball_pos.x + ball_data.vel_x,ball_pos.y,ball_size.w,ball_size.h,pos.x,pos.y,size.w,size.h)):
                ball_data.collided = 1;
                ball_data.x_dir = ball_data.x_dir * -1;

            if(ball_data.collided == 0):
                if(sys__AABB(ball_pos.x,ball_pos.y + ball_data.vel_y,ball_size.w,ball_size.h,pos.x,pos.y,size.w,size.h)):
                    ball_data.collided = 1;
                    ball_data.y_dir = ball_data.y_dir * -1;


            sys__remove_entity(ID)



def on_init(ID):
    i = 0
    while(i < 10):
      j = 0
      while(j < 10):
        id = sys__create_entity(on_block_init,on_block_update,x = 30 * i + 60,y = j * 30 + 10,w = 25,h = 25,r = 20,g = 120,b = 255);
        j = j + 1
      i = i + 1

def on_update(ID):
  pass


    
    
# generated code
def init():
	esper.add_processor(ecs_system.RectRendererSystem())  
	esper.add_processor(ecs_system.InitCallbackSystem())    
	esper.add_processor(ecs_system.UpdateCallbackSystem())    
	rect0 = esper.create_entity()
	esper.add_component(rect0,ecs_component.RectShape())
	esper.add_component(rect0,ecs_component.Position(160,580))
	esper.add_component(rect0,ecs_component.Size(80,20))
	esper.add_component(rect0,ecs_component.Color(125,10,50))
	esper.add_component(rect0,ecs_component.IdComponent("player"))
	esper.add_component(rect0,ecs_component.Storage([{ 'key' : 'speed' , 'val' : '5'  },]))
	esper.add_component(rect0,UpdateCallback(callback=_0_on_update))
	esper.add_component(rect0,InitCallback(callback=_0_on_init))
	rect1 = esper.create_entity()
	esper.add_component(rect1,ecs_component.RectShape())
	esper.add_component(rect1,ecs_component.Position(191,327))
	esper.add_component(rect1,ecs_component.Size(20,20))
	esper.add_component(rect1,ecs_component.Color(100,50,255))
	esper.add_component(rect1,ecs_component.IdComponent("ball"))
	esper.add_component(rect1,ecs_component.Storage([{ 'key' : 'collided' , 'val' : '0'  },{ 'key' : 'x_dir' , 'val' : '0.5'  },{ 'key' : 'y_dir' , 'val' : '-0.5'  },{ 'key' : 'speed' , 'val' : '5'  },{ 'key' : 'vel_x' , 'val' : '0'  },{ 'key' : 'vel_y' , 'val' : '0'  },]))
	esper.add_component(rect1,UpdateCallback(callback=_1_on_update))
	esper.add_component(rect1,InitCallback(callback=_1_on_init))
	scene = esper.create_entity()
	esper.add_component(scene,ecs_component.RectShape())
	esper.add_component(scene,ecs_component.Size(400,600))
	esper.add_component(scene,ecs_component.Color(255,255,255))
	esper.add_component(scene,ecs_component.IdComponent("Main-Scene"))
	esper.add_component(scene,UpdateCallback(callback=on_update))
	esper.add_component(scene,InitCallback(callback=on_init))



def process():
    esper.process()
    

engine.width = 400
engine.height = 600
engine.background_color = (255,255,255)

engine.init_callback = init 
engine.process_callback = process 
engine.init()


engine.run()
