#!/bin/bash

find ~/Documents/{GitHub,Sources} -name .flatpak-builder -exec rm -rf {} \; -print
find ~/Documents/{GitHub,Sources} -name builddir -exec rm -rf {} \; -print
find ~/Documents/{GitHub,Sources} -name build -exec rm -rf {} \; -print
find ~/Documents/{GitHub,Sources} -name dist -exec rm -rf {} \; -print
find ~/Documents/{GitHub,Sources} -name node_modules -exec rm -rf {} \; -print
find ~/Documents/{GitHub,Sources} -name repo -exec rm -rf {} \; -print
find ~/Documents/{GitHub,Sources} -name target -exec rm -rf {} \; -print
