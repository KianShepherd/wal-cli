#!/bin/sh
### Created by ilsenatorov
### Change WALLPAPERDIR to the directory where you store wallpapers

BACKENDDIR=~/.config/wal-cli/backends/

if [ -z $@ ]
then
function get_themes()
{
    ls $BACKENDDIR
}
echo current; get_themes
else
    THEMES=$@
    if [ x"current" = x"${THEMES}" ]
    then
        exit 0
        #wal -i `cat ~/.cache/wal/wal` > /dev/null
    elif [ -n "${THEMES}" ]
    then
        # Backends colorz colorthierd wal haishoku
        wal-cli -b ${THEMES} > /dev/null
    fi
fi