use std::env;
use std::path::PathBuf;

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let crate_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();

    #[cfg(feature = "use-bindgen")]
    {
        let mut builder = bindgen::Builder::default().header("include/client.h");

        if target_os == "windows" {
            builder = builder
                .clang_arg("-DMPV_CPLUGIN_DYNAMIC_SYM")
                .clang_arg("-target")
                .clang_arg("x86_64-unknown-linux-gnu")
                .blocklist_function("mpv_.*");
        }

        let bindings = builder.generate().expect("Unable to generate bindings");
        bindings
            .write_to_file(out_path.join("bindings.rs"))
            .expect("Couldn't write bindings!");
    }

    #[cfg(not(feature = "use-bindgen"))]
    {
        let src = if target_os == "windows" {
            crate_path.join("pregenerated_bindings_windows.rs")
        } else {
            crate_path.join("pregenerated_bindings.rs")
        };

        std::fs::copy(src, out_path.join("bindings.rs")).expect("Couldn't find pregenerated bindings!");
    }
}
