#!/bin/bash
set -e

if [ ! -d /home/duperemove ]; then
  echo "This is your first time running duperemove. It will take a long time."
  sudo mkdir -p /home/duperemove
  sudo chown -R "$USER" /home/duperemove
  sleep 1
fi

# Hash the contents of $1. Does not dedupe them.
crawl() {
  TARGET="${*:$#}" # the last argument
  if [ -d "$TARGET" ] || [ -e "$TARGET" ]; then
    echo "Crawling $TARGET..."
    time chrt -i 0 duperemove -h --hashfile=/home/duperemove/hashes "$@" >/dev/null
    sleep 1 # ensure db is free
  else
    echo "Warning: $TARGET does not exist."
  fi
}
# Dedupes files based on the hashfile
dedupe() {
  echo "Deduping hashed files..."
  # It needs a file argument so just use ~/.bashrc since it probably exists.
  time chrt -i 0 duperemove -h --hashfile=/home/duperemove/hashes -d ~/.bashrc
}

if [ -n "$1" ]; then
  crawl "$1"
else
  # Crawl specific directories instead of the whole $HOME.
  # Some directories are not good for deduping, e.g.:
  #  - ~/.cache changes frequently so results don't last
  #  - Waydroid files are not accessible and spam permission warnings
  crawl ~ # (non-recursive)
  crawl -r ~/.android
  crawl -r ~/Android
  crawl -r ~/Applications
  crawl -r ~/.cargo
  crawl -r ~/.config
  crawl -r ~/.dart-tool
  crawl -r ~/Desktop
  crawl -r ~/Documents
  crawl -r ~/fvm
  crawl -r ~/Games
  crawl -r ~/.gradle
  crawl -r --exclude=~/.local/share/waydroid ~/.local
  crawl -r ~/.mozilla
  crawl -r ~/.npm
  crawl -r ~/.nvm
  crawl -r ~/Pictures
  crawl -r ~/Projects
  crawl -r ~/.pub-cache
  crawl -r ~/Public
  crawl -r ~/.rustup
  crawl -r ~/.steam
  crawl -r ~/.thunderbird
  crawl -r ~/thunderbird
  crawl -r ~/.var
fi
dedupe
