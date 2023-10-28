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
