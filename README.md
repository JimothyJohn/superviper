# superviper

An experiment in deploying Rust on a RISC-V MCU. If you can do it here you can do it anywhere!

## Goal

Extract information from the environment as efficiently as possible.

### Hardware

- [XIAO ESP32C3](https://wiki.seeedstudio.com/XIAO_ESP32C3_Getting_Started/)

### Notes

- My microcontroller was showing up on /dev/ttyACM0 so I had to update the Dockerfile [ENV variables](.devcontainer/Dockerfile#L7) and [dialout permissions](.devcontainer/Dockerfile#L24) as well as the [devcontainer's runArgs]((.devcontainer/devcontainer.json#L46))

### Resources

1. [The Rust Programming Language](https://doc.rust-lang.org/book/)
    - [Bookmark](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
2. [The Embedded Rust Book](https://docs.rust-embedded.org/book/index.html)
    - [Bookmark](https://docs.rust-embedded.org/book/start/qemu.html)
3. [The Rust on ESP Book](https://docs.esp-rs.org/book/)
4. [Embedded Rust (no_std) on Espressif](https://docs.esp-rs.org/no_std-training/)
    - [Bookmark](https://docs.esp-rs.org/no_std-training/03_6_http_client.html)

### TODO

- [x] Create an async GET HTTP request -> [Code](https://github.com/esp-rs/esp-hal/blob/main/examples/src/bin/wifi_embassy_dhcp.rs)
- [ ] Create an async POST HTTP request
- [ ] Create a SECURE POST HTTP request using TLS. [Reference](https://github.com/drogue-iot/embedded-tls)
- [ ] Utilize flash/NVS encryption. [Reference](https://espressif.github.io/esp32-c3-book-en/chapter_13/13.3/13.3.7.html)
