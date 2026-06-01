#!/bin/bash

if [ ! -d /home/duperemove ]; then
  echo "This is your first time running duperemove. It will take a long time."
  sudo mkdir -p /home/duperemove
  sudo chown -R "$USER" /home/duperemove
fi

TARGET="$HOME"
if [ -n "$1" ]; then
  TARGET="$1"
fi
echo "Deduping $TARGET"

duperemove -rdh --skip-zeroes --hashfile=/home/duperemove/hashes "$TARGET"
