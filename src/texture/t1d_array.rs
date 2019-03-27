
use crate::ffi::root::gli;
use crate::ffi::root::bindings::Texture1DArray as bindings;

use crate::format::Format;
use crate::target::Target;
use crate::texture::{GliTexture, Texture1D};
use crate::texture::inner::TextureAccessible;
use crate::Extent1d;

/// 1d array texture
pub struct Texture1DArray {
    ffi: gli::texture1d_array,
}

impl Texture1DArray {

    /// Create an empty texture 1D array.
    #[inline]
    pub fn new_empty() -> Texture1DArray {
        Texture1DArray { ffi: unsafe { bindings::tex1darray_new_empty() } }
    }

    /// Create a texture1d_array and allocate a new storage_linear.
    #[inline]
    pub fn new(format: Format, extent: Extent1d, layers: usize, levels: usize) -> Texture1DArray {
        Texture1DArray { ffi: unsafe { bindings::tex1darray_new_(format.0, extent.into(), layers, levels) } }
    }

    /// Create a texture1d_array and allocate a new storage_linear with a complete mipmap chain.
    #[inline]
    pub fn new_with_mipmap_chain(format: Format, extent: Extent1d, layers: usize) -> Texture1DArray {
        Texture1DArray { ffi: unsafe { bindings::tex1darray_new_with_mipmap_chain(format.0, extent.into(), layers) } }
    }

    /// Create a texture1d_array view with an existing storage_linear.
    #[inline]
    pub fn share_from(texture: &impl GliTexture) -> Texture1DArray {
        Texture1DArray { ffi: unsafe { bindings::tex1darray_share_from(texture.raw_texture()) } }
    }

    /// Create a texture1d_array view with an existing storage_linear.
    #[inline]
    pub fn share_from_detail(texture: &impl GliTexture, format: Format, base_layer: usize, max_layer: usize, base_face: usize, max_face: usize, base_level: usize, max_level: usize) -> Texture1DArray {

        Texture1DArray {
            ffi: unsafe { bindings::tex1darray_share_from_detail(texture.raw_texture(), format.0, base_layer, max_layer, base_face, max_face, base_level, max_level) }
        }
    }

    /// Create a texture1d_array view, reference a subset of an existing texture1d_array instance.
    #[inline]
    pub fn share_from_subset(texture: &Texture1DArray, base_layer: usize, max_layer: usize, base_level: usize, max_level: usize) -> Texture1DArray {

        Texture1DArray {
            ffi: unsafe { bindings::tex1darray_share_from_subset(&texture.ffi, base_layer, max_layer, base_level, max_level) }
        }
    }

   /// Create a view of the texture identified by Layer in the texture array.
   ///
   /// This method is equivalent to `[]` operator in C++ version.
    #[inline]
    pub fn get_layer(&self, layer: usize) -> Texture1D {

       debug_assert!(self.empty());
       debug_assert!(layer < self.layers());

       Texture1D::share_from_detail(
           self, self.format(),
           self.base_layer() + layer, self.base_layer() + layer,
           self.base_face(), self.max_face(),
           self.base_level(), self.max_level())
   }
}

impl GliTexture for Texture1DArray {
    const TARGET_TYPE: Target = Target::TARGET_1D_ARRAY;
    type ExtentType = Extent1d; // equivalent to gli::texture1d_extent_type.
}

impl TextureAccessible for Texture1DArray {

    fn raw_texture(&self) -> &gli::texture {
        &self.ffi._base
    }

    fn raw_texture_mut(&mut self) -> &mut gli::texture {
        &mut self.ffi._base
    }
}

impl From<gli::texture> for Texture1DArray {

    fn from(ffi: gli::texture) -> Texture1DArray {
        Texture1DArray { ffi: gli::texture1d_array { _base: ffi } }
    }
}

impl std::cmp::PartialEq for Texture1DArray {

    /// Compare two textures. Two textures are the same when the data, the format and the targets are the same.
    fn eq(&self, other: &Texture1DArray) -> bool {

        use crate::ffi::root::bindings::Comparison::is_texture_equal;

        unsafe {
            is_texture_equal(self.raw_texture(), other.raw_texture())
        }
    }

    /// Compare two textures. Two textures are the same when the data, the format and the targets are the same.
    fn ne(&self, other: &Texture1DArray) -> bool {

        use crate::ffi::root::bindings::Comparison::is_texture_unequal;

        unsafe {
            is_texture_unequal(self.raw_texture(), other.raw_texture())
        }
    }
}

impl std::cmp::Eq for Texture1DArray {}
