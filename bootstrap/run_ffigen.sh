#!/bin/bash
#
# This script is run automatically in ./lib/install.dart.
# You don't need to run it manually.

# Do nothing if ffigen has already been run.
if [ -f ".dart_tool/native_assets.yaml" ]; then
  exit 0
fi

if [ -z "$CPATH" ]; then
  # Fixes this error:
  #     fatal error: 'stdarg.h' file not found [Lexical or Preprocessor Issue]
  export CPATH=$(clang -print-resource-dir)/include
fi

dart run tool/ffigen.dart -v
