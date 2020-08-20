# x86_64 library

[![Crates.io](https://img.shields.io/crates/v/x86_64)](https://crates.io/crates/x86_64)
[![Build Status](https://github.com/rust-osdev/x86_64/workflows/Build/badge.svg)](https://github.com/rust-osdev/x86_64/actions?query=workflow%3ABuild) [![docs.rs](https://img.shields.io/badge/docs.rs-documentation-green.svg)](https://docs.rs/x86_64)

Support for x86_64 specific instructions (e.g. TLB flush), registers (e.g. control registers), and structures (e.g. page tables).

## Crate Feature Flags

* `nightly`: This is the default.
* `stable`: Use this to build with non-nightly rust. Needs `default-features = false`.

## Building with stable rust

This needs to have the [compile-time requirements](https://github.com/alexcrichton/cc-rs#compile-time-requirements) of the `cc` crate installed on your system.
It was currently only tested on Linux and MacOS.
