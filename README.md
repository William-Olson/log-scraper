# log-scraper

[![CircleCI](https://dl.circleci.com/status-badge/img/gh/William-Olson/log-scraper/tree/master.svg?style=svg)](https://dl.circleci.com/status-badge/redirect/gh/William-Olson/log-scraper/tree/master)
[![Docker Image Version (tag latest semver)](https://img.shields.io/docker/v/willko/log-scraper/latest?color=blue&logo=docker)](https://hub.docker.com/r/willko/log-scraper/tags)

A simple service that queries the logs from a remote log aggregation service query endpoint (currently just New Relic's GraphQL API).

Written in Rust. Deployable as a Docker container.

## Running via Docker

You can run the log-scraper with Docker and start up a redis container to use for caching.

```bash
# optional: start up redis
docker run --name some-redis -p 6379:6379 -d redis

# run the log-scraper service
docker run -i \
  -e NRLS_ACCOUNT_ID='1234567' \
  -e NRLS_API_KEY='<my-new-relic-api-key>' \
  -e REDIS_URL='redis:6379' \
  -e LOG_DIRECTORY='/usr/src/app/logs' \
  -e LS_SVC_PORT=7777 \
  -p 8080:7777 \
  --link some-redis:redis \
  willko/log-scraper:0.3.0-with-docs
```

Note: provide `-d` instead of `-i` for detached (background) mode instead of interactive mode.

Test that the service is up:

```bash
curl -i localhost:8080

# HTTP/1.1 200 OK
# content-length: 92
# content-type: application/json
# date: Sun, 12 Mar 2023 18:01:36 GMT
# 
# {"ok":true,"message":"Healthy and kicking! Docs: /docs/log_scraper/api/logs_api/index.html"}
```


## Environment Variables

Below is a full list of environment variables you can provide to the log-scraper service.

| Name                 | Default Value           | Description                                          |
|:---------------------|:------------------------|------------------------------------------------------|
| `LOG_DIRECTORY`      | `"./"`                  | The location of where logs are stored on the system. |
| `LOG_FILE_PREFIX`    | `"app"`                 | The filename prefix for saving log files.            |
| `LOG_FILE_EXTENSION` | `"log"`                 | The extension to use when saving log files.          |
| `LS_POLL_SCHEDULE`   | `"0 1/5 * * * *"`       | The schedule to poll the remote server for new logs. |
| `LS_SVC_PORT`        | `"3333"`                | The port the service will be served at.              |
| `NRLS_ACCOUNT_ID`    | `""`                    | New Relic Account ID                                 |
| `NRLS_API_KEY`       | `""`                    | New Relic API Key                                    |
| `REDIS_URL`          | `"127.0.0.1:6379"`      | Redis URL with port                                  |
| `REDIS_KEY_NAME`     | `"last_seen_timestamp"` | The key name to store the last seen timestamp under. |


## Building locally

Building and running the service locally is also quite simple and straightforward.

### Dev Requirements

- [Rust / Cargo](https://www.rust-lang.org/tools/install)
- [Docker](https://docs.docker.com/get-docker/)


#### Useful Dev Commands

The following commands should get you up and running. There scripts provided as well to help build and run via Docker locally.

```
# Local Build
cargo build

# Local Run
cargo run

# Generate Docs
cargo doc --document-private-items --open

# Docker Build
./scripts/docker_build.sh

# Docker Run
./scripts/docker_dev_run.sh

# Docker Cleanup
./scripts/docker_cleanup.sh
```

