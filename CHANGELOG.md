# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.5.5] Q4 2022
- added `azure_c_shared_utility/shared_util_options.h` to wrapper

## [0.5.4] Q3 2022
- added `iothub_client_options.h` to wrapper

## [0.5.3] Q3 2022
- updated dependency to bindgen to fix audit warning RUSTSEC-2021-0139 in projects using this crate

## [0.5.2] Q3 2022
- added iothub_client_version.h

## [0.5.1] Q3 2022
- fixed native library search path handling
- renamed library path environment variables

## [0.5.0] Q3 2022
 - bring back additional dependency definition via environment variables
 - environment variables, if completely defined, are preferred over pkg_config
 - the following paths must be set:
   - LIB_PATH_AZURESDK
   - LIB_PATH_UUID
   - LIB_PATH_OPENSSL
   - LIB_PATH_CURL

## [0.4.0] Q2 2022
 - refactored provisioning of azure-iot-sdk dependencies:
   - remove env var based approach to provide paths to pre-built libs and headers
   - instead depend on debian packages for each target architectures
   - use pkg-config in build.rs to get and ensure package dependencies

## [0.3.0] Q2 2022
 - added iot edge support

## [0.2.3] Q1 2022
 - removed conanfile.txt and instead provide pre-built-libs in release
 - improve documentation for building the crate

## [0.2.2] Q1 2022
 - added conanfile.txt for building depending azure-iot-sdk-c libs

## [0.2.1] Q1 2022
 - added iothub_device_client_ll.h for device twin access

## [0.2.0] Q1 2022
 - removed identity service
 - renamed package and repo to azure-iot-sdk-sys
 - added useful panic to build.rs

## [0.1.1] Q4 2021
 - fix compiling for 32bit systems

## [0.1.0] Q4 2021
 - initial version
