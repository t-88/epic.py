from op_lang import *


src_json = {
    "entities" : [
        {
            "pos" : {"x" : 180, "y" : 580},
            "size" : {"w" : 60, "h" : 15},
            "color": {"r" : 255, "g" : 125, "b" : 0},
            "storage": {},
            "id": "player",
            "script" : """
                    func on_update(ID) {
                        pos = get_component(ID,Components.Position);    
                        if(pos.x > 0) {
                            if(is_pressed(Keys.Left)) {
                                pos.x = pos.x - 1 / 2;
                            }
                        }
                        if(pos.x < 340) {
                            if(is_pressed(Keys.Right)) {
                                pos.x = pos.x + 1 / 2;
                            }    
                        }
                        if(is_pressed(Keys.Down)) {
                            pos.y = pos.y + 1 / 2;
                        }    
                        if(is_pressed(Keys.Up)) {
                            pos.y = pos.y - 1 / 2;
                        }    
                        
                    }        
            """,
        },
        {
            "pos" : {"x" : 200, "y" : 300},
            "size" : {"w" : 15, "h" : 15},
            "color": {"r" : 255, "g" : 50, "b" : 50},
            "storage": {
                "x_dir": 0.2,
                "y_dir": 0.2,
                "collided": 0,
            },
            "id" : "ball",
            "script" : """
                    func on_update(ID) {
                        pos = get_component(ID,Components.Position);  
                        data = get_component(ID,Components.Storage);  
                        data.collided = 0;
                        
                        if(pos.x < 0) {
                            r_factor = randint(-2,2);
                            r_factor = r_factor / 100;                   
                            data.x_dir =  data.x_dir * -1 + r_factor; 
                        } 
                        if(pos.x > 400 - 15) {
                            r_factor = randint(-2,2);
                            r_factor = r_factor / 100;                   
                            data.x_dir =  data.x_dir * -1 + r_factor; 
                        }                         


                        if(pos.y < 0) {
                            r_factor = randint(-2,2);
                            r_factor = r_factor / 100;                   
                            data.y_dir =  data.y_dir * -1 + r_factor; 
                        } 
                        if(pos.y > 600 - 15) {
                            r_factor = randint(-2,2);
                            r_factor = r_factor / 100;                   
                            data.y_dir =  data.y_dir * -1 + r_factor; 
                            
                            
                            clear_entities();
                            init();
                        }      
                        
                        player_id = get_entity_by_id("player");
                        player_pos = get_component(player_id,Components.Position);
                        player_size = get_component(player_id,Components.Size);
                        size = get_component(ID,Components.Size);
                        if(AABB(
                            player_pos.x,player_pos.y,player_size.w,player_size.h,
                            pos.x + data.x_dir,pos.y + data.y_dir,size.w,size.h,
                        )) {
                            done = 0;
                            
                            
                            if(pos.x + data.x_dir < player_pos.x) {
                                pos.x = player_pos.x - size.w;
                                data.x_dir =  data.x_dir * -1; 
                                done = 1;
                            }
                            if(done == 0) {
                                if(pos.x + data.x_dir + size.w > player_pos.x + player_size.w) {
                                    pos.x  = player_pos.x + player_size.w;
                                    data.x_dir =  data.x_dir * -1; 
                                    done = 1;
                                }
                            }
                            data.y_dir =  data.y_dir * -1; 
                        }
                        
                        sqrted = sqrt(data.x_dir*data.x_dir+data.y_dir*data.y_dir);   
                        pos.x = pos.x + (data.x_dir / sqrted) * 1/3;
                        pos.y = pos.y + (data.y_dir / sqrted) * 1/3;
                        
                    }        
            """,
        },       
    ]
}
  

src_json["app"] = {
    "size"  : {"w" : 400 , "h" : 600},
    "script" : """
        func on_block_update(ID) {
            pos = get_component(ID,Components.Position);  
            size = get_component(ID,Components.Size);  
            
            ball_id = get_entity_by_id("ball");
            ball_pos = get_component(ball_id,Components.Position);
            ball_size = get_component(ball_id,Components.Size);
            ball_data = get_component(ball_id,Components.Storage);

            if(ball_data.collided == 0) {
                if(AABB(
                    ball_pos.x,ball_pos.y,ball_size.w,ball_size.h,
                    pos.x,pos.y,size.w,size.h,
                )) {
                    ball_data.y_dir =  ball_data.y_dir * -1; 
                    ball_data.collided = 1;
                    remove_entity(ID);
                }
                
            }
        }
    
        func on_init() {
            for(i in 0..10) {
                for(j in 0..5) {
                    id = create_entity(
                        x = i * 30 + 60,
                        y = j * 30 + 10,
                        w = 29,
                        h = 29,
                        r = 20,
                        g = 120,
                        b = 255,
                        shape = "Rect",
                        on_update = app_on_block_update
                    );
                }
            }
        }
        func on_update() {
            
        }
    """
}


parser =  op_parser.OPParser()
transpiler = op_transpiler.OPTraspiler()


identation = "\t"
gen = "" 
funcs = ""
config = ""



# generate entities
for (idx , entity) in enumerate(src_json["entities"]):
    gen += identation + f"rect{idx} =  esper.create_entity()\n"
    gen += identation + f"esper.add_component(rect{idx},ecs_component.Position({entity['pos']['x']},{entity['pos']['y']}))\n"
    gen += identation + f"esper.add_component(rect{idx},ecs_component.Size({entity['size']['w']},{entity['size']['h']}))\n"
    gen += identation + f"esper.add_component(rect{idx},ecs_component.Color({entity['color']['r']},{entity['color']['g']},{entity['color']['b']}))\n"
    gen += identation + f"esper.add_component(rect{idx},ecs_component.Storage({entity['storage']}))\n"
    
    
    if "id" in entity: 
        gen += identation + f"esper.add_component(rect{idx},ecs_component.IdComponent('{entity['id']}'))\n"
    
    gen += identation + f"esper.add_component(rect{idx},ecs_component.RectShape())\n"
    gen += identation + f"esper.add_component(rect{idx},ecs_component.UpdateCallback(callback = _{idx}_on_update))\n"
    
    
    
    parser.parse(entity["script"])
    funcs += transpiler.transpile(parser.program,prefix_functions=f"_{idx}_")
    
    
# generate systems 
gen += identation + "esper.add_processor(ecs_system.RectRendererSystem())\n"
gen += identation + "esper.add_processor(ecs_system.UpdateCallbackSystem())\n"



# config app

parser.parse(src_json["app"]["script"])
funcs += transpiler.transpile(parser.program,prefix_functions="app_")

config = f"""
engine.width  = {src_json['app']['size']['w']}
engine.height  = {src_json['app']['size']['h']}
engine.update = app_on_update
"""


# combine code
src = f"""
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
{funcs}


# generated code
def init():
\tapp_on_init()
{gen}

def process():
    esper.process()
    
{config}
engine.init_callback = init 
engine.process_callback = process 
engine.init()


engine.run()
"""


with open("gen-program.py","w") as f:
    f.write(src)



import os
os.system("python3 gen-program.py")