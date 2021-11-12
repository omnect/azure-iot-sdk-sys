extern crate bindgen;

use glob::glob;
use std::env;
use std::path::PathBuf;

fn path_handling(env: &str) -> String {
    let not_set = format!("env {} is not available", env);
    let error_glob = format!("Failed to read glob pattern for {}", env);

    let path = env::var(env).expect(&not_set);
    let mut path_lib = vec![];
    let mut index = 0;

    for entry in glob(&path.to_string()).expect(&error_glob) {
        match entry {
            Ok(path) => {
                path_lib.push(path.display().to_string());
            }
            Err(e) => panic!("LIB_PATH: {}", e),
        }
        index += 1;
    }
    if index > 1 {
        for libs in &path_lib {
            println!("{}", libs)
        }
        panic!("Multiple paths found for {}", env);
    }
    path_lib[0].clone()
}

fn main() {
    let path_lib_azuresdk = path_handling("LIB_PATH_AZURESDK");
    let path_lib_eisutils = path_handling("LIB_PATH_EISUTILS");
    let path_lib_uuid = path_handling("LIB_PATH_UUID");
    let path_lib_openssl = path_handling("LIB_PATH_OPENSSL");
    let path_lib_curl = path_handling("LIB_PATH_CURL");

    // Tell cargo to tell rustc the search path of the libraries
    println!(
        "cargo:rustc-link-search=native={}/lib",
        path_lib_azuresdk.to_string()
    );
    println!(
        "cargo:rustc-link-search=native={}/lib",
        path_lib_eisutils.to_string()
    );
    println!(
        "cargo:rustc-link-search=native={}/lib",
        path_lib_uuid.to_string()
    );
    println!(
        "cargo:rustc-link-search=native={}/lib",
        path_lib_openssl.to_string()
    );
    println!(
        "cargo:rustc-link-search=native={}/lib",
        path_lib_curl.to_string()
    );

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
        .clang_arg(format!("-I{}/include", path_lib_azuresdk.to_string()))
        .clang_arg(format!(
            "-I{}/include/azureiot",
            path_lib_azuresdk.to_string()
        ))
        .clang_arg(format!("-I{}/include", path_lib_eisutils.to_string()))
        .clang_arg(format!("-I{}/include/aduc", path_lib_eisutils.to_string()))
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
