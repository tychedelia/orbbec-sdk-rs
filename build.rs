use std::env;
use std::path::PathBuf;

fn main() {
    let project_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    // Tell cargo to look for shared libraries in the specified directory
    println!("cargo:rustc-link-search={project_dir}/OrbbecSDK/lib/macOS/");
    // Tell cargo to link the shared library named "libOrbbecSDK.dylib"
    println!("cargo:rustc-link-lib=dylib=OrbbecSDK");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        .clang_arg("-I./OrbbecSDK/include/")
        .header("./OrbbecSDK/include/libobsensor/ObSensor.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}