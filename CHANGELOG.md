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

