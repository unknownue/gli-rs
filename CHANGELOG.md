# Version 0.3.2 (2019-11-29)

- Add basic example codes.
- Mark `data_mut()` methods as `unsafe fn` for `GliTexture` and `GliImage`.
- Mark ffi newtype with `#[repr(transparent)]`.
- Make `TexFormatType` public to user. #3



# Version 0.3.1 (2019-4-7)

- Fix build error on Windows platform(Temporarily disable some API on Windows).
- Fix some parameter types.



# Version 0.3.0 (2019-4-1)

- Implement partial API in gli::Sampler* namespace.
- Implement API in gli::dx and gli::gl namespace.
- Implement `ParticalEq` and `Eq` for `GliImage` and `Texture*` structs.



# Version 0.2.1 (2019-3-14)

- Now gli-rs is able to build on main desktop platforms(By manually implement link header files).
- Refactor the `Extent1d`, `Extent2d`, `Extent3d` struct.
- Rename the constructor method for all `Texture*` structs.
- Fix some bugs and documentation typos.



# Version 0.2.0 (2019-3-7)

- First Release to crates.io. The crate mainly provide the following features:
  - Use `load_*` methods to load ktx, kmg or dds image file into `Texture*` style struct.
  - Save the image to ktx, kmg or dds format in local file system.
  - Access part of the `Texture*` image(Use `get_layer`, `get_level`, or `get_face` method).
  - Get the data pointer to the image, and transfer its data to other graphics program.

