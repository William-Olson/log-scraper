#!/bin/bash

HAS_REDIS_UP=`docker ps | grep some-redis`
if [ -n "$HAS_REDIS_UP" ]; then
  echo 'using already running redis container';
else 
  # start a redis container
  echo 'starting redis (dev) container...';
  docker run --name some-redis --rm -p 6379:6379 -d redis
fi


# link the redis container when running the log-scraper
REDIS_URL='redis:6379' docker run -i \
  -e NRLS_ACCOUNT_ID \
  -e NRLS_API_KEY \
  -e REDIS_URL \
  --link some-redis:redis \
  willko/log-scraper:latest

# # clean up: stop and remove the redis container
# docker kill some-redis
