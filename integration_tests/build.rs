extern crate cbindgen;

use std::env;

fn main() {

    println!("cargo:rustc-link-search=native=/home/jan/.local/share/sicstus4.8.0/lib");
    println!("cargo:rustc-link-lib=dylib=sprt4-8-0");
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let crate_name = env::var("CARGO_PKG_NAME").unwrap();

    cbindgen::Builder::new()
        .with_crate(crate_dir)
        .with_language(cbindgen::Language::C)
        .with_no_includes()
        .with_include(&format!("{}_glue.h", crate_name))
        .with_sys_include("sicstus/sicstus.h")
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(format!("{}.c", crate_name));
}
