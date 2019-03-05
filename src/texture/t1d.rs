
use crate::ffi::root::gli;
use crate::format::{Format, Swizzle};
use crate::target::Target;
use crate::image::GliImage;
use crate::texture::GliTexture;
use crate::texture::inner::TextureAccess;
use crate::Extent1d;

/// 1d texture
pub struct Texture1D {
    ffi: gli::texture1d,
}

impl Texture1D {

    /// Create an empty texture 1D.
    #[inline]
    pub fn new_empty() -> Texture1D {
        Texture1D { ffi: unsafe { gli::texture1d::new() } }
    }

    /// Create a texture1d and allocate a new storage_linear.
    #[inline]
    pub fn new(format: Format, extent: Extent1d, levels: usize) -> Texture1D {
        let default_swizzles = [Swizzle::RED.0, Swizzle::GREEN.0, Swizzle::BLUE.0, Swizzle::ALPHA.0];
        Texture1D { ffi: unsafe { gli::texture1d::new1(format.0, &extent, levels, &default_swizzles) } }
    }

    /// Create a texture1d and allocate a new storage_linear with a complete mipmap chain.
    #[inline]
    pub fn new_with_mipmap_chain(format: Format, extent: Extent1d) -> Texture1D {
        let default_swizzles = [Swizzle::RED.0, Swizzle::GREEN.0, Swizzle::BLUE.0, Swizzle::ALPHA.0];
        Texture1D { ffi: unsafe { gli::texture1d::new2(format.0, &extent, &default_swizzles) } }
    }

    /// Create a texture1d view with an existing storage_linear.
    #[inline]
    pub fn new_from(texture: &Texture1D) -> Texture1D {
        Texture1D { ffi: unsafe { gli::texture1d::new3(&texture.ffi._base) } }
    }

    /// Create a texture1d view with an existing storage_linear.
    #[inline]
    pub fn new_detail(texture: &Texture1D, format: Format, base_layer: usize, max_layer: usize, base_face: usize, max_face: usize, base_level: usize, max_level: usize) -> Texture1D {
        let default_swizzles = [Swizzle::RED.0, Swizzle::GREEN.0, Swizzle::BLUE.0, Swizzle::ALPHA.0];
        Texture1D { ffi: unsafe { gli::texture1d::new4(&texture.ffi._base, format.0, base_layer, max_layer, base_face, max_face, base_level, max_level, &default_swizzles) } }
    }

    /// Create a texture1d view, reference a subset of an existing texture1d instance.
    #[inline]
    pub fn new_from_subset(texture: &Texture1D, base_level: usize, max_level: usize) -> Texture1D {
        Texture1D { ffi: unsafe { gli::texture1d::new5(&texture.ffi, base_level, max_level) } }
    }
}

// TODO: Impl index operations.
impl ::std::ops::Index<usize> for Texture1D {
    type Output = GliImage;

    fn index(&self, _index: usize) -> &Self::Output {
        unimplemented!("Texture1D index operation")
    }
}

impl GliTexture for Texture1D {
    const TARGET_TYPE: Target = Target::TARGET_1D;
    type ExtentType = Extent1d; // equivalent to gli::texture1d_extent_type.

    fn extent(&self, level: usize) -> Self::ExtentType {
        unsafe { self.ffi.extent(level) }
    }
}

impl TextureAccess for Texture1D {

    fn raw_texture(&self) -> &gli::texture {
        &self.ffi._base
    }

    fn raw_texture_mut(&mut self) -> &mut gli::texture {
        &mut self.ffi._base
    }
}
