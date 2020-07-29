#!/bin/bash

set -e

cd $(dirname ${BASH_SOURCE[0]})

cp napoleon.service /etc/systemd/system/
systemctl daemon-reload
systemctl enable napoleon
