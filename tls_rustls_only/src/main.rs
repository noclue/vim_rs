//! Compile-time check: `vim_rs` without `default-client` links only the consumer's TLS stack.
//!
//! Run `cargo tree -i openssl-sys` in this directory; expect no matches.

fn main() {
    let http = reqwest::Client::builder()
        .cookie_store(true)  // IMPORTANT!!! SOAP/XML servers use cookies for session auth
        .build()
        .expect("reqwest client");
    let _builder = vim_rs::core::client::ClientBuilder::new("vcenter.example.com", http);
}
