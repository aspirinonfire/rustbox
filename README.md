# rustbox
Miscellaneous Rustacean shenanigans
[![Rust](https://github.com/aspirinonfire/rustbox/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/aspirinonfire/rustbox/actions/workflows/rust.yml)

## api_poc
Game Api proof-of-concept using [Actix Web](https://actix.rs/docs/getting-started/). The direction of the development is a replacement for a [License Plate Game Backend](https://github.com/aspirinonfire/thegame/tree/master/backend). Rust version will use MongoDB (hosted by Azure Cosmos) instead of SQL.

#### Docker
```bash
# Start mongo instance for development
docker run --publish 27017:27017 --env-file .env --detach mongo:7.0

# build and run rust app from docker
cd api_poc
# populate .env file. see .env.sample
docker-compose build
docker-compose up
```

#### POC Topics
- [x] Logging
- [ ] Error Handling (4xx vs 5xx)
- [x] Configuration
- [x] JSON (de-)serialization
- [x] Unit Testing
- [x] Integration testing with testcontainers
- [x] Project/folder structure
- [x] Mongo Client
- [x] Docker
- [x] Auth middleware
- [x] Bearer Auth (JWT)