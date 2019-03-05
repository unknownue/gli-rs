
use std::fs;
use std::path::Path;
use std::ffi::CString;

use crate::ffi::root::gli;
use crate::texture::inner::TextureAccessible;
use crate::error::{Result, ErrorKind};


/// Loads a texture storage_linear from file.
///
/// The file must be a valid (DDS, KTX or KMG) file.
///
/// Returns an empty storage_linear in case of failure(use `is_empty()` method to check it).
pub fn load<T>(path: impl AsRef<Path>) -> Result<T>
    where
        T: TextureAccessible {

    // Read the texture file content into bytes in Rust.
    let texture_data = fs::read(path)
        .map_err(|_| ErrorKind::Io)?;
    let texture = load_from_bytes(texture_data);
    Ok(texture)
}

/// Loads a texture(DDS, KTX or KMG) storage_linear from memory.
///
/// Returns an empty storage_linear in case of failure(use `is_empty()` method to check it).
pub fn load_from_bytes<T>(data: Vec<u8>) -> T
    where
        T: TextureAccessible {

    // Read the texture file content into bytes in Rust.
    let bytes_length = data.len();
    let texture_data = unsafe { CString::from_vec_unchecked(data) };

    // let gli lib to handle the post-processing.
    // TODO: bytes_length may be invalid.
    let raw_texture = unsafe {
        gli::load2(texture_data.as_ptr(), bytes_length)
    };

    T::from(raw_texture)
}



/// Loads a texture storage_linear from DDS file.
///
/// The file must be a valid DDS file.
///
/// Returns an empty storage_linear in case of failure(use `is_empty()` method to check it).
pub fn load_dds<T>(path: impl AsRef<Path>) -> Result<T>
    where
        T: TextureAccessible {

    let texture_data = fs::read(path)
        .map_err(|_| ErrorKind::Io)?;
    let texture = load_dds_from_bytes(texture_data);
    Ok(texture)
}

/// Loads a texture storage_linear from DDS memory.
///
/// Returns an empty storage_linear in case of failure(use `is_empty()` method to check it).
pub fn load_dds_from_bytes<T>(data: Vec<u8>) -> T
    where
        T: TextureAccessible {

    let bytes_length = data.len();
    let texture_data = unsafe { CString::from_vec_unchecked(data) };

    let raw_texture = unsafe {
        gli::load_dds2(texture_data.as_ptr(), bytes_length)
    };

    T::from(raw_texture)
}



/// Loads a texture storage_linear from KTX file.
///
/// The file must be a valid KTX file.
///
/// Returns an empty storage_linear in case of failure(use `is_empty()` method to check it).
pub fn load_ktx<T>(path: impl AsRef<Path>) -> Result<T>
    where
        T: TextureAccessible {

    let texture_data = fs::read(path)
        .map_err(|_| ErrorKind::Io)?;
    let texture = load_ktx_from_bytes(texture_data);
    Ok(texture)
}

/// Loads a texture storage_linear from KTX memory.
///
/// Returns an empty storage_linear in case of failure(use `is_empty()` method to check it).
pub fn load_ktx_from_bytes<T>(data: Vec<u8>) -> T
    where
        T: TextureAccessible {

    let bytes_length = data.len();
    let texture_data = unsafe { CString::from_vec_unchecked(data) };

    let raw_texture = unsafe {
        gli::load_ktx2(texture_data.as_ptr(), bytes_length)
    };

    T::from(raw_texture)
}



/// Loads a texture storage_linear from KMG (Khronos Image) file.
///
/// The file must be a valid KMG file.
///
/// Returns an empty storage_linear in case of failure(use `is_empty()` method to check it).
pub fn load_kmg<T>(path: impl AsRef<Path>) -> Result<T>
    where
        T: TextureAccessible {

    let texture_data = fs::read(path)
        .map_err(|_| ErrorKind::Io)?;
    let texture = load_kmg_from_bytes(texture_data);
    Ok(texture)
}

/// Loads a texture storage_linear from KMG (Khronos Image) memory.
///
/// Returns an empty storage_linear in case of failure(use `is_empty()` method to check it).
pub fn load_kmg_from_bytes<T>(data: Vec<u8>) -> T
    where
        T: TextureAccessible {

    let bytes_length = data.len();
    let texture_data = unsafe { CString::from_vec_unchecked(data) };

    let raw_texture = unsafe {
        gli::load_kmg2(texture_data.as_ptr(), bytes_length)
    };

    T::from(raw_texture)
}
