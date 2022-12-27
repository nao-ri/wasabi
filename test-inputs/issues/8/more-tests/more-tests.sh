#!/usr/bin/env bash

set -e

for wat_file in *.wat; do 
  wat2wasm $wat_file
done

for wasm_file in *.wasm; do 
  wasabi $wasm_file --hooks=begin,end wasabi
done

cp wasabi/*.js .
cp wasabi/*.wasm .

for wasm_file in *.wasm; do 
  echo testing $wasm_file ...
  $V8_HOME/out/x64.release/d8 assert-begin-end-matching.js -- ${wasm_file%.wasm}
done

echo "+---------+"
echo "| SUCCESS |"
echo "+---------+"


