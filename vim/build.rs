fn main() {
    // Capture Rustc version at compile time. Used in core::client to set the User-Agent header.
    let version = rustc_version::version().unwrap();
    println!("cargo:rustc-env=RUSTC_VERSION={}", version);
}