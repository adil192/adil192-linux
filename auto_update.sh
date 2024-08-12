#!/bin/bash

pushd "$(dirname "$0")"
git pull
dart pub get
dart run --no-interaction
popd
