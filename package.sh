#!/usr/bin/env bash

mkdir -p out
rm out/*

cp -p target/arm-unknown-linux-musleabihf/$1/examples/{image,fill,clear,flashes,dot} out/

cmd=armv7l-unknown-linux-gnueabihf-strip
command -v "$cmd" >/dev/null || cmd=arm-linux-gnueabihf-strip

"$cmd" out/*

cd out
tar -czv * -f ../$1$2.tar.gz
