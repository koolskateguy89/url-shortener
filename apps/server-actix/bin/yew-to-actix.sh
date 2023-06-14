#!/bin/sh

echo "Copying Yew files to Actix server..."
rm -rf ./static/yew
mkdir -p ./static/yew
cp -r ../../apps/web-yew/dist/* ./static/yew
echo "Done."
