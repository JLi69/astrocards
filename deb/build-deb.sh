#!/bin/sh

# build lastest release of astrocards
cargo build --release

NAME=astrocards

# Clean up the directory
mv $NAME/DEBIAN .
rm -rf $NAME
mkdir $NAME
mv DEBIAN/ $NAME

# make directories
mkdir $NAME/usr/bin/ -p
mkdir $NAME/usr/share/games/$NAME/ -p
mkdir $NAME/usr/share/applications/ -p
mkdir $NAME/usr/share/icons/hicolor/96x96/apps/ -p

# copy over executable
cp ../target/release/$NAME $NAME/usr/bin/

# copy over assets/
cp ../assets/ $NAME/usr/share/games/$NAME/ -r
# copy over sets/
cp ../sets/ $NAME/usr/share/games/$NAME/ -r
# copy over cfg.impfile
cp ../cfg.impfile $NAME/usr/share/games/$NAME/
# copy over .desktop file
cp astrocards.desktop $NAME/usr/share/applications/
# copy over icon
cp ../assets/icon.png $NAME/usr/share/icons/hicolor/96x96/apps/astrocards.png

# build the debian package
dpkg-deb --build $NAME
