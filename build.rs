use std::path::PathBuf;

fn main() {
    // Get the location of the linker script
    let linker_script = PathBuf::from("src/linker.ld");

    // Tell Cargo to rerun this build script if the linker script changes
    println!("cargo:rerun-if-changed={}", linker_script.display());

    // Pass the linker script to the linker through rustc flags
    println!("cargo:rustc-link-arg=-T{}", linker_script.display());
}
