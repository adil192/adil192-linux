#!/bin/bash

FLUTTER_DIR="$HOME/Documents/Sources/flutter/"

echo "Installing Flutter's dependencies..."
if [ -n "$(which dnf)" ]; then
  sudo dnf install curl git unzip xz zip libGLU
elif [ -n "$(which apt)" ]; then
  sudo apt update
  sudo apt install curl git unzip xz-utils zip libglu1-mesa
else
  echo "Can't find apt or dnf. Please install dependencies manually."
fi
echo

echo "Downloading Flutter into $FLUTTER_DIR ..."
if [ -d $FLUTTER_DIR ]; then
  echo "Flutter already exists in $FLUTTER_DIR. Skipping download."
else
  git clone https://github.com/flutter/flutter.git $FLUTTER_DIR -b stable
fi
echo

if [ -n "$(which flutter)" ]; then
  echo "Flutter is already on path."
else
  echo "Adding Flutter to PATH..."
  echo "export PATH=\"\$PATH:${FLUTTER_DIR}bin\"" >> ~/.bashrc
  echo "export PATH=\"\$PATH:\$HOME/.pub-cache/bin\"" >> ~/.bashrc
  source ~/.bashrc
fi
echo

echo "Running flutter doctor..."
flutter doctor
echo
