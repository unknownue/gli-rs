
use std::path::Path;
use std::ffi::OsStr;

use crate::ffi::root::bindings;
use crate::texture::GliTexture;
use crate::error::{Result, Error};
use std::ffi::CString;

// TODO: Some other save methods are not yet implemented.

/// Save a texture storage_linear file.
///
/// `texture` is texture to save.
///
/// `path` is the path for where to save the file. It must include the filename and filename extension.
///
/// The function use the filename extension included in the path to figure out the file container to use.
pub fn save(texture: &impl GliTexture, path: impl AsRef<Path>) -> Result<()> {

    if let Some(dst_extension) = path.as_ref().extension().and_then(OsStr::to_str) {
        match dst_extension {
            | "dds" => save_dds(texture, path),
            | "ktx" => save_ktx(texture, path),
            | "kmg" => save_kmg(texture, path),
            | _ => Err(Error::save_texture(format!("Saving {} format is not support.", dst_extension)))
        }
    } else {
        Err(Error::save_texture("Invalid path to save texture."))
    }
}

/// Save a texture storage_linear to a DDS file.
///
/// `texture` is texture to save.
///
/// `path` is the path for where to save the file. It must include the filename and filename extension.
///
/// This function ignores the filename extension in the path and save to DDS anyway but keep the requested filename extension.
pub fn save_dds(texture: &impl GliTexture, path: impl AsRef<Path>) -> Result<()> {

    let dst_path = path.as_ref().to_str()
        .and_then(|p| CString::new(p).ok())
        .ok_or(Error::save_texture("Invalid file path."))?;

    if unsafe { bindings::Save::save_save_dds(texture.raw_texture(), dst_path.as_ptr()) } {
        Ok(())
    } else {
        Err(Error::save_texture("Failed to save dds texture."))
    }
}

/// Save a texture storage_linear to a KTX file.
///
/// `texture` is texture to save.
///
/// `path` is the path for where to save the file. It must include the filename and filename extension.
///
/// This function ignores the filename extension in the path and save to KTX anyway but keep the requested filename extension.
pub fn save_ktx(texture: &impl GliTexture, path: impl AsRef<Path>) -> Result<()> {

    let dst_path = path.as_ref().to_str()
        .and_then(|p| CString::new(p).ok())
        .ok_or(Error::save_texture("Invalid file path."))?;

    if unsafe { bindings::Save::save_save_ktx(texture.raw_texture(), dst_path.as_ptr()) } {
        Ok(())
    } else {
        Err(Error::save_texture("Failed to save ktx texture."))
    }
}

/// Save a texture storage_linear to a KMG (Khronos Image) file.
///
/// `texture` is texture to save.
///
/// `path` is the path for where to save the file. It must include the filename and filename extension.
///
/// This function ignores the filename extension in the path and save to KMG anyway but keep the requested filename extension.
pub fn save_kmg(texture: &impl GliTexture, path: impl AsRef<Path>) -> Result<()> {

    let dst_path = path.as_ref().to_str()
        .and_then(|p| CString::new(p).ok())
        .ok_or(Error::save_texture("Invalid file path."))?;

    if unsafe { bindings::Save::save_save_kmg(texture.raw_texture(), dst_path.as_ptr()) } {
        Ok(())
    } else {
        Err(Error::save_texture("Failed to save kmg texture."))
    }
}
