# gli-rs

Rust binding for [OpenGL Image(GLI)](https://github.com/g-truc/gli) library.
![MIT](https://img.shields.io/badge/license-MIT-blue.svg)

- [Documentation](https://docs.rs/gli-rs)
- [Change Logs](./CHANGES.md)
- [GLI GitHub](https://github.com/g-truc/gli)

This crate aims to fill the gap of loading KTX and DDS image file for graphic developer in Rust ecosystem.

Although not all the api has been translated to Rust and fully tested, it does work for the most part.

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

