#!/bin/sh
output=$(dapu -o)


if [[ -e $output ]]; then
    if [[ -d $output ]]; then
        cd $output
    else 
        cd $(dirname $output)
    fi
    if [[ $1 != "cd" ]]; then
        if [[ -e $output/shell.nix ]]; then
            nix-shell $output/shell.nix --command "nvim $output"
        elif [[ -e $output/flake.nix ]]; then
            nix develop --command nvim .
        else
            nvim $output
        fi
    else
        if [[ -e $output/shell.nix ]]; then
            nix-shell $output/shell.nix --command "zsh"
        elif [[ -e $output/flake.nix ]]; then
            nix develop --command zsh
        fi
    fi
fi
