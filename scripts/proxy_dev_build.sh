#!/bin/bash

THIS_DIR=$(dirname "$0")

# Function to trap SIGINT and kill background processes
trap 'kill $(jobs -p)' SIGINT


# run server in the background
cargo run > /dev/null &


# run the web ui in the foreground so we can kill everything at once
cd $THIS_DIR/../web && npm run proxy-dev

