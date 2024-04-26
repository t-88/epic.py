# Epic
Rust Compiler/Transpiler for the op scripting language.    
**I REGRET USING RUST**

## Requirements
run install.sh to install [rust](https://www.rust-lang.org/tools/install)


## Features
### Op Language
- fully working lexer, parser , compiler , transpiler
- basic error handling using a symbol analyser
- js, py transpiler
- a very basic and limited ffi system using "$." to access pre defined native language  

### Build
- generates wasm code for the js transpiler
- generates a python file with game logic
- builds the python game using pyinstaller


## Quick Start
- generate **src.json** file from the [epic-editor website](https://epic-editor.vercel.app/)
- move the json to the current directory 
```
    $ ./run.sh # building a standalone python executable using pyinstaller can take a while     
```
- your project will be in dist/game 


## Preview
`$ ./run.sh`
![expected after ./run.sh](preview/run_sh.png)    
</br> </br>
![working pong game](preview/pong_game.gif)


