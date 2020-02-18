
use crate::ffi::root::gli;
use crate::ffi::root::bindings::FSamplerCubeArray as bindings;

use crate::sampler::{Wrap, Filter};
use crate::texture::TextureCubeArray;
use crate::extent::{Extent2d, NormalizeCoord2d};

use std::marker::PhantomData;

/// Cube map array texture sampler.
///
/// It interprets the texture data as float.
#[repr(transparent)]
pub struct FSamplerCubeArray<'a> {

    ffi: gli::fsamplerCubeArray,
    phantom_type: PhantomData<&'a ()>,
}

impl<'a, 'b: 'a> FSamplerCubeArray<'a> {

    /// Constructor of `FSamplerCubeArray`.
    #[cfg(not(target_os = "windows"))]
    pub fn new(texture: &'b TextureCubeArray, wrap: Wrap, mip: Filter, min: Filter) -> FSamplerCubeArray {
        FSamplerCubeArray {
            ffi: unsafe { bindings::fsampler_cube_array_new(texture.raw_ffi(), wrap.0, mip.0, min.0) },
            phantom_type: PhantomData,
        }
    }

    /// Set the border color used by sampler. Default is `[0.0, 0.0, 0.0, 1.0]`.
    pub fn set_border_color(&mut self, color: [f32; 4]) {
        unsafe { bindings::fsampler_cube_array_set_border_color(&mut self.ffi, color.into()); }
    }

    /// Clear the sampler texture with a uniform texel.
    pub fn clear(&mut self, texel: [f32; 4]) {
        unsafe { bindings::fsampler_cube_array_clear(&mut self.ffi, texel.into()); }
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
    pub fn texel_fetch(&self, texel_coord: Extent2d, layer: usize, face: usize, level: usize) -> [f32; 4] {
        let raw = unsafe { bindings::fsampler_cube_array_texel_fetch(&self.ffi, texel_coord.into(), layer, face, level) };
        raw.content
    }

    /// Write a texel in the sampler texture.
    pub fn texel_write(&mut self, texel_coord: Extent2d, layer: usize, face: usize, level: usize, texel: [f32; 4]) {
        unsafe { bindings::fsampler_cube_array_texel_write(&mut self.ffi, texel_coord.into(), layer, face, level, texel.into()); }
    }

    /// Sample the sampler texture at a specific level.
    pub fn texel_lod(&self, sample_coord: NormalizeCoord2d, layer: usize, face: usize, level: f32) -> [f32; 4] {
        let coord: [f32; 2] = sample_coord.into();
        let raw = unsafe { bindings::fsampler_cube_array_texel_lod(&self.ffi, &coord, layer, face, level) };
        raw.content
    }
}

impl Drop for gli::fsamplerCubeArray {

    fn drop(&mut self) {
        unsafe {
            bindings::destroy_sampler_cube_array(self);
        }
    }
}
