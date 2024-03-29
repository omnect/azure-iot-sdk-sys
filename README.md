# azure-iot-sdk-sys
Product page: https://www.omnect.io/home

This repository provides low level rust bindings for the [azure-iot-sdk-c](https://github.com/Azure/azure-iot-sdk-c).<br>
 `azure-iot-sdk-sys` serves as basic sys crate for [azure-iot_sdk](https://github.com/omnect/azure-iot-sdk) and [iot-client-template](https://github.com/omnect/iot-client-template-rs) crates.

# Build

## Optionally enable iot edge support

In order to build `azure-iot-sdk-sys` with iot edge module API's enabled you have to set `edge_modules` cargo feature:
```
cargo build --features edge_modules
```

## Install dependencies

In order to build `azure-iot-sdk-sys` dependencies to the following library must be installed:
- `azure-iot-sdk-c` for iot module development
- `azure-iotedge-sdk-c` for iotedge module development

We provide debian packages for the following target architectures:
- `amd64`
- `arm64`
- `arm32v7`

Available debian packages can be listed as a xml document via this [link](https://omnectassetst.blob.core.windows.net/azure-iot-sdk-dev-packages?restype=container&comp=list). Choose the one appropriate for your usecase and platform and download e.g. via:
```
wget https://omnectassetst.blob.core.windows.net/azure-iot-sdk-dev-packages/azure-iot-sdk-dev-0.1.0-amd64.deb
```

**Note:** The libraries depend on further development libraries that must be installed for the chosen target architecture:
- `libcurl4-openssl-dev`
- `libcurl4`
- `libssl-dev`
- `libuuid1`
- `uuid-dev`

# License

Licensed under either of
* Apache License, Version 2.0, (./LICENSE-APACHE or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license (./LICENSE-MIT or <http://opensource.org/licenses/MIT>)
at your option.

# Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.

---

copyright (c) 2021 conplement AG<br>
Content published under the Apache License Version 2.0 or MIT license, are marked as such. They may be used in accordance with the stated license conditions.
