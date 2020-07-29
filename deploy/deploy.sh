#!/bin/bash

set -e

cd $(dirname ${BASH_SOURCE[0]})
cd ../client
yarn
yarn build

cd ../server
$HOME/.cargo/bin/cargo build --release
cd ..

cp server/target/release/ws_server /etc/napoleon/bin
rm -r /etc/napoleon/www || {}
mv client/build /etc/napoleon/www

sudo service napoleon restart
