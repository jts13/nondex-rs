# Technical Notes

## sysroot

The "sysroot" is where `rustc` looks for the crates that come with the Rust distribution. Typically, the sysroot is
installed by 'rustup' as part of installing a particular toolchain. Rustup can be used to include additional
[components](https://rust-lang.github.io/rustup/concepts/components.html), like `rust-src`, which will be added to the
corresponding sysroot.

The active sysroot can be found via:

```shell
rustc --print sysroot
```

`nondex-rs` must make modifications to the source code of the standard library, and it provides this modified `std`
to Cargo via a custom sysroot.

The process - which is manual today - for creating this modified standard library is as follows:

* Pin to a particular _nightly_ toolchain via [rust-toolchain.toml](rust-toolchain.toml)
* Fetch the `rust-src` for that pinned toolchain version
    * In general, links to the various components can be
      found [here](https://static.rust-lang.org/dist/channel-rust-nightly.toml)
    * In our case, we're
      using [dist/2024-10-02/rust-src-nightly.tar.xz](https://static.rust-lang.org/dist/2024-10-02/rust-src-nightly.tar.xz)

### Building when a custom sysroot

Note that the Cargo support for building with a custom standard library is unstable and limited.

Here's one approach - from https://github.com/rust-lang/wg-cargo-std-aware/issues/7#issuecomment-1579283605 - which
importantly called out the need to `cargo clean` for modifications to standard library to be rebuilt:

```
Here's a workaround for experimenting with changes to std

> To set up, run:
>
> cp --recursive ~/.rustup/toolchains/nightly-* new-directory-path
> rustup toolchain link new-toolchain-name new-directory-path
>
> Make changes in new-directory-path/lib/rustlib/src/rust/library/std
> Use cargo +new-toolchain-name with -Z build-std to build a crate using the modified standard library
> When the standard library is modified again, the changes will not take effect until you remove
> the artifacts from the target directory in your crate. To do this when using release mode, run:
>
> rm -rf target/*/release/build/std-*
```

`nondex-rs` uses a slightly different approach: [
`__CARGO_TESTS_ONLY_SRC_ROOT`](https://github.com/rust-lang/cargo/blob/643a025b3c3ad6f7d3acea558d223784ea8ab932/src/cargo/core/compiler/standard_lib.rs#L184-L186)
is an undocumented environment variable that can be used to redirect Cargo to a specified location for the 'sysroot'
used to build the crate(s).

## Thoughts on how to upgrade this with newer versions of Rust

* proc-macro for wrapping iterators?
