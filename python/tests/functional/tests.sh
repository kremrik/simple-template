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
    expect="${1}"
    actual="${2}"
    fncname="${FUNCNAME[1]}"
    if [ "$expect" == "$actual" ]; then
        color_echo 'passed: ' "$GREEN"
        echo "$fncname"
    else
        color_echo 'failed: ' "$RED"
        echo "$fncname"
        diff -u <(echo "$expect") <(echo "$actual")
    fi
}

# tests
# ---------------------------------------------------------
function test_specify_all_params() {
    expect='<!DOCTYPE html>
<html lang="en">
  <head>
    <link rel="stylesheet" href="styles.css">
  </head>
  <div>
    <p>
      This is a test paragraph.
    </p>
  </div>
</html>'

    tmplts="tests/functional/templates"
    actual=$(cat ${tmplts}/base.html | htmlayout-d --head ${tmplts}/head.html --body ${tmplts}/body.html)
    assert_equal "$expect" "$actual"
}


function test_multi_pipe_params() {
    expect='<!DOCTYPE html>
<html lang="en">
  <head>
    <link rel="stylesheet" href="styles.css">
  </head>
  <div>
    <p>
      This is a test paragraph.
    </p>
  </div>
</html>'

    tmplts="tests/functional/templates"
    actual=$(cat ${tmplts}/base.html | htmlayout-d --head ${tmplts}/head.html | htmlayout-d --body ${tmplts}/body.html)
    assert_equal "$expect" "$actual"
}


function test_help() {
    expect='usage: htmlayout-d [-h] [--head HEAD] [--body BODY]

options:
  -h, --help   show this help message and exit
  --head HEAD  /path/to/head.html template
  --body BODY  path to body.html template file'

    tmplts="tests/functional/templates"
    actual=$(cat ${tmplts}/base.html | htmlayout-d --help)
    assert_equal "$expect" "$actual"
}


# run tests
# ---------------------------------------------------------
test_specify_all_params
test_multi_pipe_params
test_help
