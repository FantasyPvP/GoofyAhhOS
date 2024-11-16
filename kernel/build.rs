use std::process::Command;
use std::{env, path::Path};

fn main() {
    // Get the output directory from cargo
    let out_dir = env::var("OUT_DIR").unwrap();
    let build_dir = Path::new(&out_dir).ancestors().nth(3).unwrap();
    let iso_root = build_dir.join("iso_root");

    // Create ISO directory structure
    std::fs::create_dir_all(iso_root.join("boot/limine")).unwrap();
    std::fs::create_dir_all(iso_root.join("EFI/BOOT")).unwrap();

    // Clone and build Limine if needed
    let limine_dir = build_dir.join("limine");
    if !limine_dir.exists() {
        Command::new("git")
            .args([
                "clone",
                "https://github.com/limine-bootloader/limine.git",
                "--branch=v8.x-binary",
                "--depth=1",
            ])
            .arg(&limine_dir)
            .status()
            .expect("Failed to clone Limine");

        Command::new("make")
            .current_dir(&limine_dir)
            .status()
            .expect("Failed to build Limine");
    }

    // Tell cargo to rerun if these files change
    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed=linker.ld");
    println!("cargo:rerun-if-changed=../config/limine.conf");
}
