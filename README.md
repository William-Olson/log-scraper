# log-scraper

A simple program that queries the logs from New Relic's GraphQL API.

Written in rust. Deployable as a docker container.

### Requirements

- Rust / Cargo
- Docker
- Redis
- Environment Variables
  - NRLS_ACCOUNT_ID: New Relic Account ID
  - NRLS_API_KEY: New Relic API Key
  - REDIS_URL: Redis URL with port


