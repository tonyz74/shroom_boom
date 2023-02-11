#!/bin/zsh

PROJECT=ShroomBoom

cargo build --target=x86_64-apple-darwin -Zbuild-std --release

rm -rf dist/$PROJECT.app
mkdir dist/$PROJECT.app
mkdir dist/$PROJECT.app/Contents
mkdir dist/$PROJECT.app/Contents/MacOS
mkdir dist/$PROJECT.app/Contents/Resources

cp -r assets dist/$PROJECT.app/Contents/MacOS
cp target/x86_64-apple-darwin/release/shroom_boom dist/$PROJECT.app/Contents/MacOS/ShroomBoom
cp dist/Info.plist dist/$PROJECT.app/Contents/Info.plist
cp assets/icon/Icon.icns dist/$PROJECT.app/Contents/Resources/Icon.icns