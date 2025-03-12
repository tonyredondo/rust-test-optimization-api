use std::{env, fs, process};
use std::path::PathBuf;

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
    let lib_name = format!("{}-{}-libcivisibility.7z", platform, arch);
    let url = format!(
        "https://github.com/tonyredondo/rust-test-optimization-api/releases/download/v0.1.0/{}",
        lib_name
    );
    let lib_7z_path = PathBuf::from(out_dir.clone()).join("libcivisibility.7z");

    // Download the shared library
    println!("Downloading native library from: {}", url);
    let response = reqwest::blocking::get(&url)
        .unwrap_or_else(|e| {
            eprintln!("Failed to download native library: {}", e);
            process::exit(1);
        })
        .bytes()
        .unwrap_or_else(|e| {
            eprintln!("Failed to read response body: {}", e);
            process::exit(1);
        });

    // Write the binary to the output directory
    fs::write(&lib_7z_path, &response)
        .unwrap_or_else(|e| {
            eprintln!("Failed to write native library to disk: {}", e);
            process::exit(1);
        });

    sevenz_rust::decompress_file(lib_7z_path, PathBuf::from(out_dir.clone())).expect("Failed to decompress native library");

    let lib_filename = format!("{}-{}-libcivisibility", platform, arch);
    let lib_dir = PathBuf::from(out_dir.clone()).join(lib_filename);
    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    println!("cargo:rustc-link-lib=static=civisibility");

    if !target.contains("windows") {
        // Link to the dynamic dependency
        println!("cargo:rustc-link-lib=dylib=resolv");
    } else {
        // Windows version requires cc as a build-dependency
        #[cfg(target_os = "windows")]
        configure_windows();
    }

    // If we are in osx, we need to add a couple of frameworks
    if target.contains("apple-darwin") {
        println!("cargo:rustc-link-lib=framework=CoreFoundation");
        println!("cargo:rustc-link-lib=framework=IOKit");
        println!("cargo:rustc-link-lib=framework=Security");
    }
}

#[cfg(target_os = "windows")]
fn configure_windows() {
    // Windows target
    println!("cargo::rerun-if-changed=src/cgo.c");
    cc::Build::new()
        .file("src/cgo.c")
        .compile("cgo");

    // Link to the lib
    println!("cargo:rustc-link-lib=static=cgo");
}
