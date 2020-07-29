#!/bin/bash

set -e

cd "$( dirname "${BASH_SOURCE[0]}" )"
cd ../client
yarn
yarn build

cd ../server
cargo build --release
cd ..

mkdir -p /etc/napoleon/
cp server/target/release/napoleon /etc/napoleon/bin
rm -r /etc/napoleon/www || {}
mv client/build /etc/napoleon/www

sudo service napoleon restart
