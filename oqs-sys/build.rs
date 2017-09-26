extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    let out_dir = env::var("OUT_DIR").map(PathBuf::from).unwrap();

    let oqs_dir = env::var("OQS_DIR").map(PathBuf::from).expect(
        "Set the environment variable OQS_DIR to the absolute path to your liboqs dir",
    );
    let oqs_include_dir = oqs_dir.join("include");

    println!("cargo:rustc-link-lib=oqs");
    println!("cargo:rustc-link-lib=sodium");
    println!(
        "cargo:rustc-link-search=native={}",
        oqs_dir.to_string_lossy()
    );

    let _ = bindgen::builder()
        .header(format!("{}/oqs/kex.h", oqs_include_dir.to_string_lossy()))
        .clang_arg(format!("-I{}", oqs_include_dir.to_string_lossy()))
        .link_static("oqs")
        .use_core()
        .ctypes_prefix("::libc")
        .generate()
        .unwrap()
        .write_to_file(out_dir.join("kex.rs"))
        .unwrap();
}
