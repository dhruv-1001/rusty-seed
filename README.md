# Rusty-Seed

## About 
This project aims to enable peer to peer file transfer somewhat like torrent, and therefore "seed" word in the project name (and ofc Rusty as it's in Rust). It is not compatable with torrent, as it follows custom protocol.

## Features
- [ ] Network
  - [x] Works for devices on LAN 
  - [ ] NAT traversal to connect to peers on differet network
- [ ] Multi-threaded
  - [ ] Client Side
    - [ ] Start listening on port for local CLI requests 
    - [ ] Open database to check files to be downloaded and their download status
    - [ ] Connects to the peers to download files that are to be downloaded, and also those that are in progress
  - [ ] Server Side
    - [x] On start validates already added seeds (checks if there are any changes in the file)
    - [x] Start listening on port for local Client & CLI requests
    - [ ] Start listening on port for requests from the Internet
- [ ] CLI commands
  - [ ] Client Side (TODO: Describe in detail)
    - [ ] `download --link="file link"` downloads files from provided link 
  - [ ] Server Side 
    - [x] `list-seeds` lists all seeding paths
    - [x] `add-seed --path=path/to/file` add file to seeding list
    - [ ] `remove-seed --path=path/to/file` remove file from seeding list

### Setup Requirements
[Rust](https://www.rust-lang.org/tools/install). Install rust using
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Clone the project build the project
```
git clone https://github.com/dhruv-1001/rusty-seed.git
cd rusty-seed
caro build 
```

- [ ] TODO: add bash script to build and run the project