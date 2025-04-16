#!/bin/sh

DIR=astrocards-linux/

# rebuild release
cargo build --release
# creates a linux release of astrocards (package is in .tar.gz format)
rm -rf release/
mkdir release/$DIR -p
# Copy the binary over
cp target/release/astrocards release/$DIR
# Copy `cfg.impfile`
cp cfg.impfile release/$DIR
# Copy assets/
cp -r assets/ release/$DIR
# Copy sets/
cp -r sets/ release/$DIR
cd release/ && tar -czf astrocards-linux.tar.gz $DIR
