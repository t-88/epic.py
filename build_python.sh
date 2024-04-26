#!/bin/sh


source py-env/env/bin/activate.sh
pyinstaller --onefile --noconfirm py-env/game.py
deactivate
echo "to run game type ./dist/game"