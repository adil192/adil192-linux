#!/bin/bash
#
# Sets the screen's resolution and other settings based on Sunshine's
# environment variables.
#
# This requires displayconfig-mutter:
#   sudo dnf copr enable eaglesemanation/displayconfig-mutter
#   sudo dnf install displayconfig-mutter
# See more instructions at https://github.com/eaglesemanation/displayconfig-mutter.

set -e

# A temporary file used to backup and then restore monitor settings.
# It is deleted after being restored to avoid using stale state in the future.
# When present, this SAVE_FILE is better than FALLBACK_SAVE_FILE since the
# latter may be outdated.
SAVE_FILE="$(dirname "$0")/.sunshine-res.backup.tmp"
# A fallback file used in "undo" if SAVE_FILE isn't present.
# It is only created once on first use and is not deleted after restore.
# This provides a fallback when a newer fresher SAVE_FILE is not available.
FALLBACK_SAVE_FILE="$(dirname "$0")/.sunshine-res.backup.fallback"

if [ "$1" == "do" ]; then
  # Make a backup to restore later in "undo"
  [ -f "$SAVE_FILE" ] || displayconfig-mutter save-file "$SAVE_FILE"
  [ -f "$FALLBACK_SAVE_FILE" ] || cp "$SAVE_FILE" "$FALLBACK_SAVE_FILE"

  # Disable second monitor so Steam doesn't use it
  displayconfig-mutter set \
    --connector DP-2 \
    --disable

  # Set resolution from sunshine on DP-1
  displayconfig-mutter set \
    --connector DP-1 \
    --resolution "${SUNSHINE_CLIENT_WIDTH:-1280}x${SUNSHINE_CLIENT_HEIGHT:-720}" \
    --refresh-rate "${SUNSHINE_CLIENT_FPS:-30}" \
    --vrr false \
    --hdr "${SUNSHINE_CLIENT_HDR:-false}"
elif [ "$1" == "undo" ]; then
  # Restore backed up monitor configuration
  if [ -f "$SAVE_FILE" ]; then
    displayconfig-mutter load-file "$SAVE_FILE"
    rm "$SAVE_FILE"
  else
    displayconfig-mutter load-file "$FALLBACK_SAVE_FILE"
  fi
else
  echo "Unknown option '$1'. Must be do or undo."
  exit 1
fi
