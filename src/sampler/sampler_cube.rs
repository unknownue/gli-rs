
use crate::ffi::root::gli;
use crate::ffi::root::bindings::FSamplerCube as bindings;

use crate::sampler::{Wrap, Filter};
use crate::texture::TextureCube;
use crate::extent::Extent2d;

use std::marker::PhantomData;

/// Cube map texture sampler.
pub struct FSamplerCube<'a> {

    ffi: gli::fsamplerCube,
    phantom_type: PhantomData<&'a ()>,
}

impl<'a, 'b: 'a> FSamplerCube<'a> {

    /// Constructor of `FSamplerCube`.
    pub fn new(texture: &'b TextureCube, wrap: Wrap, mip: Filter, min: Filter) -> FSamplerCube {
        FSamplerCube {
            ffi: unsafe { bindings::fsampler_cube_new(texture.raw_ffi(), wrap.0, mip.0, min.0) },
            phantom_type: PhantomData,
        }
    }

    /// Set the border color used by sampler. Default is `[0, 0, 0, 1]`.
    pub fn set_border_color(&mut self, color: [u32; 4]) {
        unsafe { bindings::fsampler_cube_set_border_color(&mut self.ffi, color); }
    }

    /// Clear the sampler texture with a uniform texel.
    pub fn clear(&mut self, texel: [u32; 4]) {
        unsafe { bindings::fsampler_cube_clear(&mut self.ffi, texel); }
    }

    /// Generate all the mipmaps of the sampler texture from the texture base level.
    pub fn generate_mipmaps(&mut self, minification: Filter) {
        unsafe { bindings::fsampler_cube_generate_mipmaps1(&mut self.ffi, minification.0); }
    }

    /// Generate the mipmaps of the sampler texture from the texture base level to the texture max level included.
    pub fn generate_mipmaps_detail(&mut self, base_face: usize, max_face: usize, base_level: usize, max_level: usize, minification: Filter) {
        unsafe { bindings::fsampler_cube_generate_mipmaps2(&mut self.ffi, base_face, max_face, base_level, max_level, minification.0); }
    }

    /// Fetch a texel from the sampler texture.
    pub fn texel_fetch(&self, texel_coord: Extent2d, face: usize, level: usize) -> [u32; 4] {
        unsafe { bindings::fsampler_cube_texel_fetch(&self.ffi, texel_coord.into(), face, level) }
    }

    /// Write a texel in the sampler texture.
    pub fn texel_write(&mut self, texel_coord: Extent2d, face: usize, level: usize, texel: [u32; 4]) {
        unsafe { bindings::fsampler_cube_texel_write(&mut self.ffi, texel_coord.into(), face, level, texel); }
    }

    /// Sample the sampler texture at a specific level.
    pub fn texel_lod(&self, sample_coord: [u32; 2], face: usize, level: usize) -> [u32; 4] {
        unsafe { bindings::fsampler_cube_texel_lod(&self.ffi, sample_coord, face, level) }
    }
}
