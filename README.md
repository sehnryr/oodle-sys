# oodle-sys

Low-level auto-generated bindings for [Oodle](http://www.radgametools.com/oodle.htm).

## Build-time bindgen

This library includes a pre-generated `bindings.rs` file. It was generated using
the `bindgen` tool, and is provided for convenience to avoid the need to install
`bindgen`. You can also generate it yourself by running `cargo build` with the
`bindgen` feature enabled :
```sh
cargo build --features bindgen
```
