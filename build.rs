extern crate bindgen;
extern crate pkg_config;

use std::env;
use std::path::{PathBuf, Path};
use cmake;

fn main() {


    // check for dependencies
    pkg_config::probe_library("uuid").unwrap();
    pkg_config::probe_library("openssl").unwrap();
    pkg_config::probe_library("libcurl").unwrap();

    // Builds the azure iot sdk, installing it
    // into $OUT_DIR
    // use cmake::Config;

    // #[cfg(feature = "build_sdk")]
    // let dst = Config::new("azure/azure-iot-sdk-c")
    //                  .define("use_edge_modules", "OFF")
    //                  .define("hsm_type_sastoken", "OFF")
    //                  .define("hsm_type_x509", "OFF")
    //                  .define("skip_samples", "ON")
    //                  .define("use_prov_client", "OFF")
    //                  .define("use_tpm_simulator", "OFF")
    //                  .build();

    // println!("Libraries dst: {}", dst.display());


    // let mut src3: String = "-L".to_owned();
    // let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // src3.push_str(&out_dir.display().to_string());
    // src3.push_str("/lib");

    // let str3: &str = &src3;

    // let rst = Config::new("azure/iot-hub-device-update/src/utils/eis_utils")
    //                 .cflag(str3)
    //                 .build();

    //check environment variables, could be used to use path from there?
	for (name, _value) in env::vars() {
		if name.starts_with("XDG") {
			println!("env found: {}", name);
		}
	}


    let path_lib_azuresdk = Path::new("//home/joerg/projects/GitHub/simulator/build/.conan/data/azure-iot-sdk-c/LTS_07_2021_Ref01/_/_/package/3bf7811c9395d29095bf663023235996901b6af2");
    let path_lib_eisutils = Path::new("/home/joerg/projects/GitHub/simulator/build/.conan/data/libeis_utils/0.7.0/_/_/package/c0fdc5d58cc58ac5226b60cdbbb338a4bdcefac5");
    println!("{}", path_lib_azuresdk.display());
    println!("{}", path_lib_eisutils.display());

    // Tell cargo to tell rustc the search path of the libraries
    //println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-search=native={}/lib", path_lib_azuresdk.display());
    println!("cargo:rustc-link-search=native={}/lib", path_lib_eisutils.display());

    // Tell cargo to tell rustc to link the azure iot-sdk and eis_utils libraries.
    println!("cargo:rustc-link-lib=iothub_client_mqtt_transport");
    println!("cargo:rustc-link-lib=iothub_client");
    println!("cargo:rustc-link-lib=parson");
    println!("cargo:rustc-link-lib=umqtt");
    println!("cargo:rustc-link-lib=uhttp");
    println!("cargo:rustc-link-lib=aziotsharedutil");
    println!("cargo:rustc-link-lib=curl");
    println!("cargo:rustc-link-lib=ssl");
    println!("cargo:rustc-link-lib=crypto");
    println!("cargo:rustc-link-lib=uuid");
    println!("cargo:rustc-link-lib=eis_utils");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // additional clang arguments.
        // .clang_arg(format!("-I{}/include", dst.display()))
        // .clang_arg(format!("-I{}/include/azureiot", dst.display()))
        .clang_arg(format!("-I{}/include", path_lib_azuresdk.display()))
        .clang_arg(format!("-I{}/include/azureiot", path_lib_azuresdk.display()))
        .clang_arg(format!("-I{}/include", path_lib_eisutils.display()))
        .clang_arg(format!("-I{}/include/aduc", path_lib_eisutils.display()))
        //.clang_arg("-DUSE_EDGE_MODULES")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
