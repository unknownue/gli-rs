# gli-rs

Rust binding for [OpenGL Image(GLI)](https://github.com/g-truc/gli) library.

[![gli-rs on travis-ci.com](https://travis-ci.com/Usami-Renko/gli-rs.svg?branch=master)](https://travis-ci.com/Usami-Renko/gli-rs)
[![Latest version](https://img.shields.io/crates/g/gli-rs.svg)](https://crates.io/crates/gli-rs)
[![Documentation](https://docs.rs/gli-rs/badge.svg)](https://docs.rs/gli-rs)
![MIT](https://img.shields.io/badge/license-MIT-blue.svg)

- [Documentation](https://docs.rs/gli-rs)
- [Change Logs](./CHANGES.md)
- [GLI GitHub](https://github.com/g-truc/gli)

This crate aims to fill the gap of loading KTX and DDS image file for graphic developer in Rust ecosystem.

Although not all the api has been translated to Rust and fully tested, it does work for the most part.

This crate is still in early stage, and failed to build on Windows and Linux.

## Usage

Add this to `Cargo.toml`:

```toml
[dependencies]
gli-rs = "0.2.0"

# or
[dependencies]
gli = { package = "gli-rs", version = "0.2.0" }
```

## Get help

Welcome to create issues if you have any questions or find any of api is missing.

## License

The repository is under [MIT license](http://opensource.org/licenses/MIT).

