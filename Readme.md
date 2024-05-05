<div align="center">
<img src="assets/512x512.png" alt="logo" width="150" style="border-radius: 25px"/>

# Sanchaar - A Offline REST API client

Sanchaar is a offline REST API client built using Iced in Rust. It is a simple tool to test REST APIs without the need of internet connection. It supports GET, POST, PUT, DELETE requests with path, query and header parameters.

</div>

## Screenshot

![Screenshot](./screenshots/app.png)

## Features

- Send GET, POST, PUT, DELETE requests
- Path, Query, Header params
- Multiple requests in tabs
- Save/Load requests from collections
- Create/Open collection from local disk
- Environment variables

## Roadmap

- [x] Path param support
- [x] Query param support
- [x] Header param support
- [x] Body support
  - [x] JSON
  - [x] Form
  - [x] XML
  - [x] Multipart
- [x] Request cancellation
- [ ] Authentication
  - [x] Basic
  - [x] Bearer
  - [ ] OAuth
  - [ ] Digest Auth
- [x] Tab view for multiple requests
- [x] File persistence
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
  - [x] Remove
  - [x] Rename collection/folder
  - [ ] Export/Import
  - [ ] Settings
    - [ ] Update default env
    - [ ] Change template syntax
    - [ ] Collection headers
    - [ ] Collection auth
    - [ ] Request preset
- [ ] Environments
  - [x] Add/Remove/Update
  - [x] Choose environment
  - [x] Auto Save/Load current state
  - [ ] Secure environment variables
  - [ ] Variables from .env file
- [ ] Request Tests
- [ ] History
  - [ ] List
  - [ ] Clear
  - [ ] Open from history
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
  - [x] Theme
  - [ ] Proxy
  - [ ] SSL
  - [ ] Timeout
- [ ] Cookies
  - [ ] List
  - [ ] Edit/Add/Remove
- [ ] Code export
