# rustbox
Miscellaneous Rustacean shenanigans

## api_poc
Game Api proof-of-concept using [Actix Web](https://actix.rs/docs/getting-started/). The direction of the development is a replacement for a [License Plate Game Backend](https://github.com/aspirinonfire/thegame/tree/master/backend). Rust version will use MongoDB (hosted by Azure Cosmos) instead of SQL.

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
- [ ] Docker
- [x] Auth middleware
- [ ] Bearer Auth (JWT)