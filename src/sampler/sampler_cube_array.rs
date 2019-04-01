
use crate::ffi::root::gli;
use crate::ffi::root::bindings::FSamplerCubeArray as bindings;

use crate::sampler::{Wrap, Filter};
use crate::texture::TextureCubeArray;
use crate::extent::Extent2d;

use std::marker::PhantomData;

/// Cube map array texture sampler.
pub struct FSamplerCubeArray<'a> {

    ffi: gli::fsamplerCubeArray,
    phantom_type: PhantomData<&'a ()>,
}

impl<'a, 'b: 'a> FSamplerCubeArray<'a> {

    /// Constructor of `FSamplerCubeArray`.
    pub fn new(texture: &'b TextureCubeArray, wrap: Wrap, mip: Filter, min: Filter) -> FSamplerCubeArray {
        FSamplerCubeArray {
            ffi: unsafe { bindings::fsampler_cube_array_new(texture.raw_ffi(), wrap.0, mip.0, min.0) },
            phantom_type: PhantomData,
        }
    }

    /// Set the border color used by sampler. Default is `[0, 0, 0, 1]`.
    pub fn set_border_color(&mut self, color: [u32; 4]) {
        unsafe { bindings::fsampler_cube_array_set_border_color(&mut self.ffi, color); }
    }

    /// Clear the sampler texture with a uniform texel.
    pub fn clear(&mut self, texel: [u32; 4]) {
        unsafe { bindings::fsampler_cube_array_clear(&mut self.ffi, texel); }
    }

    /// Generate all the mipmaps of the sampler texture from the texture base level.
    pub fn generate_mipmaps(&mut self, minification: Filter) {
        unsafe { bindings::fsampler_cube_array_generate_mipmaps1(&mut self.ffi, minification.0); }
    }

    /// Generate the mipmaps of the sampler texture from the texture base level to the texture max level included.
    pub fn generate_mipmaps_detail(&mut self, base_layer: usize, max_layer: usize, base_face: usize, max_face: usize, base_level: usize, max_level: usize, minification: Filter) {
        unsafe { bindings::fsampler_cube_array_generate_mipmaps2(&mut self.ffi, base_layer, max_layer, base_face, max_face, base_level, max_level, minification.0); }
    }

    /// Fetch a texel from the sampler texture.
    pub fn texel_fetch(&self, texel_coord: Extent2d, layer: usize, face: usize, level: usize) -> [u32; 4] {
        unsafe { bindings::fsampler_cube_array_texel_fetch(&self.ffi, texel_coord.into(), layer, face, level) }
    }

    /// Write a texel in the sampler texture.
    pub fn texel_write(&mut self, texel_coord: Extent2d, layer: usize, face: usize, level: usize, texel: [u32; 4]) {
        unsafe { bindings::fsampler_cube_array_texel_write(&mut self.ffi, texel_coord.into(), layer, face, level, texel); }
    }

    /// Sample the sampler texture at a specific level.
    pub fn texel_lod(&self, sample_coord: [u32; 2], layer: usize, face: usize, level: usize) -> [u32; 4] {
        unsafe { bindings::fsampler_cube_array_texel_lod(&self.ffi, sample_coord, layer, face, level) }
    }
}
