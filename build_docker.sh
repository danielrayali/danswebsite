#!/usr/bin/bash

pushd webapp
cargo build --release
popd

sudo docker build --no-cache -t danswebengine .
sudo docker save -o danswebengine.tar danswebengine
sudo chown dali.dali ./danswebengine.tar
gzip --force ./danswebengine.tar
