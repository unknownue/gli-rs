
use crate::ffi::root::gli;
use crate::format::{Format, Swizzle};
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
        Texture3D { ffi: unsafe { gli::texture3d::new() } }
    }

    /// Create a texture3d and allocate a new storage_linear.
    #[inline]
    pub fn new(format: Format, extent: Extent3d, levels: usize) -> Texture3D {
        let default_swizzles = [Swizzle::RED.0, Swizzle::GREEN.0, Swizzle::BLUE.0, Swizzle::ALPHA.0];
        Texture3D { ffi: unsafe { gli::texture3d::new1(format.0, &extent.into(), levels, &default_swizzles) } }
    }

    /// Create a texture3d and allocate a new storage_linear with a complete mipmap chain.
    #[inline]
    pub fn new_with_mipmap_chain(format: Format, extent: Extent3d) -> Texture3D {
        let default_swizzles = [Swizzle::RED.0, Swizzle::GREEN.0, Swizzle::BLUE.0, Swizzle::ALPHA.0];
        Texture3D { ffi: unsafe { gli::texture3d::new2(format.0, &extent.into(), &default_swizzles) } }
    }

    /// Create a texture3d view with an existing storage_linear.
    #[inline]
    pub fn new_from(texture: &impl GliTexture) -> Texture3D {
        Texture3D { ffi: unsafe { gli::texture3d::new3(texture.raw_texture()) } }
    }

    /// Create a texture3d view with an existing storage_linear.
    #[inline]
    pub fn new_detail(texture: &impl GliTexture, format: Format, base_layer: usize, max_layer: usize, base_face: usize, max_face: usize, base_level: usize, max_level: usize) -> Texture3D {
        let default_swizzles = [Swizzle::RED.0, Swizzle::GREEN.0, Swizzle::BLUE.0, Swizzle::ALPHA.0];
        Texture3D { ffi: unsafe { gli::texture3d::new4(texture.raw_texture(), format.0, base_layer, max_layer, base_face, max_face, base_level, max_level, &default_swizzles) } }
    }

    /// Create a texture3d view, reference a subset of an existing texture3d instance.
    #[inline]
    pub fn new_from_subset(texture: &Texture3D, base_level: usize, max_level: usize) -> Texture3D {
        Texture3D { ffi: unsafe { gli::texture3d::new5(&texture.ffi, base_level, max_level) } }
    }

    /// Create a view of the image identified by Level in the mipmap chain of the texture.
    ///
    /// This method is equivalent to `[]` operator in C++ version.
    #[inline]
    pub fn get_level(&self, level: usize) -> GliImage {

        debug_assert!(level < self.levels());
        GliImage::inner_new(self, self.format(), self.base_layer(), self.base_face(), self.base_level() + level)
    }
}

impl GliTexture for Texture3D {
    const TARGET_TYPE: Target = Target::TARGET_3D;
    type ExtentType = Extent3d; // equivalent to gli::texture3d_extent_type.

    /// Return the dimensions of a texture instance: width, height and depth
    fn extent(&self, level: usize) -> Self::ExtentType {
        unsafe { self.ffi.extent(level).into() }
    }
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
