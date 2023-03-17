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

## Helm Chart

There is a helm chart for deploying the service to a Kubernetes environment. See the [`./helm`](./helm/) directory for more information.

Deploy to Kubernetes:

```bash
helm install log-scraper ./helm --namespace <my-namespace> \
  --set service.newRelicAccountId=$NRLS_ACCOUNT_ID \
  --set service.newRelicApiKey=$NRLS_API_KEY \
  --set service.logPrefix='my-awesome-app-logs'
```


## Configuration

There are environment variable and helm variable names for each configuration setting the `log-scraper` accepts. The table below shows the corresponding names for each setting and their default value for quick reference followed by the same list with their descriptions for futher details.

| Name                 | Helm Setting                 | Default Value           |
|:---------------------|:-----------------------------|:------------------------|
| `LOG_DIRECTORY`      | `service.logDirectory`       | `"./"`                  |
| `LOG_FILE_PREFIX`    | `service.logPrefix`          | `"app"`                 |
| `LOG_FILE_EXTENSION` | `service.logExtension`       | `"log"`                 |
| `LS_POLL_SCHEDULE`   | `service.pollSchedule`       | `"0 1/5 * * * *"`       |
| `LS_SVC_PORT`        | `service.port`               | `"3333"`                |
| `NRLS_ACCOUNT_ID`    | `service.newRelicAccountId`  | `""`                    |
| `NRLS_API_KEY`       | `service.newRelicApiKey`     | `""`                    |
| `REDIS_URL`          | `service.redisURL`           | `"127.0.0.1:6379"`      |
| `REDIS_KEY_NAME`     | `service.redisKeyName`       | `"last_seen_timestamp"` |


## Config Details


**LOG_DIRECTORY** (`service.logDirectory`)

The location of where logs are stored on the system. Note this defaults to a volume path when deploying with Helm (mounted at `/usr/src/app/logs`).

**LOG_FILE_PREFIX** (`service.logPrefix`)

The filename prefix for saving log files. The resulting saved files wll have the pattern similar to the following:

```bash
{logPrefix}_{date}.{logExtension}
```

**LOG_FILE_EXTENSION** (`service.logExtension`)

The extension to use when saving log files. Does not include the "dot".

**LS_POLL_SCHEDULE** (`service.pollSchedule`)

The cron schedule to set for polling the remote server to search for new logs.

**LS_SVC_PORT** (`service.port`)

The port the service will be served at.

**NRLS_ACCOUNT_ID** (`service.newRelicAccountId`)

The New Relic Account ID to authenticate as. This is passed in the query that is sent to their API.

**NRLS_API_KEY** (`service.newRelicApiKey`)

This is an API Key from New Relic that works with their NRQL GraphQL API.

**REDIS_URL** (`service.redisURL`)

Redis URL with port.

**REDIS_KEY_NAME** (`service.redisKeyName`)

The key name to store the timestamp of the last seen log entry fetched from the remote server.


---

## Development

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

