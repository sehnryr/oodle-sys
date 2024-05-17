# oodle-sys

Low-level auto-generated bindings for [Oodle](http://www.radgametools.com/oodle.htm).

## Dependencies

This library binds to `liboo2corelinux64.so` on Linux and `oo2core_win64.lib` on Windows. MacOS isn't supported.
You can retrieve it by following the instructions here : <https://github.com/sehnryr/get-oodle-lib>

## Build-time bindgen

This library includes a pre-generated `bindings.rs` file. It was generated using
the `bindgen` tool, and is provided for convenience to avoid the need to install
`bindgen`. You can also generate it yourself by running `cargo build` with the
`bindgen` feature enabled:

```sh
cargo build --features bindgen
```
