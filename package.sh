#!/usr/bin/env bash

mkdir -p out
rm out/*

cp -p target/arm-unknown-linux-musleabihf/$1/examples/{image,fill,clear,flashes,dot} out/

armv7l-unknown-linux-gnueabihf-strip out/*

cd out
tar -czv * -f ../$1$2.tar.gz
