#!/bin/bash

function test_func() {
    echo "Hello"
    x=5
    if [ $x -eq 5 ]; then
        echo "x is 5"
    # Missing 'fi'
}