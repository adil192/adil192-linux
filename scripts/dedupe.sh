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

# Some volatile/inaccessible dirs are excluded to save time
time duperemove \
  -rdh --io-threads=16 --cpu-threads=8 \
  --exclude="$HOME/.cache/" \
  --hashfile=/home/duperemove/hashes \
  "$TARGET"
