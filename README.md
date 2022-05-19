# azure-iot-sdk-sys

This repository provides low level rust bindings for the [azure-iot-sdk-c](https://github.com/Azure/azure-iot-sdk-c). `azure-iot-sdk-sys` serves as basic sys crate for [azure-iot_sdk](https://github.com/ICS-DeviceManagement/azure-iot-sdk) and [iot-client-template](https://github.com/ICS-DeviceManagement/iot-client-template-rs).

# Build

In order to build `azure-iot-sdk-sys` the following library dependencies must be provided via environment variable:
- `LIB_PATH_AZURESDK`: path to azure-iot-sdk-c libraries
- `LIB_PATH_UUID`: path to libuuid libraries
- `LIB_PATH_OPENSSL`: path to openssl libraries
- `LIB_PATH_CURL`: path to libcurl libraries

There are absolute paths expected that might include wildcard semantics. It is expected that the path points to a directory with a "lib" and "include" subfolder.

## Provide your own libraries

You're free to build your own versions of libraries for the target platform of your choice or use the ones provided by your operating system.

## Use prebuild libraries for x86_64

For your convenience we provide for x86_64 a bundle of libraries, created on Ubuntu 20.04lts, as part of our github release. Please find the library archive [here](https://github.com/ICS-DeviceManagement/azure-iot-sdk-sys/releases/latest) 

## Enable iot edge support

In order to build `azure-iot-sdk-sys` with iot edge module API's enabled you have to set `edge_modules` cargo feature. This builds bindings with `"-DUSE_EDGE_MODULES"` option.<br>
**Note: `azure-iot-sdk-c` also must have been built with `"-DUSE_EDGE_MODULES"` option!**

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

Copyright (c) 2021 conplement AG
