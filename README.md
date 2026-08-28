# LTC2686 Rust driver

This is a pure Rust implementation of a driver for the
AnalogDevices [LTC2686] DAC.
The driver is designed to use the SPI interface and is built on top of
[embedded-hal](https://docs.rs/embedded-hal/latest/embedded_hal/) traits
and making use of the excellent
[device_driver](https://docs.rs/device-driver/latest/device_driver/)
toolkit.

**This driver is not yet complete and is currently under development.**
Thus, no package has as of yet been published to [crates.io]

## License

All source code in this repository is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

[crates.io]: https://crates.io
[LTC2686]: https://www.analog.com/en/products/ltc2686.html
