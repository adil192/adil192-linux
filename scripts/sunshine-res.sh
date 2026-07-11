#!/bin/bash
#
# Sets the screen's resolution and other settings based on Sunshine's
# environment variables.
#
# This requires displayconfig-mutter:
#   sudo dnf copr enable eaglesemanation/displayconfig-mutter
#   sudo dnf install displayconfig-mutter
# See more instructions at https://github.com/eaglesemanation/displayconfig-mutter.
#
# Update: This now requires my branch for file save/load functionality.
# Build and install from https://github.com/adil192/displayconfig-mutter/tree/savefile-and-normalize-position

set -e

SAVE_FILE="$(dirname $0)/.sunshine-res.backup"

if [ "$1" == "do" ]; then
  # Make a backup to restore later in "undo"
  [ -f "$SAVE_FILE" ] || displayconfig-mutter save-file "$SAVE_FILE"

  # Disable second monitor so Steam doesn't use it
  displayconfig-mutter set \
    --connector DP-2 \
    --disable

  # Set resolution from sunshine on DP-1
  displayconfig-mutter set \
    --connector DP-1 \
    --resolution ${SUNSHINE_CLIENT_WIDTH:-1280}x${SUNSHINE_CLIENT_HEIGHT:-720} \
    --refresh-rate ${SUNSHINE_CLIENT_FPS:-30} \
    --vrr false \
    --hdr ${SUNSHINE_CLIENT_HDR:-false}
elif [ "$1" == "undo" ]; then
  # Restore backed up monitor configuration
  displayconfig-mutter load-file "$SAVE_FILE"
  rm "$SAVE_FILE"
else
  echo "Unknown option '$1'."
  exit 1
fi
