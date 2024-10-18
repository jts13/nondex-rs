# nondex-rs

`nondex-rs` is a tool for detecting and debugging assumptions of non-deterministic and 'implementation-dependent'
behavior.

Operations, such as iterating over the entries in a hash-map, are sometimes 'under-determined' in the standard libraries
of programming languages. Developers, focused on writing tests for the business logic and requirements with which they
are tasked, often take the behind of libraries - especially the standard library of the language - as deterministic and
straight-forward. When the behavior is actually - strictly-speaking - unspecified, this can result in non-deterministic
behavior and 'implementation-dependent' _flaky_ tests.

* [Technical notes](NOTES.md)

## Getting Started

First, we'll need Rust, so [rustup](https://rustup.rs/) must be installed.

To run `nondex-rs` on a local crate:

```shell
./x.sh test --crate tests 
```

To run `nondex-rs` on a remotely-hosted crate in a Git repository:

```shell
./x.sh test --crate hashbrown --repo https://github.com/rust-lang/hashbrown
```
