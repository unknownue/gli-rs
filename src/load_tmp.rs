
use std::path::Path;
use std::ffi::CString;

use crate::ffi::root::gli;
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
        gli::load(c_path.as_ptr())
    };

    let dst_texture = T::from(raw_texture);

    // gli failed to load the texture, if its return variable is empty.
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

    let c_path = path_to_cstring(path)?;

    let raw_texture = unsafe {
        gli::load_dds(c_path.as_ptr())
    };

    let dst_texture = T::from(raw_texture);

    if dst_texture.empty() {
        Err(Error::load_texture("Failed to load."))
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

    let c_path = path_to_cstring(path)?;

    let raw_texture = unsafe {
        gli::load_ktx(c_path.as_ptr())
    };

    let dst_texture = T::from(raw_texture);

    if dst_texture.empty() {
        Err(Error::load_texture("Failed to load."))
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

    let c_path = path_to_cstring(path)?;

    let raw_texture = unsafe {
        gli::load_kmg(c_path.as_ptr())
    };

    let dst_texture = T::from(raw_texture);

    if dst_texture.empty() {
        Err(Error::load_texture("Failed to load."))
    } else {
        Ok(dst_texture)
    }
}

fn path_to_cstring(path: impl AsRef<Path>) -> Result<CString> {

    // Some conversion from Path to CString.
    // If you find better way to do this, welcome to create a pull request.

    let path_str = path.as_ref().to_str()
        .ok_or(ErrorKind::Io)?;

    CString::new(path_str)
        .map_err(|_| ErrorKind::Io.into())
}
