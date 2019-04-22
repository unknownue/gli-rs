
use std::os::raw::c_void;

use crate::ffi::root::gli;
use crate::ffi::root::bindings::Image as bindings;

use crate::format::Format;
use crate::texture::GliTexture;
use crate::Extent3d;

/// GliImage representation for a single texture level.
#[cfg(not(feature = "rc_debug"))]
#[repr(transparent)]
pub struct GliImage {
    ffi: gli::image,
}

#[cfg(feature = "rc_debug")]
#[repr(transparent)]
pub struct GliImage {
    pub ffi: gli::image,
}

impl GliImage {

    /// Create an image object and allocate an image storage for it.
    #[inline]
    pub fn new(format: Format, extent: Extent3d) -> GliImage {
        GliImage { ffi: unsafe { bindings::image_new_(format.0, extent.into()) } }
    }

    /// Create an empty image instance.
    #[inline]
    pub fn new_empty() -> GliImage {
        GliImage { ffi: unsafe { bindings::image_new_empty() } }
    }

    /// Create an image object by sharing an existing image storage_linear from another image instance.
    /// This image object is effectively an image view where format can be reinterpreted with a different compatible image format.
    /// For formats to be compatible, the block size of source and destination must match.
    #[inline]
    pub fn share_from(image: &GliImage, format: Format) -> GliImage {
        GliImage { ffi: unsafe { bindings::image_share_from(&image.ffi, format.0) } }
    }

    /// Clear the entire image storage_linear with zeros.
    #[inline]
    pub fn clear(&mut self) {
        unsafe { bindings::image_clear(&mut self.ffi) }
    }

    // TODO: another clear(..) method is missing, due to template specialization.

    /// Return a pointer to the beginning of the texture instance data.
    #[inline]
    pub fn data(&self) -> *const c_void {
        unsafe { bindings::image_data(&self.ffi) }
    }

    /// Return a mutable pointer to the beginning of the texture instance data.
    #[inline]
    pub unsafe fn data_mut(&mut self) -> *mut c_void {
        bindings::image_data_mut(&mut self.ffi)
    }

    // TODO: another two data(..) methods are missing, due to template specialization.

    /// Return whether the image instance is empty, no storage_linear or description have been assigned to the instance.
    #[inline]
    pub fn empty(&self) -> bool {
        unsafe { bindings::image_empty(&self.ffi) }
    }

    /// Return the dimensions of an image instance: width, height and depth.
    #[inline]
    pub fn extent(&self) -> Extent3d {
        unsafe { bindings::image_extent(&self.ffi).into() }
    }

    /// Return the image instance format.
    #[inline]
    pub fn format(&self) -> Format {
        unsafe { Format(bindings::image_format(&self.ffi)) }
    }

    // TODO: load(..) method is missing, due to template specialization.
    // fn load();

    /// Return the memory size of an image instance storage_linear in bytes.
    #[inline]
    pub fn size(&self) -> usize {
        unsafe { bindings::image_size(&self.ffi) }
    }

    // TODO: another size(&self) method is missing, due to template specialization.

    // TODO: store(..) methods is missing, due to template specialization.

    /// This function is just for inner crate usage. Don't call this function.
    #[inline]
    pub(crate) fn shared_from_texture(texture: &impl GliTexture, format: Format, base_layer: usize, base_face: usize, base_level: usize) -> GliImage {
        GliImage {
            ffi: unsafe { bindings::image_share_from_texture(texture.raw_texture(), format.0, base_layer, base_face, base_level) }
        }
    }
}

impl Drop for gli::image {

    fn drop(&mut self) {

        // Same with gli::texture class, gli::image class in C++ contains member wrapped with std::shared_ptr.
        // Rust can't dual with shared_ptr in ffi.
        // Manually call destructor to decrease its shared_ptr counter.
        unsafe {
            bindings::destroy_image(self)
        }
    }
}

impl std::cmp::PartialEq for GliImage {

    /// Compare two images. Two images are equal when the data is the same.
    fn eq(&self, other: &GliImage) -> bool {

        use crate::ffi::root::bindings::Comparison::is_image_equal;

        unsafe {
            is_image_equal(&self.ffi, &other.ffi)
        }
    }

    /// Compare two images. Two images are equal when the date is the same.
    fn ne(&self, other: &GliImage) -> bool {

        use crate::ffi::root::bindings::Comparison::is_image_unequal;

        unsafe {
            is_image_unequal(&self.ffi, &other.ffi)
        }
    }
}

impl std::cmp::Eq for GliImage {}
