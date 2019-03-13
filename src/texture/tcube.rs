
use crate::ffi::root::gli;
use crate::format::{Format, Swizzle};
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
        TextureCube { ffi: unsafe { gli::texture_cube::new() } }
    }

    /// Create a texture_cube.hpp and allocate a new storage_linear.
    #[inline]
    pub fn new(format: Format, extent: Extent2d, levels: usize) -> TextureCube {
        let default_swizzles = [Swizzle::RED.0, Swizzle::GREEN.0, Swizzle::BLUE.0, Swizzle::ALPHA.0];
        TextureCube { ffi: unsafe { gli::texture_cube::new1(format.0, &extent.into(), levels, &default_swizzles) } }
    }

    /// Create a texture_cube.hpp and allocate a new storage_linear with a complete mipmap chain.
    #[inline]
    pub fn new_with_mipmap_chain(format: Format, extent: Extent2d) -> TextureCube {
        let default_swizzles = [Swizzle::RED.0, Swizzle::GREEN.0, Swizzle::BLUE.0, Swizzle::ALPHA.0];
        TextureCube { ffi: unsafe { gli::texture_cube::new2(format.0, &extent.into(), &default_swizzles) } }
    }

    /// Create a texture_cube.hpp view with an existing storage_linear.
    #[inline]
    pub fn new_from(texture: &impl GliTexture) -> TextureCube {
        TextureCube { ffi: unsafe { gli::texture_cube::new3(texture.raw_texture()) } }
    }

    /// Create a texture_cube.hpp view with an existing storage_linear.
    #[inline]
    pub fn new_detail(texture: &impl GliTexture, format: Format, base_layer: usize, max_layer: usize, base_face: usize, max_face: usize, base_level: usize, max_level: usize) -> TextureCube {
        let default_swizzles = [Swizzle::RED.0, Swizzle::GREEN.0, Swizzle::BLUE.0, Swizzle::ALPHA.0];
        TextureCube { ffi: unsafe { gli::texture_cube::new4(texture.raw_texture(), format.0, base_layer, max_layer, base_face, max_face, base_level, max_level, &default_swizzles) } }
    }

    /// Create a texture_cube.hpp view, reference a subset of an existing texture_cube.hpp instance.
    #[inline]
    pub fn new_from_subset(texture: &TextureCube, base_layer: usize, max_layer: usize, base_level: usize, max_level: usize) -> TextureCube {
        TextureCube { ffi: unsafe { gli::texture_cube::new5(&texture.ffi, base_layer, max_layer, base_level, max_level) } }
    }

    /// Create a view of the texture identified by Face in the texture cube.
    ///
    /// This method is equivalent to `[]` operator in C++ version.
    #[inline]
    pub fn get_face(&self, face: usize) -> Texture2D {

        debug_assert!(face < self.faces());

        Texture2D::new_detail(
            self, self.format(),
            self.base_layer(), self.max_layer(),
            self.base_face() + face, self.base_face() + face,
            self.base_level(), self.max_level())
    }
}

impl GliTexture for TextureCube {
    const TARGET_TYPE: Target = Target::TARGET_CUBE;
    type ExtentType = Extent2d; // equivalent to gli::texture2d_extent_type.

    /// Return the dimensions of a texture instance: width and height where both should be equal.
    fn extent(&self, level: usize) -> Self::ExtentType {
        unsafe { self.ffi.extent(level).into() }
    }
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
