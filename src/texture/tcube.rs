
use crate::ffi::root::gli;
use crate::ffi::root::bindings::TextureCube as bindings;

use crate::format::Format;
use crate::target::Target;
use crate::texture::{GliTexture, Texture2D};
use crate::texture::inner::TextureAccessible;
use crate::Extent2d;

/// Cube map texture
pub struct TextureCube {
    ffi: gli::texture_cube,
}

impl TextureCube {

    /// Create an empty texture cube.
    #[inline]
    pub fn new_empty() -> TextureCube {
        TextureCube { ffi: unsafe { bindings::texcube_new_empty() } }
    }

    /// Create a texture_cube.hpp and allocate a new storage_linear.
    #[inline]
    pub fn new(format: Format, extent: Extent2d, levels: usize) -> TextureCube {
        TextureCube { ffi: unsafe { bindings::texcube_new_(format.0, extent.into(), levels) } }
    }

    /// Create a texture_cube.hpp and allocate a new storage_linear with a complete mipmap chain.
    #[inline]
    pub fn new_with_mipmap_chain(format: Format, extent: Extent2d) -> TextureCube {
        TextureCube { ffi: unsafe { bindings::texcube_new_with_mipmap_chain(format.0, extent.into()) } }
    }

    /// Create a texture_cube.hpp view with an existing storage_linear.
    #[inline]
    pub fn new_from(texture: &impl GliTexture) -> TextureCube {
        TextureCube { ffi: unsafe { bindings::texcube_share_from(texture.raw_texture()) } }
    }

    /// Create a texture_cube.hpp view with an existing storage_linear.
    #[inline]
    pub fn new_detail(texture: &impl GliTexture, format: Format, base_layer: usize, max_layer: usize, base_face: usize, max_face: usize, base_level: usize, max_level: usize) -> TextureCube {

        TextureCube {
            ffi: unsafe { bindings::texcube_share_from_detail(texture.raw_texture(), format.0, base_layer, max_layer, base_face, max_face, base_level, max_level) }
        }
    }

    /// Create a texture_cube.hpp view, reference a subset of an existing texture_cube.hpp instance.
    #[inline]
    pub fn new_from_subset(texture: &TextureCube, base_layer: usize, max_layer: usize, base_level: usize, max_level: usize) -> TextureCube {

        TextureCube {
            ffi: unsafe { bindings::texcube_share_from_subset(&texture.ffi, base_layer, max_layer, base_level, max_level) }
        }
    }

    /// Create a view of the texture identified by Face in the texture cube.
    ///
    /// This method is equivalent to `[]` operator in C++ version.
    #[inline]
    pub fn get_face(&self, face: usize) -> Texture2D {

        debug_assert!(face < self.faces());

        Texture2D::share_from_detail(
            self, self.format(),
            self.base_layer(), self.max_layer(),
            self.base_face() + face, self.base_face() + face,
            self.base_level(), self.max_level())
    }

    #[doc(hidden)]
    #[inline]
    pub(crate) fn raw_ffi(&self) -> &gli::texture_cube {
        &self.ffi
    }
}

impl GliTexture for TextureCube {
    const TARGET_TYPE: Target = Target::TARGET_CUBE;
    type ExtentType = Extent2d; // equivalent to gli::texture2d_extent_type.
}

impl TextureAccessible for TextureCube {

    fn raw_texture(&self) -> &gli::texture {
        &self.ffi._base
    }

    fn raw_texture_mut(&mut self) -> &mut gli::texture {
        &mut self.ffi._base
    }
}

impl From<gli::texture> for TextureCube {

    fn from(ffi: gli::texture) -> TextureCube {
        TextureCube { ffi: gli::texture_cube { _base: ffi } }
    }
}

impl std::cmp::PartialEq for TextureCube {

    /// Compare two textures. Two textures are the same when the data, the format and the targets are the same.
    fn eq(&self, other: &TextureCube) -> bool {

        use crate::ffi::root::bindings::Comparison::is_texture_equal;

        unsafe {
            is_texture_equal(self.raw_texture(), other.raw_texture())
        }
    }

    /// Compare two textures. Two textures are the same when the data, the format and the targets are the same.
    fn ne(&self, other: &TextureCube) -> bool {

        use crate::ffi::root::bindings::Comparison::is_texture_unequal;

        unsafe {
            is_texture_unequal(self.raw_texture(), other.raw_texture())
        }
    }
}

impl std::cmp::Eq for TextureCube {}
