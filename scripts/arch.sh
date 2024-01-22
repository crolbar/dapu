#!/bin/sh
output=$(dapu -o)


if [[ -e $output ]]; then
    if [[ -d $output ]]; then
        cd $output
    else 
        cd $(dirname $output)
    fi
    if [[ $1 != "cd" ]]; then
        nvim $output
    fi
fi
