#!/bin/sh

CWD=$(pwd)
DST="$CWD/fixtures/t1/proc"

(cd /proc && (
  for p in */status; do
    bn=$(dirname "$p")
    fn=$(basename "$p")
    #echo "$bn" " --> " "$fn"
    mkdir -p "$DST/$bn"
    sudo cp "$bn/$fn" "$DST/$bn/"
    sudo cp "$bn/cmdline" "$DST/$bn/"
    sudo cp "$bn/comm" "$DST/$bn/"
  done
))

