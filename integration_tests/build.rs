extern crate cbindgen;

use std::env;
use std::path::PathBuf;

fn find_sicstus_root_dir() -> Option<PathBuf> {
    if let Ok(sp_path) = env::var("SICSTUSDIR") {
        return Some(PathBuf::from(sp_path));
    }

    // Check if sicstus is on the path
    let cmd = "sicstus";
    let status = std::process::Command::new(cmd).arg("--version").status();

    if status.is_ok() && status.unwrap().success() {
        let output = std::process::Command::new("which")
            .arg(cmd)
            .output()
            .expect("failed to use 'which' command to find sicstus");
        let sicstus_path =
            String::from_utf8(output.stdout).expect("failed to convert output to string");
        let sicstus_path = sicstus_path.trim();
        let sicstus_path = PathBuf::from(sicstus_path);
        let sicstus_path = sicstus_path
            .parent()
            .expect("failed to get parent dir of sicstus")
            .parent()
            .expect("failed to get parent dir of sicstus");
        return Some(sicstus_path.to_path_buf());
    }
    None
}

fn gen_c_bindings(crate_dir: String, crate_name: String) {
    let filename = format!("{}.c", crate_name);
    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_language(cbindgen::Language::C)
        .with_no_includes()
        .with_include(&format!("{}_glue.h", crate_name))
        .with_sys_include("sicstus/sicstus.h")
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(filename);
}

fn main() {
    let sicstus_root_dir = find_sicstus_root_dir().expect("failed to find sicstus root dir. Set SP_PATH or SICSTUSDIR environment variable in CARGO_MANIFEST_DIR/.cargo/config.toml to configure it manually.");
    println!("cargo:rustc-link-search=native={}lib", sicstus_root_dir.to_str().unwrap());
    println!("cargo:rustc-link-lib=dylib=sprt4-8-0");

    let crate_dir: String = env::var("CARGO_MANIFEST_DIR").unwrap();
    let crate_name: String = env::var("CARGO_PKG_NAME").unwrap();
    let _out_dir: String = env::var("OUT_DIR").unwrap();
    gen_c_bindings(crate_dir, crate_name);
}
