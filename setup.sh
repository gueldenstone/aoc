#!/bin/bash

set -e

DAY=$(date +%d)
mkdir $DAY
aocdl -year 2024 -day $DAY -output $DAY/input
cargo init $DAY --name aoc_$DAY
cp main.rs.tmpl $DAY/src/main.rs
nvim $DAY
