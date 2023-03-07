# log-scraper

A simple service that queries the logs from a remote log aggregation service query endpoint (New Relic's GraphQL API).

Written in Rust. Deployable as a Docker container.

### Requirements

- Rust / Cargo
- Docker
- Redis


## Environment Variables

| Name                 | Default Value           | Description                                          |
|:---------------------|:------------------------|------------------------------------------------------|
| `LOG_DIRECTORY`      | `"./"`                  | The location of where logs are stored on the system. |
| `LOG_FILE_PREFIX`    | `"app"`                 | The filename prefix for saving log files.            |
| `LOG_FILE_EXTENSION` | `"log"`                 | The extension to use when saving log files.          |
| `LS_SVC_PORT`        | `"3333"`                | The port the service will be served at.              |
| `NRLS_ACCOUNT_ID`    | `""`                    | New Relic Account ID                                 |
| `NRLS_API_KEY`       | `""`                    | New Relic API Key                                    |
| `REDIS_URL`          | `"127.0.0.1:6379"`      | Redis URL with port                                  |
| `REDIS_KEY_NAME`     | `"last_seen_timestamp"` | The key name to store the last seen timestamp under. |


## Useful Dev Commands

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

