
use crate::ffi::root::gli;
use crate::ffi::root::bindings::Texture3D as bindings;

use crate::format::Format;
use crate::target::Target;
use crate::image::GliImage;
use crate::texture::GliTexture;
use crate::texture::inner::TextureAccessible;
use crate::Extent3d;

/// 3d texture
pub struct Texture3D {
    ffi: gli::texture3d,
}

impl Texture3D {

    /// Create an empty texture 3D.
    #[inline]
    pub fn new_empty() -> Texture3D {
        Texture3D { ffi: unsafe { bindings::tex3d_new_empty() } }
    }

    /// Create a texture3d and allocate a new storage_linear.
    #[inline]
    pub fn new(format: Format, extent: Extent3d, levels: usize) -> Texture3D {
        Texture3D { ffi: unsafe { bindings::tex3d_new_(format.0, extent.into(), levels) } }
    }

    /// Create a texture3d and allocate a new storage_linear with a complete mipmap chain.
    #[inline]
    pub fn new_with_mipmap_chain(format: Format, extent: Extent3d) -> Texture3D {
        Texture3D { ffi: unsafe { bindings::tex3d_new_with_mipmap_chain(format.0, extent.into()) } }
    }

    /// Create a texture3d view with an existing storage_linear.
    #[inline]
    pub fn share_from(texture: &impl GliTexture) -> Texture3D {
        Texture3D { ffi: unsafe { bindings::tex3d_share_from(texture.raw_texture()) } }
    }

    /// Create a texture3d view with an existing storage_linear.
    #[inline]
    pub fn share_from_detail(texture: &impl GliTexture, format: Format, base_layer: usize, max_layer: usize, base_face: usize, max_face: usize, base_level: usize, max_level: usize) -> Texture3D {

        Texture3D {
            ffi: unsafe { bindings::tex3d_share_from_detail(texture.raw_texture(), format.0, base_layer, max_layer, base_face, max_face, base_level, max_level) }
        }
    }

    /// Create a texture3d view, reference a subset of an existing texture3d instance.
    #[inline]
    pub fn share_from_subset(texture: &Texture3D, base_level: usize, max_level: usize) -> Texture3D {
        Texture3D { ffi: unsafe { bindings::tex3d_share_from_subset(&texture.ffi, base_level, max_level) } }
    }

    /// Create a view of the image identified by Level in the mipmap chain of the texture.
    ///
    /// This method is equivalent to `[]` operator in C++ version.
    #[inline]
    pub fn get_level(&self, level: usize) -> GliImage {

        debug_assert!(level < self.levels());
        GliImage::shared_from_texture(self, self.format(), self.base_layer(), self.base_face(), self.base_level() + level)
    }

    #[doc(hidden)]
    #[inline]
    pub(crate) fn raw_ffi(&self) -> &gli::texture3d {
        &self.ffi
    }
}

impl GliTexture for Texture3D {
    const TARGET_TYPE: Target = Target::TARGET_3D;
    type ExtentType = Extent3d; // equivalent to gli::texture3d_extent_type.
}

impl TextureAccessible for Texture3D {

    fn raw_texture(&self) -> &gli::texture {
        &self.ffi._base
    }

    fn raw_texture_mut(&mut self) -> &mut gli::texture {
        &mut self.ffi._base
    }
}

impl From<gli::texture> for Texture3D {

    fn from(ffi: gli::texture) -> Texture3D {
        Texture3D { ffi: gli::texture3d { _base: ffi } }
    }
}

impl std::cmp::PartialEq for Texture3D {

    /// Compare two textures. Two textures are the same when the data, the format and the targets are the same.
    fn eq(&self, other: &Texture3D) -> bool {

        use crate::ffi::root::bindings::Comparison::is_texture_equal;

        unsafe {
            is_texture_equal(self.raw_texture(), other.raw_texture())
        }
    }

    /// Compare two textures. Two textures are the same when the data, the format and the targets are the same.
    fn ne(&self, other: &Texture3D) -> bool {

        use crate::ffi::root::bindings::Comparison::is_texture_unequal;

        unsafe {
            is_texture_unequal(self.raw_texture(), other.raw_texture())
        }
    }
}

impl std::cmp::Eq for Texture3D {}
