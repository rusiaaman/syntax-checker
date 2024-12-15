<?php
function test($var) {
    echo "Hello" . $var;
    echo "Missing semicolon"
    return "test"
}

$x = test("World");
