# superviper

An experiment in deploying Rust on a RISC-V ESP32. If you can do it here you can do it anywhere!

### Hardware

- [XIAO ESP32C3](https://wiki.seeedstudio.com/XIAO_ESP32C3_Getting_Started/)

### Notes

- My microcontroller was showing up on /dev/ttyACM0 so I had to update the Dockerfile [ENV variables](.devcontainer/Dockerfile#L7) and [dialout permissions](.devcontainer/Dockerfile#L24) as well as the [devcontainer's runargs]((.devcontainer/devcontainer.json#L46))

### Resources

- [The Rust on ESP Book](https://docs.esp-rs.org/book/)

### TODO

- [ ] Create an HTTP request
