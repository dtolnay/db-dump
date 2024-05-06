fn main() {
    // Warning: build.rs is not published to crates.io.

    println!("cargo:rustc-check-cfg=cfg(db_dump_panic_on_unrecognized_csv)");
}
