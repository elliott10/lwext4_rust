use std::process::Command;
use std::env;
use std::path::PathBuf;

fn main() {
    let c_path = PathBuf::from("c/lwext4").canonicalize().expect("cannot canonicalize path");
    //let out_dir = env::var("OUT_DIR").unwrap();
    let arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let status = Command::new("make").args(&["musl-generic", "-C", c_path.to_str().expect("invalid path of lwext4")])
        .arg(&format!("ARCH={}", arch))
        .status().expect("failed to execute process: make lwext4");
    assert!(status.success());

    generates_bindings_to_rust();

    println!("cargo:rustc-link-lib=lwext4");
    println!("cargo:rustc-link-search=native={}", c_path.to_str().unwrap());
    println!("cargo:rerun-if-changed=c/wrapper.h");
    println!("cargo:rerun-if-changed={}", c_path.to_str().unwrap());
}

fn generates_bindings_to_rust() {
        let bindings = bindgen::Builder::default()
        .use_core()
        // The input header we would like to generate bindings for.
        .header("c/wrapper.h")
        //.clang_arg("--sysroot=/path/to/sysroot")
        .clang_arg("-I../../ulib/axlibc/include")
        .clang_arg("-I./c/lwext4/include")
        .clang_arg("-I./c/lwext4/build_musl-generic/include/")
        .layout_tests(false)
        // Tell cargo to invalidate the built crate whenever any of the included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Finish the builder and generate the bindings.
        .generate()
        .expect("Unable to generate bindings");

        // Write the bindings to the $OUT_DIR/bindings.rs file.
        let out_path = PathBuf::from("src");
        bindings
            .write_to_file(out_path.join("bindings.rs"))
            .expect("Couldn't write bindings!");
}