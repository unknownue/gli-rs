
use std::fs;
use std::path::Path;
use std::ffi::CStr;

use crate::ffi::root::bindings;

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

    // Read the texture file content into bytes in Rust.
    let texture_data = fs::read(path)
        .map_err(|_| ErrorKind::Io)?;
    load_from_bytes(&texture_data)
}

/// Loads a texture(DDS, KTX or KMG) storage_linear from memory.
///
/// Return an error in case of failure.
pub fn load_from_bytes<T>(data: &[u8]) -> Result<T>
    where
        T: GliTexture {

    // Read the texture file content into bytes in Rust.
    let bytes_length = data.len();
    let texture_data = unsafe { CStr::from_bytes_with_nul_unchecked(data) };

    // let gli lib to handle the post-processing.
    // TODO: bytes_length may be invalid.
    let raw_texture = unsafe {
        bindings::Load::load_load(texture_data.as_ptr(), bytes_length)
    };

    let dst_texture = T::from(raw_texture);
    if dst_texture.empty() {
        Err(Error::load_texture("Failed to load."))
    } else {
        Ok(dst_texture)
    }
}



/// Loads a texture storage_linear from DDS file.
///
/// The file must be a valid DDS file.
///
/// Return an error in case of failure.
pub fn load_dds<T>(path: impl AsRef<Path>) -> Result<T>
    where
        T: GliTexture {

    let texture_data = fs::read(path)
        .map_err(|_| ErrorKind::Io)?;
    load_dds_from_bytes(&texture_data)
}

/// Loads a texture storage_linear from DDS memory.
///
/// Return an error in case of failure.
pub fn load_dds_from_bytes<T>(data: &[u8]) -> Result<T>
    where
        T: GliTexture {

    let bytes_length = data.len();
    let texture_data = unsafe { CStr::from_bytes_with_nul_unchecked(data) };

    let raw_texture = unsafe {
        bindings::Load::load_load_dds(texture_data.as_ptr(), bytes_length)
    };

    let dst_texture = T::from(raw_texture);
    if dst_texture.empty() {
        Err(Error::load_texture("Failed to load dds texture."))
    } else {
        Ok(dst_texture)
    }
}



/// Loads a texture storage_linear from KTX file.
///
/// The file must be a valid KTX file.
///
/// Return an error in case of failure.
pub fn load_ktx<T>(path: impl AsRef<Path>) -> Result<T>
    where
        T: GliTexture {

    let texture_data = fs::read(path)
        .map_err(|_| ErrorKind::Io)?;
    load_ktx_from_bytes(&texture_data)
}

/// Loads a texture storage_linear from KTX memory.
///
/// Return an error in case of failure.
pub fn load_ktx_from_bytes<T>(data: &[u8]) -> Result<T>
    where
        T: GliTexture {

    let bytes_length = data.len();
    let texture_data = unsafe { CStr::from_bytes_with_nul_unchecked(data) };

    let raw_texture = unsafe {
        bindings::Load::load_load_ktx(texture_data.as_ptr(), bytes_length)
    };

    let dst_texture = T::from(raw_texture);
    if dst_texture.empty() {
        Err(Error::load_texture("Failed to load ktx texture."))
    } else {
        Ok(dst_texture)
    }
}



/// Loads a texture storage_linear from KMG (Khronos Image) file.
///
/// The file must be a valid KMG file.
///
/// Return an error in case of failure.
pub fn load_kmg<T>(path: impl AsRef<Path>) -> Result<T>
    where
        T: GliTexture {

    let texture_data = fs::read(path)
        .map_err(|_| ErrorKind::Io)?;
    load_kmg_from_bytes(&texture_data)
}

/// Loads a texture storage_linear from KMG (Khronos Image) memory.
///
/// Return an error in case of failure.
pub fn load_kmg_from_bytes<T>(data: &[u8]) -> Result<T>
    where
        T: GliTexture {

    let bytes_length = data.len();
    let texture_data = unsafe { CStr::from_bytes_with_nul_unchecked(data) };

    let raw_texture = unsafe {
        bindings::Load::load_load_kmg(texture_data.as_ptr(), bytes_length)
    };

    let dst_texture = T::from(raw_texture);
    if dst_texture.empty() {
        Err(Error::load_texture("Failed to load kmg texture."))
    } else {
        Ok(dst_texture)
    }
}
