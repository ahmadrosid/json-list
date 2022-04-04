#!/bin/sh
echo '[
    1,2,3,4,5
]' | cargo run -q

echo '[
    "first",
    "love",
]' | cargo run -q

cargo run -q '[3,2,1]'

cargo run -q sample.json