#!/usr/bin/env bash
wasm-pack build
wasm-opt pkg/wasm_game_of_life_bg.wasm -o pkg/wasm_game_of_life.wasm -O --enable-mutable-globals
cd www
npm install