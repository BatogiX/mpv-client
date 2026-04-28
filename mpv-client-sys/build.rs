use std::env;
use std::path::PathBuf;

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let crate_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    std::fs::copy(
        crate_path.join("pregenerated_bindings.rs"),
        out_path.join("bindings.rs"),
    )
    .expect("Couldn't find pregenerated bindings!");

    println!("cargo:rustc-link-search=native={}", crate_path.display());
    println!("cargo:rustc-link-lib=mpv");
}
