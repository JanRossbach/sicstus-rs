extern crate cbindgen;

use std::env;

fn gen_c_bindings(crate_dir: String, crate_name: String) {
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

fn main() {

    println!("cargo:rustc-link-search=native=/home/jan/.local/share/sicstus4.8.0/lib");
    println!("cargo:rustc-link-lib=dylib=sprt4-8-0");

    let crate_dir: String = env::var("CARGO_MANIFEST_DIR").unwrap();
    let crate_name: String = env::var("CARGO_PKG_NAME").unwrap();
    let _out_dir: String = env::var("OUT_DIR").unwrap();
    gen_c_bindings(crate_dir, crate_name);
}
