#!/bin/sh
[ -z $1 ] && echo '$1 is URL' && exit 1
[ -z $2 ] && echo '$2 is Username' && exit 1
[ -z $3 ] && echo '$3 is Password' && exit 1
cd "$(dirname "$0")"
export URL=$1
export USERNAME=$2
export PASSWORD=$3

nvim -u app.vim --noplugin
