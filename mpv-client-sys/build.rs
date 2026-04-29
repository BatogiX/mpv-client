use std::env;
use std::path::PathBuf;

#[cfg(not(feature = "use-bindgen"))]
#[cfg(target_os = "linux")]
fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let crate_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    std::fs::copy(
        crate_path.join("pregenerated_bindings.rs"),
        out_path.join("bindings.rs"),
    )
    .expect("Couldn't find pregenerated bindings!");
}

#[cfg(feature = "use-bindgen")]
#[cfg(target_os = "linux")]
fn main() {
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    println!("cargo:rustc-link-lib=mpv");
}

#[cfg(not(feature = "use-bindgen"))]
#[cfg(target_os = "windows")]
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

#[cfg(feature = "use-bindgen")]
#[cfg(target_os = "windows")]
fn main() {
    let crate_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    let bindings = bindgen::Builder::default()
        .header("include/client.h")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    println!("cargo:rustc-link-search=native={}", crate_path.display());
    println!("cargo:rustc-link-lib=mpv");
}
