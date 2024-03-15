#!/bin/bash

THIS_DIR=$(dirname "$0")

# build the log-scraper-ui
docker build \
  -f $THIS_DIR/../web/Web.Dockerfile \
  -t willko/log-scraper-ui \
  $THIS_DIR/../web
