# Fluffer Changelog

## 2023.10.25
* Finalized certificate identity functions.
* Switched to using the `sanitize-filename` crate for `Fluff::File`.
* Trimmed whitespace after delimiting in `examples/dice.rs`

## 2023.10.27
* Require both `cert.pem` and `key.pem` to be missing before triggering
  certificate prompt.
* Removed anyhow dependencies, and switched to two custom error enums: the
  private `StreamErr`, and the public `AppErr`.
* Removed emojis in app's default `not_found` and `Fluffer::File` messages, due
  to some clients not expecting unicode in the meta tag.
* Added `ident_expired` function.
* Added an example of custom errors (`examples/errors.rs`), and a trick to hide
  pages from unauthorized users (`examples/admin.rs`).

## 2023.10.27 - 0.1.0
* Switched to semantic versioning 🤦
* Removed broken `Box` impl.
* Added example for handling routes that can return multiple
  types(`examples/mult.rs`.
* *breaking* Changed `Context` to `Client` to avoid naming conflicts with
  Tera, and other crates. sorry :(

## 2023.10.28 - 0.1.1
* Added `GemBytes` impls for `reqwest::Response` and
  `reqwest::Result<reqwest::Response>`.
* Added an http proxy example using `reqwest` (`example/http.rs`).

## 2023.10.29 - 0.2.0
* Added mvp app state with a generic on `Client`.
* Added example (`example/app_state.rs`).

## 2023.11.02 - 0.3.0
* Added method to retrieve client's IP. (`Client::ip`)
* Removed fox emoji from default `not_found` message

## 2023.11.05 - 0.4.0
* Added `AsGemtext` trait.
* Added example demonstrating how to use `AsGemtext` to
  embed another route's response within a gemtext document
  (`example/as_gemtext.rs`).
* Added temporary/permanent failures to `Fluff`.
* Added option to change `cert` and `key` path in `App`.
* Improved gen_cert error handling.

## 2023.11.05 - 0.5.0
* Added `Static` wrapper around `GemBytes` types implement
  `GemCall` on byte types without a closure. (`examples/static.rs`)

## 2023.11.09 - 0.6.0
* Removed the `AsGemtext` trait.
* Added the `Client::render` function to replace `impl AsGemtext for Vec<u8>`.
* Implemented `GemBytes` for `anyhow::Error`.
* Set permissions to `600` for generated private key file.

## 2023.11.09 - 0.7.0
* Added re-export of `trotter::Status`
* Changed tuple impls of `GemBytes` to use `Into<u8>`
  generic so it can support `trotter::Status`
* Added `GemBytes` impl of `trotter::Response`
