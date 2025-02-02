# superviper

An experiment in deploying Rust on a RISC-V ESP32. If you can do it here you can do it anywhere!

## Goals

Extract information from the environment as efficiently as possible.

### Hardware

- [XIAO ESP32C3](https://wiki.seeedstudio.com/XIAO_ESP32C3_Getting_Started/)

### Notes

- My microcontroller was showing up on /dev/ttyACM0 so I had to update the Dockerfile [ENV variables](.devcontainer/Dockerfile#L7) and [dialout permissions](.devcontainer/Dockerfile#L24) as well as the [devcontainer's runArgs]((.devcontainer/devcontainer.json#L46))

### Resources

1. [The Rust Programming Language](https://doc.rust-lang.org/book/)
    - [Bookmark](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
2. [The Embedded Rust Book](https://docs.rust-embedded.org/book/index.html)
    - [Bookmark](https://docs.rust-embedded.org/book/start/qemu.html)
3. [The Rust on ESP Book](https://docs.esp-rs.org/book/)
    - Completed
4. [Embedded Rust (no_std) on Espressif](https://docs.esp-rs.org/no_std-training/)
    - [Bookmark](https://docs.esp-rs.org/no_std-training/03_6_http_client.html)
    - [no_std Training](https://github.com/esp-rs/no_std-training)

### TODO

- [ ] Create an GET HTTP request
    - [Example](https://github.com/esp-rs/no_std-training/tree/main/intro/http-client)
- [ ] Create an POST HTTP request
- [ ] Identify a security/encryption method 
