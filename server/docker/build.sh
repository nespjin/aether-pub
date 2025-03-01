#!/bin/bash
CURRENT_PATH=$(dirname $(readlink -f "$0"))
cd $CURRENT_PATH

docker-compose up --build