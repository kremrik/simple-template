#!/usr/bin/env bash

GREEN="\e[0;92m"
RED="\e[0;91m"
RESET="\e[0m"

# helpers
# ---------------------------------------------------------
function color_echo() {
    inpt="${1}"
    color="${2}"
    printf "${color}${inpt}${RESET}"
}

function assert_equal() {
    gold="${1}"
    value="${2}"
    fncname="${FUNCNAME[1]}"
    if [ "$gold" == "$output" ]; then
        color_echo 'passed: ' "$GREEN"
        echo "$fncname"
    else
        color_echo 'failed: ' "$RED"
        echo "$fncname"
        echo "expected:"
        echo "${gold}"
        echo "received:"
        echo "${value}"
    fi
}

# tests
# ---------------------------------------------------------
function test() {
    gold=$(printf "+-----+\n| foo |\n+-----+")
    output=$(echo "foo" | box-it-up)
    assert_equal "$gold" "$output"
}


# run tests
# ---------------------------------------------------------
test
