extern crate bindgen;
use std::env;
use std::path::PathBuf;

fn main() {
    let link_paths;
    let include_paths;
    let env_path_vars = [
        "AZURESDK_PATH",
        "UUID_PATH",
        "OPENSSL_PATH",
        "CURL_PATH",
    ];

    let env_paths: Vec<Option<String>> = env_path_vars
        .into_iter()
        .map(|k| env::var(k).ok())
        .collect();

    let (use_iotedge_modules, pkg_name) = if env::var("CARGO_FEATURE_EDGE_MODULES").is_ok() {
        ("-DUSE_EDGE_MODULES", "azure-iotedge-sdk-dev")
    } else {
        ("", "azure-iot-sdk-dev")
    };

    match env_paths.contains(&None) {
        // at least one PATH env is not set
        // thus fall back to pkg_config
        true => {
            println!("build from pkg_config paths");
            let lib = pkg_config::Config::new().probe(pkg_name).unwrap();
            link_paths = lib.link_paths;
            include_paths = lib.include_paths;
        }
        // all PATH envs are set
        // thus use those paths instead of pkg_config
        false => {
            println!("build from environment paths");
            link_paths = env_paths
                .clone()
                .into_iter()
                .map(|p| PathBuf::from(p.unwrap()))
                .collect();

            include_paths = vec![
                PathBuf::from(format!("{}/include", env_paths[0].clone().unwrap())),
                PathBuf::from(format!("{}/include/azureiot", env_paths[0].clone().unwrap())),
            ];

            env_path_vars
                .iter()
                .for_each(|env| println!("cargo:rerun-if-changed={}", env));
        }
    }

    link_paths
        .iter()
        .for_each(|p| println!("cargo:rustc-link-search=native={}/lib", p.display()));

    // link the azure iot-sdk libraries.
    // order of libraries matters!
    println!("cargo:rustc-link-lib=iothub_client_mqtt_transport");
    println!("cargo:rustc-link-lib=iothub_client");
    println!("cargo:rustc-link-lib=parson");
    println!("cargo:rustc-link-lib=umqtt");

    if env::var("CARGO_FEATURE_EDGE_MODULES").is_ok() {
        println!("cargo:rustc-link-lib=prov_auth_client");
        println!("cargo:rustc-link-lib=hsm_security_client");
    }

    println!("cargo:rustc-link-lib=uhttp");
    println!("cargo:rustc-link-lib=aziotsharedutil");
    println!("cargo:rustc-link-lib=curl");
    println!("cargo:rustc-link-lib=ssl");
    println!("cargo:rustc-link-lib=crypto");
    println!("cargo:rustc-link-lib=uuid");

    // invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=CARGO_FEATURE_EDGE_MODULES");

    let bindings = bindgen::Builder::default()
        // input header we would like to generate bindings for.
        .header("wrapper.h")
        // enable/disable iotedge
        .clang_arg(use_iotedge_modules)
        .clang_args(
            include_paths
                .iter()
                .map(|path| format!("-I{}", path.to_string_lossy())),
        )
        // seems it is a controverse topic, but for 32bit vs 64 bit systems we
        // need size_t to be usize to be able to bind callbacks
        // https://github.com/rust-lang/rust-bindgen/issues/1901 vs
        // https://github.com/rust-lang/rust-bindgen/issues/1671
        .size_t_is_usize(true)
        // blacklist time_t so it doesn't get i32 on 32bit systems and set
        // it appropriately
        .blocklist_type("time_t")
        .raw_line("pub type time_t = std::os::raw::c_longlong;")
        // invalidate the built crate whenever any of the included header files changed
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // generate bindings
        .generate()
        .expect("unable to generate bindings");

    // write bindings to the $OUT_DIR/bindings.rs
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
