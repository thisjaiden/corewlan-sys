fn main() {
    // Credit to https://kazlauskas.me/entries/writing-proper-buildrs-scripts for this gem:
    let target_os = std::env::var("CARGO_CFG_TARGET_OS");
    match target_os.as_ref().map(|x| &**x) {
        Ok("macos") => println!("cargo:rustc-link-lib=framework=CoreWLAN"),
        Ok(other_os) => panic!("Cannot build `corewlan-sys` for {}.", other_os),
        _ => panic!("Cannot build `corewlan-sys` for an unknown OS."),
    }
    let target_pointer_width = std::env::var("CARGO_CFG_TARGET_POINTER_WIDTH");
    match target_pointer_width.as_ref().map(|x| &**x) {
        Ok("64") => {}, // continue!
        Ok("32") => panic!("Cannot build `corewlan-sys` for a 32 bit target. If you really need this, open an issue!"),
        _ => panic!("Cannot build `corewlan-sys` for a target with unknown pointer width."),
    }
}
