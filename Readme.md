# Sanchaar - A Offline REST API client

Sanchaar is a offline REST API client built using Iced in Rust. It is a simple tool to test REST APIs without the need of internet connection. It supports GET, POST, PUT, DELETE requests with path, query and header parameters.

## Screenshot

![Screenshot](./screenshots/app.png)

## Features

- Send GET, POST, PUT, DELETE requests
- Path, Query, Header params
- Multiple requests in tabs
- Save/Load requests from collections
- Create/Open collection from local disk

## Roadmap

- [x] Path param support
- [x] Query param support
- [x] Header param support
- [ ] Body support
  - [x] JSON
  - [x] Form
  - [x] XML
  - [ ] Multipart
- [x] Request cancellation
- [ ] Authentication
  - [ ] Basic
  - [ ] Bearer
  - [ ] OAuth
  - [ ] Digest Auth
- [x] Tab view for multiple requests
- [ ] File persistence
  - [x] TOML file format
  - [x] Save/Load
  - [x] Changed indicator
  - [x] File Rename
- [ ] Collections/Folder
  - [x] Tree view
  - [x] Create/Open
  - [x] Auto Save
  - [ ] Refresh tree manually
  - [ ] Refresh tree automatically
  - [ ] Remove
  - [ ] Rename/Update default env
  - [ ] Export/Import
- [ ] Environments
  - [x] Add/Remove/Update
  - [x] Choose environment
  - [x] Auto Save/Load current state
  - [ ] Secure environment variables
- [ ] Request Tests
- [ ] Code export
- [ ] History
  - [ ] Save/Load
  - [ ] Auto Save/Load
- [ ] Scripting
  - [ ] Pre request
  - [ ] Post request
- [ ] Mock APIs
- [ ] CLI
  - [ ] Run request by path
  - [ ] Select environment by name
  - [ ] Run tests by path
  - [ ] Run all collection tests
- [ ] Settings
  - [ ] Theme
  - [ ] Font
  - [ ] Proxy
  - [ ] SSL
  - [ ] Timeout
- [ ] Cookies
  - [ ] List
  - [ ] Edit/Add/Remove
