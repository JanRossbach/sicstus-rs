use std::env;
use std::path::PathBuf;

fn find_sicstus_root_dir() -> PathBuf {
    if let Ok(sp_path) = env::var("SP_PATH") {
        return PathBuf::from(sp_path);
    }

    if let Ok(sp_path) = env::var("SICSTUSDIR") {
        return PathBuf::from(sp_path);
    }

    let cmd = "sicstus";
    let status = std::process::Command::new(cmd)
        .arg("--version")
        .status()
        .expect("failed to execute process");

    if status.success() {
        let output = std::process::Command::new("which")
            .arg(cmd)
            .output()
            .expect("failed to find sicstus with the which command. Set SP_PATH or SICSTUSDIR environment variable in CARGO_MANIFEST_DIR/.cargo/config.toml to configure it manually.");
        let sicstus_path = String::from_utf8(output.stdout).expect("failed to convert output to string");
        let sicstus_path = sicstus_path.trim();
        let sicstus_path = PathBuf::from(sicstus_path);
        let sicstus_path = sicstus_path.parent().expect("failed to get parent dir of sicstus").parent().expect("failed to get parent dir of sicstus");
        return sicstus_path.to_path_buf();
    } else {
        panic!("failed to find sicstus with the which command. Set SP_PATH or SICSTUSDIR environment variable.");
    }
}

fn main() {
    // Tell cargo to look for shared libraries in the specified directory
    //println!("cargo:rustc-link-search=/path/to/lib");

    let sicstus_root_dir = find_sicstus_root_dir();

    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    //println!("cargo:rustc-link-lib=bz2");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    //println!("cargo:rerun-if-changed=wrapper.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header(sicstus_root_dir.join("include").join("sicstus").join("sicstus.h").to_str().unwrap())
        // Some header files have relative paths to the include dir so we need to pass the include dir to clang.
        .clang_arg(format!("-I{}", sicstus_root_dir.join("include").to_str().unwrap()))
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        // .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
