
use std::path::Path;
use std::ffi::{CString, CStr};

use crate::ffi::root::bindings::Load as bindings;

use crate::format::TexFormatType;
use crate::texture::GliTexture;
use crate::error::{Result, Error, ErrorKind};


/// Loads a texture storage_linear from file.
///
/// The file must be a valid (DDS, KTX or KMG) file.
///
/// Return an error in case of failure.
pub fn load<T>(path: impl AsRef<Path>) -> Result<T>
    where
        T: GliTexture {

    let c_path = path_to_cstring(path)?;

    let raw_texture = unsafe {
        bindings::load_by_path(c_path.as_ptr())
    };

    construct_texture(raw_texture)
}

/// Construct a `GliTexture` by sharing another texture data in memory.
///
/// The `format` type and resulting texture type must be specified correctly.
///
/// The lifetime of `data` must be longer than constructed `GliTexture`, which is not guaranteed by this crate.
pub fn load_from_memory<T>(data: &[u8], format: TexFormatType) -> Result<T>
    where
        T: GliTexture {

    // Read the texture file content into bytes in Rust.
    let bytes_length = data.len();

    let raw_texture = unsafe {
        let texture_data = CStr::from_bytes_with_nul_unchecked(data);

        // TODO: bytes_length may be invalid.
        match format {
            TexFormatType::DDS => bindings::load_dds_memory(texture_data.as_ptr(), bytes_length),
            TexFormatType::KMG => bindings::load_kmg_memory(texture_data.as_ptr(), bytes_length),
            TexFormatType::KTX => bindings::load_ktx_memory(texture_data.as_ptr(), bytes_length),
        }
    };

    construct_texture(raw_texture)
}

/// Loads a texture storage_linear from DDS file.
///
/// The file must be a valid DDS file.
///
/// Return an error in case of failure.
pub fn load_dds<T>(path: impl AsRef<Path>) -> Result<T>
    where
        T: GliTexture {

    let c_path = path_to_cstring(path)?;

    let raw_texture = unsafe {
        bindings::load_dds_by_path(c_path.as_ptr())
    };

    construct_texture(raw_texture)
}

/// Loads a texture storage_linear from KTX file.
///
/// The file must be a valid KTX file.
///
/// Return an error in case of failure.
pub fn load_ktx<T>(path: impl AsRef<Path>) -> Result<T>
    where
        T: GliTexture {

    let c_path = path_to_cstring(path)?;

    let raw_texture = unsafe {
        bindings::load_ktx_by_path(c_path.as_ptr())
    };

    construct_texture(raw_texture)
}

/// Loads a texture storage_linear from KMG (Khronos Image) file.
///
/// The file must be a valid KMG file.
///
/// Return an error in case of failure.
pub fn load_kmg<T>(path: impl AsRef<Path>) -> Result<T>
    where
        T: GliTexture {

    let c_path = path_to_cstring(path)?;

    let raw_texture = unsafe {
        bindings::load_kmg_by_path(c_path.as_ptr())
    };

    construct_texture(raw_texture)
}

#[inline]
fn construct_texture<T>(raw_texture: crate::ffi::root::gli::texture) -> Result<T>
    where
        T: GliTexture {

    let dst_texture = T::from(raw_texture);

    // gli failed to load the texture, if its return variable is empty.
    if dst_texture.empty() {
        Err(Error::load_texture("Failed to load."))
    } else {
        Ok(dst_texture)
    }
}

#[inline]
fn path_to_cstring(path: impl AsRef<Path>) -> Result<CString> {

    // Some conversion from Path to CString.
    // If you find better way to do this, welcome to create a pull request.

    let path_str = path.as_ref().to_str()
        .ok_or(ErrorKind::Io)?;

    CString::new(path_str)
        .map_err(|_| ErrorKind::Io.into())
}
