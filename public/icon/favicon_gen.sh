#!/bin/bash

tags=""
rm -f icon-*.png
for size in $(cat sizes.txt); do
    convert icon.png -resize ${size}x${size} icon-${size}.png
    tags="${tags}<link rel=\"icon\" type=\"image/png\" sizes=\"${size}x${size}\" href=\"/icon/icon-${size}.png\" />"
    tags="${tags}<link rel=\"apple-touch-icon\" sizes=\"${size}x${size}\" href=\"/icon/icon-${size}.png\" />"
done

echo $tags
