use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    // Get the out directory.
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());

    // Add out directory to the linker search path.
    println!("cargo:rustc-link-search={}", out.display());

    // Put the memory.x linker script somewhere the linker can find it.
    File::create(out.join("memory.x"))
        .unwrap()
        .write_all(include_bytes!("memory.x"))
        .unwrap();

    // TODO: Generate the EEPROM file. Add the ucsc-ectf-eeprom-layout src folder to the "rerun if changed".

    // Only re-run the build script when this file or memory.x is changed.
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=memory.x");
}
