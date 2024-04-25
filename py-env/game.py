
from engine import engine
from meta import *        
import random
import math

engine.entities = {}

    
# generated funcs
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



    
    
# generated code
def init():
	sys__create_entity(on_init,
                              on_update, 
                              x = 0, 
                              y = 0, 
                              w = 400, 
                              h = 600,
                              r = 255,
                              g = 255,
                              b = 255,
                              id='Main-Scene',
                              storage = [])
	sys__create_entity(_0_on_init,
                              _0_on_update, 
                              x = 160, 
                              y = 580, 
                              w = 80, 
                              h = 20,
                              r = 125,
                              g = 10,
                              b = 50,
                              id='player',
                              storage = [{ 'key' : 'speed' , 'val' : '5'  },])
	sys__create_entity(_1_on_init,
                              _1_on_update, 
                              x = 191, 
                              y = 327, 
                              w = 20, 
                              h = 20,
                              r = 100,
                              g = 50,
                              b = 255,
                              id='ball',
                              storage = [{ 'key' : 'collided' , 'val' : '0'  },{ 'key' : 'x_dir' , 'val' : '0.5'  },{ 'key' : 'y_dir' , 'val' : '-0.5'  },{ 'key' : 'speed' , 'val' : '5'  },{ 'key' : 'vel_x' , 'val' : '0'  },{ 'key' : 'vel_y' , 'val' : '0'  },])


    

engine.width = 400
engine.height = 600
engine.background_color = (255,255,255)


engine.width = 400
engine.height = 600
engine.pre_init = init
engine.init()
engine.run()

