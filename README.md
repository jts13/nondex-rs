# nondex-rs

* [Technical notes](NOTES.md)

## Getting Started

First, we'll need Rust, so [rustup](https://rustup.rs/) must be installed.

To run `nondex-rs` on a local crate:

```shell
./x.sh test --crate tests 
```

To run `nondex-rs` on a remotely-hosted crate in a Git repository:

```shell
./x.sh test --repo https://github.com/rust-lang/hashbrown
```
