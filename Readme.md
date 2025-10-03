<div align="center">
<img src="assets/512x512.png" alt="logo" width="150" style="border-radius: 25px"/>

[![Discord](https://img.shields.io/discord/1261282563138392117?color=5865F2&label=Discord&logo=discord&logoColor=white)](https://discord.gg/FSK25BXgdt)

</div>

# Sanchaar - A Offline REST API client

Sanchaar is a offline REST API client built using Iced in Rust. It is a simple tool to test REST APIs without the need of internet connection. It supports GET, POST, PUT, DELETE requests with path, query and header parameters.

## Screenshot

![Screenshot](./screenshots/app.png)

## Installation

### Pre-built Binaries

Download the latest release for your platform from the [Releases page](https://github.com/nrjais/sanchaar/releases).

#### macOS Installation

**Important:** Since the app is not notarized by Apple, you'll need to remove the quarantine attribute after downloading:

1. Download the `.dmg` or `.tar.gz` file
2. If using DMG: Open it and drag Sanchaar to Applications
3. Remove the quarantine attribute:
   ```bash
   xattr -cr /Applications/Sanchaar.app
   ```

   Or if installed to a different location:
   ```bash
   xattr -cr /path/to/Sanchaar.app
   ```

4. Now you can open Sanchaar normally

**Alternative method:** Right-click the app → "Open" → Click "Open" in the dialog (only needed first time)

#### Linux Installation

- **AppImage**: `chmod +x Sanchaar-*.AppImage && ./Sanchaar-*.AppImage`
- **Debian/Ubuntu**: `sudo dpkg -i Sanchaar-*.deb && sudo apt-get install -f`
- **Archive**: Extract and run the binary

#### Windows Installation

- **MSI Installer**: Double-click to install
- **ZIP Archive**: Extract and run `Sanchaar.exe`

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
  - [x] Text
  - [x] Raw File
  - [x] Multipart (Files not supported with GET method)
- [x] Request cancellation
- [ ] Authentication
  - [x] Basic
  - [x] Bearer
  - [ ] OAuth
  - [ ] OAuth2
  - [ ] AWS
  - [ ] Digest Auth
- [x] Tab view for multiple requests
- [x] File persistence
  - [x] HCL file format
  - [x] Save/Load
  - [x] Changed indicator
  - [x] File Rename
- [ ] Collections/Folder
  - [x] Tree view
  - [x] Create/Open
  - [ ] Auto Save
  - [ ] Refresh tree manually
  - [ ] Refresh tree automatically
  - [x] Remove
  - [x] Rename collection/folder
  - [ ] Export/Import
  - [ ] Settings
    - [x] Update default env
    - [x] Collection headers
    - [x] Collection Variables
    - [x] SSL verification
    - [ ] Collection auth
    - [ ] Request preset
    - [ ] Timeout
- [ ] Environments
  - [x] Add/Remove/Update
  - [x] Choose environment
  - [x] Auto Save/Load current state
  - [ ] Secure environment variables (keyring)
  - [x] Variables from .env file
  - [x] dotenv file var access in environment vars
- [ ] Assertions
  - [x] Status code
  - [x] Response time
  - [x] Response body
  - [x] Response headers
  - [ ] GUI editor/viewer
- [ ] Scripting
  - [ ] Pre request
  - [ ] Post request
- [ ] Settings
  - [x] Theme
  - [ ] Cookie store toggle
  - [ ] Proxy
  - [ ] About
- [ ] Cookies
  - [ ] List/Remove
  - [ ] Edit/Add ?
- [ ] History
  - [ ] List
  - [ ] Clear
  - [ ] Open from history
- [ ] Mock APIs
- [ ] CLI
  - [x] Run request by path
  - [x] Run assertion by path/folder
  - [x] Pretty print assertion results
  - [ ] Select environment by name
  - [x] Run tests by path
  - [ ] Run all collection tests
- [ ] Code export
- [ ] Body Viewer improvements
  - [x] Json path filter
  - [ ] XML path filter
  - [ ] Search in body
  - [x] Download body
- [ ] Body Editor
  - [x] Prettify body JSON
  - [ ] Search in body
- [x] Hotkeys
  - [x] Close tab (Cmd + W)
  - [x] Close all tabs (Cmd + Shift + W)
  - [x] Close other tabs (Cmd + Alt + W)
  - [x] New request (Cmd + T)
  - [x] Send request (Cmd + Enter)
  - [x] Save request (Cmd + S - In request view)
  - [x] Save collection (Cmd + S - In collection view)
  - [x] Save environment (Cmd + S - In environment view)
  - [x] App Setting (Cmd + ,)
  - [x] Collection Setting (Cmd + ; - In http tab)
- [ ] Other improvemetns
  - [ ] Error handling
  - [ ] Logging and tracing request
  - [ ] Reduce clones
  - [x] Variable highlighting
  - [ ] Migrate to KDL for files
