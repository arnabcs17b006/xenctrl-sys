use std::{env, path::PathBuf};

static XEN_HEADERS_WRAPPER: &str = "src/wrapper.h";

fn main() {
    println!("cargo:rerun-if-changed={}", XEN_HEADERS_WRAPPER);

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header(XEN_HEADERS_WRAPPER)
        // Keep C's enums as Rust's enums.
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: false,
        })
        // Disable data layout tests.
        .layout_tests(false)
        // Run rustfmt on the bindings
        .rustfmt_bindings(true)
        //initialize structs with default values
        .derive_default(true)
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = {
        let out_path = env::var("OUT_DIR").expect("Unable to get OUT_DIR environment variable");
        PathBuf::from(out_path)
    };
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Unable to write bindings!");

    // what library to link with
    println!("cargo:rustc-link-lib={}={}", "dylib", "xenctrl");
}
