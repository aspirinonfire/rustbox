# rustbox
Miscellaneous Rustacean shenanigans
[![Rust](https://github.com/aspirinonfire/rustbox/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/aspirinonfire/rustbox/actions/workflows/rust.yml)

## api_poc
Game Api proof-of-concept using [Actix Web](https://actix.rs/docs/getting-started/). The direction of the development is a replacement for a [License Plate Game Backend](https://github.com/aspirinonfire/thegame/tree/master/backend). Rust version will use MongoDB (hosted by Azure Cosmos) instead of SQL.

#### Docker
```bash
cd api_poc
docker build . --tag gameapi/rust:latest
docker run --publish 8000:8000 --env-file .env --tty gameapi/rust:latest
```

#### POC Topics
- [x] Logging
- [ ] Error Handling (4xx vs 5xx)
- [x] Configuration
- [x] JSON (de-)serialization
- [ ] Testing
  - [x] unit
  - [ ] mocking
  - [ ] testcontainers
- [x] Project/folder structure
- [ ] Mongo Client
- [x] Docker
- [x] Auth middleware
- [ ] Bearer Auth (JWT)