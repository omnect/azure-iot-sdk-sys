# azure-iot-sdk-sys

This repository provides a low level rust interface wrapping a prebuild [azure-iot-sdk-c](https://github.com/Azure/azure-iot-sdk-c).

## Usage

To successfully build the low level rust wrapper, the system environment variables
LIB_PATH_AZURESDK, LIB_PATH_UUID, LIB_PATH_OPENSSL, LIB_PATH_CURL
must be present.

example for LIB_PATH_AZURESDK:

```sh
export LIB_PATH_AZURESDK=/build/.conan/data/azure-iot-sdk-c/*/_/_/package/*
```

You can use an absolute path as well as including wildcard semantics.
It is expected that the path points to the main directory of the respective c library.
The built-in libraries must be in the "lib" subfolder.
The header files must be in the "include" subfolder.

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
