
use std::os::raw::c_void;

use crate::ffi::root::gli;
use crate::format::Format;
use crate::Extent3d;
use crate::texture::GliTexture;

/// GliImage representation for a single texture level.
pub struct GliImage {
    ffi: gli::image,
}

impl GliImage {

    /// Create an image object and allocate an image storage for it.
    #[inline]
    pub fn new(format: Format, extent: Extent3d) -> GliImage {
        GliImage { ffi: unsafe { gli::image::new1(format.0, &extent.into()) } }
    }

    /// Create an empty image instance.
    #[inline]
    pub fn new_empty() -> GliImage {
        GliImage { ffi: unsafe { gli::image::new() } }
    }

    /// Create an image object by sharing an existing image storage_linear from another image instance.
    /// This image object is effectively an image view where format can be reinterpreted with a different compatible image format.
    /// For formats to be compatible, the block size of source and destination must match.
    #[inline]
    pub fn new_from(image: &GliImage, format: Format) -> GliImage {
        GliImage { ffi: unsafe { gli::image::new2(&image.ffi, format.0) } }
    }

    /// Clear the entire image storage_linear with zeros.
    #[inline]
    pub fn clear(&mut self) {
        unsafe { self.ffi.clear() }
    }

    // TODO: another clear(..) method is missing, due to template specialization.

    /// Return a pointer to the beginning of the texture instance data.
    #[inline]
    pub fn data(&self) -> *const c_void {
        unsafe { self.ffi.data1() }
    }

    /// Return a mutable pointer to the beginning of the texture instance data.
    #[inline]
    pub fn data_mut(&mut self) -> *mut c_void {
        unsafe { self.ffi.data() }
    }

    // TODO: another two data(..) methods are missing, due to template specialization.

    /// Return whether the image instance is empty, no storage_linear or description have been assigned to the instance.
    #[inline]
    pub fn empty(&self) -> bool {
        unsafe { self.ffi.empty() }
    }

    /// Return the dimensions of an image instance: width, height and depth.
    #[inline]
    pub fn extent(&self) -> Extent3d {
        unsafe { self.ffi.extent().into() }
    }

    /// Return the image instance format.
    #[inline]
    pub fn format(&self) -> Format {
        unsafe { Format(self.ffi.format()) }
    }

    // TODO: load(..) method is missing, due to template specialization.
    // fn load();

    /// Return the memory size of an image instance storage_linear in bytes.
    #[inline]
    pub fn size(&self) -> usize {
        unsafe { self.ffi.size() }
    }

    // TODO: another size(&self) method is missing, due to template specialization.

    // TODO: store(..) methods is missing, due to template specialization.

    /// This function is just for inner crate usage. Don't call this function.
    #[doc(hidden)]
    #[inline]
    pub(crate) fn inner_new(texture: &impl GliTexture, format: Format, base_layer: usize, base_face: usize, base_level: usize) -> GliImage {
        GliImage { ffi: unsafe { gli::image::new3(texture.raw_texture(), format.0, base_layer, base_face, base_level) } }
    }
}

impl Drop for gli::image {

    fn drop(&mut self) {

        // Same with gli::texture class, gli::image class in C++ contains member wrapped with std::shared_ptr.
        // Rust can't dual with shared_ptr in ffi.
        // Manually call destructor to decrease its shared_ptr counter.
        unsafe {
            gli::destroy_image(self)
        }
    }
}
