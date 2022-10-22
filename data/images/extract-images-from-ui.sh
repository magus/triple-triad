#!/bin/bash

# set -x

# generate with SaintCoinach
# see https://github.com/mattantonelli/xiv-data/blob/master/extract.bat
UI_ZIP="$HOME/Downloads/ui.zip"
IMAGE_PATH="$HOME/Downloads/ffxiv-images"

mkdir -p $IMAGE_PATH
mv $UI_ZIP $IMAGE_PATH
cd $IMAGE_PATH
unzip -oq ui.zip
find ./icon -type f -exec chmod 664 {} \;

open $IMAGE_PATH
