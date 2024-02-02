use regex;
use std::env;
use std::path::PathBuf;
use std::process::Command;

static SUPPORTED_SICSTUS_VERSIONS: [&str; 3] = ["4.7.1", "4.8.0", "4.9.0"];

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

fn generate_bindings(sicstus_root_dir: PathBuf, out_path: PathBuf) {
    let bindings = bindgen::Builder::default()
        .use_core()
        .header(
            sicstus_root_dir
                .join("include")
                .join("sicstus")
                .join("sicstus.h")
                .to_str()
                .unwrap(),
        )
        .clang_arg(format!(
            "-I{}",
            sicstus_root_dir.join("include").to_str().unwrap()
        ))
        .generate()
        .expect("Unable to generate bindings");
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn get_sicstus_version(sicstus_root_dir: PathBuf) -> String {
    let output: String = Command::new(sicstus_root_dir.join("bin").join("sicstus"))
        .arg("--version")
        .output()
        .expect("failed to get sicstus version")
        .stdout
        .iter()
        .map(|&c| c as char)
        .collect();
    // find the version number in the output
    let re = regex::Regex::new(r"(\d+\.\d+\.\d+)").unwrap();
    let caps = re.captures(&output).unwrap();
    caps.get(1).unwrap().as_str().to_string()
}

fn main() {
    let sicstus_root_dir = find_sicstus_root_dir().expect("failed to find sicstus root dir. Set SP_PATH or SICSTUSDIR environment variable in CARGO_MANIFEST_DIR/.cargo/config.toml to configure it manually.");
    let sicstus_version = get_sicstus_version(sicstus_root_dir.clone());
    assert!(
        SUPPORTED_SICSTUS_VERSIONS.contains(&&sicstus_version[..]),
        "Unsupported sicstus version {}. Supported versions are: {:?}",
        sicstus_version,
        SUPPORTED_SICSTUS_VERSIONS
    );
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    generate_bindings(sicstus_root_dir.clone(), out_path.clone());
    println!("cargo:rustc-cfg=sicstus_version=\"{}\"", sicstus_version); // for conditional compilation
}
