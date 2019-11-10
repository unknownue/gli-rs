# gli-rs

[![Build Status](https://dev.azure.com/usami-ssc/usami-ssc/_apis/build/status/unknownue.gli-rs?branchName=master)](https://dev.azure.com/usami-ssc/usami-ssc/_build/latest?definitionId=6&branchName=master) [![Latest version](https://img.shields.io/badge/crates.io-gli--rs-green.svg)](https://crates.io/crates/gli-rs) [![Documentation](https://docs.rs/gli-rs/badge.svg)](https://docs.rs/gli-rs) ![MIT](https://img.shields.io/badge/license-MIT-blue.svg)

Rust binding for [OpenGL Image(GLI)](https://github.com/g-truc/gli) library.

This crate aims to fill the gap of loading KTX and DDS image file in Rust ecosystem.

- [Documentation](https://docs.rs/gli-rs)
- [Change Logs](./CHANGELOG.md)
- [GLI GitHub](https://github.com/g-truc/gli)

## Compatibility

Although not all the api has been translated to Rust and fully tested, it does work for the most part.

This crate does not guarantee full compatibility with original library.

## Requirement

To build this crate, you must get C++ build tools installed(clang on MacOS, MSVC on Windows, gcc on Linux).

## Usage

Add this to `Cargo.toml`:

```toml
[dependencies]
gli-rs = "0.3.1"

# or
[dependencies]
gli = { package = "gli-rs", version = "0.3.1" }
```

## Get help

If you get any questions or find any api is missing, welcome to create an issue.

