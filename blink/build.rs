use std::{env, fs, path::Path};

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    fs::copy("memory.x", Path::new(&out_dir).join("memory.x")).unwrap();
    println!("cargo:rustc-link-search={}", out_dir);
    println!("cargo:rerun-if-changed=memory.x");
}
