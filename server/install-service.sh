#!/bin/bash
CURRENT_PATH=$(dirname $(readlink -f "$0"))
cd $CURRENT_PATH

set -e

sudo cp ./aether-pub-server.service /etc/systemd/system/