use std::{env};

fn main() {
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let vendor_dir = match target_arch.as_str() {
        "x86_64" => "./lib/win/amd64",
        arch => panic!("x86_64 architecture needed ! (not {})", arch),
    };

    println!("cargo::rustc-link-search={}", vendor_dir);
    println!("cargo::rerun-if-changed=build.rs");
}
