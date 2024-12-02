use std::{env, fs};
use std::path::PathBuf;
use std::process;

fn main() {
    let target = env::var("TARGET").expect("Cargo did not provide TARGET");
    let out_dir = env::var("OUT_DIR").expect("Cargo did not provide OUT_DIR");

    // Detect arch and platform
    let (platform, arch) = if target.contains("apple-darwin") {
        ("macos", if target.contains("aarch64") { "arm64" } else { "x64" })
    } else if target.contains("windows") {
        ("windows", if target.contains("aarch64") { "arm64" } else { "x64" })
    } else if target.contains("linux") {
        ("linux", if target.contains("aarch64") { "arm64" } else { "x64" })
    } else {
        panic!("Unsupported platform: {}", target);
    };

    // Get the folder
    let lib_dir = format!("libs/{}/{}/", platform, arch);
    println!("cargo:rustc-link-search=native={}", lib_dir);

    // Link to the native library
    println!("cargo:rustc-link-lib=dylib=civisibility");

    // Create bindings from .h
    let bindings = bindgen::Builder::default()
        .header("include/libcivisibility.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(out_dir);
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    fs::copy(
        PathBuf::from(lib_dir).join("libcivisibility.dylib"),
        out_path.join("../../../libcivisibility.dylib")).unwrap_or_else(|e| {
        eprintln!("Failed to write native library to disk: {}", e);
        process::exit(1);
    });

    // If files changes let's rebuild everything
    println!("cargo:rerun-if-changed=include/libcivisibility.h");
    println!("cargo:rerun-if-changed=libs");
}
