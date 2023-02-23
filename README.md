# log-scraper

A simple program that queries the logs from New Relic's GraphQL API.

Written in Rust. Deployable as a Docker container.

### Requirements

- Rust / Cargo
- Docker
- Redis
- Environment Variables
  - NRLS_ACCOUNT_ID: New Relic Account ID
  - NRLS_API_KEY: New Relic API Key
  - REDIS_URL: Redis URL with port
  - LS_SVC_PORT: (optional) App server port (defaults to 3333)

