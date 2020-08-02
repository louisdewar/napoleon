#!/bin/bash

set -e

cd $(dirname ${BASH_SOURCE[0]})
cd ../client
yarn
PUBLIC_URL=$1 yarn build 

cd ../server
$HOME/.cargo/bin/cargo build --release
cd ..

rm /etc/napoleon/bin || echo "new bin"
cp server/target/release/ws_server /etc/napoleon/bin
rm -r /etc/napoleon/www || echo "www did not exist"
mv client/build /etc/napoleon/www

sudo service napoleon restart
