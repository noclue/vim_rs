# tls_rustls_only

Compile check for an OpenSSL-free `vim_rs` build where the application picks the rustls crypto provider.

`vim_rs` is depended on with `default-features = false`, so `default-client` is off and the library does not create a `reqwest` client. This crate builds one and passes it to `ClientBuilder::new(server, http)`.

reqwest uses the `rustls-no-provider` feature. A direct `rustls` dependency with the **ring** feature installs the crypto provider. reqwest's `rustls` feature would pull in **aws-lc-rs** instead; this crate keeps that out of the graph.

`src/main.rs` constructs the builder so those crates are linked. It does not log in.

The same opt-out, with a full connect sample, is in the [crate guide](../vim_rs/README.md#tls-and-your-own-http-client).

## Check the graph

From this directory:

```bash
cargo build
cargo tree -i openssl-sys
```

An empty result is the success case. Cargo prints `error: package ID specification 'openssl-sys' did not match any packages` and exits 101. A hit would print a tree rooted at `openssl-sys`.

`ring` is in the graph. `aws-lc-rs` is not (`cargo tree -i aws-lc-rs` fails the same way).

## Cookies

The reqwest client enables the `cookies` feature and `.cookie_store(true)`. SOAP session auth needs that store. This crate leaves `vim_rs`'s `xml` feature off; enable `xml` in your own manifest when you want the SOAP transport, and keep the cookie store.
