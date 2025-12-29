use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    // Tell cargo to invalidate the built crate whenever the linker script changes
    println!("cargo:rerun-if-changed=linker.ld");
    
    // Put the linker script somewhere the linker can find it
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    fs::copy("linker.ld", out.join("linker.ld")).unwrap();
    println!("cargo:rustc-link-search={}", out.display());
}

