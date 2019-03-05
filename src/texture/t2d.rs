
use crate::ffi::root::gli;
use crate::format::{Format, Swizzle};
use crate::target::Target;
use crate::image::GliImage;
use crate::texture::GliTexture;
use crate::texture::inner::TextureAccessible;
use crate::Extent2d;

/// 2d texture
pub struct Texture2D {
    ffi: gli::texture2d,
}

impl Texture2D {

    /// Create an empty texture 2D.
    #[inline]
    pub fn new_empty() -> Texture2D {
        Texture2D { ffi: unsafe { gli::texture2d::new() } }
    }

    /// Create a texture2d and allocate a new storage_linear.
    #[inline]
    pub fn new(format: Format, extent: Extent2d, levels: usize) -> Texture2D {
        let default_swizzles = [Swizzle::RED.0, Swizzle::GREEN.0, Swizzle::BLUE.0, Swizzle::ALPHA.0];
        Texture2D { ffi: unsafe { gli::texture2d::new1(format.0, &extent, levels, &default_swizzles) } }
    }

    /// Create a texture2d and allocate a new storage_linear with a complete mipmap chain.
    #[inline]
    pub fn new_with_mipmap_chain(format: Format, extent: Extent2d) -> Texture2D {
        let default_swizzles = [Swizzle::RED.0, Swizzle::GREEN.0, Swizzle::BLUE.0, Swizzle::ALPHA.0];
        Texture2D { ffi: unsafe { gli::texture2d::new2(format.0, &extent, &default_swizzles) } }
    }

    /// Create a texture2d view with an existing storage_linear.
    #[inline]
    pub fn new_from(texture: &Texture2D) -> Texture2D {
        Texture2D { ffi: unsafe { gli::texture2d::new3(&texture.ffi._base) } }
    }

    /// Create a texture2d view with an existing storage_linear.
    #[inline]
    pub fn new_detail(texture: &Texture2D, format: Format, base_layer: usize, max_layer: usize, base_face: usize, max_face: usize, base_level: usize, max_level: usize) -> Texture2D {
        let default_swizzles = [Swizzle::RED.0, Swizzle::GREEN.0, Swizzle::BLUE.0, Swizzle::ALPHA.0];
        Texture2D { ffi: unsafe { gli::texture2d::new4(&texture.ffi._base, format.0, base_layer, max_layer, base_face, max_face, base_level, max_level, &default_swizzles) } }
    }

    /// Create a texture2d view, reference a subset of an existing texture2d instance.
    #[inline]
    pub fn new_from_subset(texture: &Texture2D, base_level: usize, max_level: usize) -> Texture2D {
        Texture2D { ffi: unsafe { gli::texture2d::new5(&texture.ffi, base_level, max_level) } }
    }
}

impl GliTexture for Texture2D {
    const TARGET_TYPE: Target = Target::TARGET_2D;
    type ExtentType = Extent2d; // equivalent to gli::texture2d_extent_type.

    /// Return the dimensions of a texture instance: width and height.
    fn extent(&self, level: usize) -> Self::ExtentType {
        unsafe { self.ffi.extent(level) }
    }
}


// TODO: Impl index operations.
impl ::std::ops::Index<usize> for Texture2D {
    type Output = GliImage;

    fn index(&self, _index: usize) -> &Self::Output {
        unimplemented!("Texture2D index operation")
    }
}

impl TextureAccessible for Texture2D {

    fn raw_texture(&self) -> &gli::texture {
        &self.ffi._base
    }

    fn raw_texture_mut(&mut self) -> &mut gli::texture {
        &mut self.ffi._base
    }
}

impl From<gli::texture> for Texture2D {

    fn from(ffi: gli::texture) -> Texture2D {
        Texture2D { ffi: gli::texture2d { _base: ffi } }
    }
}
