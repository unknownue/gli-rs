
use crate::ffi::root::gli;
use crate::ffi::root::bindings::FSampler1D as bindings;

use crate::sampler::{Wrap, Filter};
use crate::texture::Texture1D;
use crate::extent::Extent1d;

use std::marker::PhantomData;

pub struct FSampler1D<'a> {

    ffi: gli::fsampler1D,
    phantom_type: PhantomData<&'a ()>,
}

impl<'a, 'b: 'a> FSampler1D<'a> {

    pub fn new(texture: &'b Texture1D, wrap: Wrap, mip: Filter, min: Filter) -> FSampler1D {
        FSampler1D {
            ffi: unsafe { bindings::fsampler1d_new(texture.raw_ffi(), wrap.0, mip.0, min.0) },
            phantom_type: PhantomData,
        }
    }

    pub fn set_border_color(&mut self, color: [u32; 4]) {
        unsafe { bindings::fsampler1d_set_border_color(&mut self.ffi, color); }
    }

    /// Clear the sampler texture with a uniform texel.
    pub fn clear(&mut self, texel: [u32; 4]) {
        unsafe { bindings::fsampler1d_clear(&mut self.ffi, texel); }
    }

    /// Generate all the mipmaps of the sampler texture from the texture base level.
    pub fn generate_mipmaps(&mut self, minification: Filter) {
        unsafe { bindings::fsampler1d_generate_mipmaps1(&mut self.ffi, minification.0); }
    }

    /// Generate the mipmaps of the sampler texture from the texture base level to the texture max level included.
    pub fn generate_mipmaps_detail(&mut self, base_level: usize, max_level: usize, minification: Filter) {
        unsafe { bindings::fsampler1d_generate_mipmaps2(&mut self.ffi, base_level, max_level, minification.0); }
    }

    /// Fetch a texel from the sampler texture.
    pub fn texel_fetch(&self, texel_coord: Extent1d, level: usize) -> [u32; 4] {
        unsafe { bindings::fsampler1d_texel_fetch(&self.ffi, texel_coord.width, level) }
    }

    /// Write a texel in the sampler texture.
    pub fn texel_write(&mut self, texel_coord: Extent1d, level: usize, texel: [u32; 4]) {
        unsafe { bindings::fsampler1d_texel_write(&mut self.ffi, texel_coord.width, level, texel); }
    }

    /// Sample the sampler texture at a specific level.
    pub fn texel_lod(&self, sampler_coord: u32, level: usize) -> [u32; 4] {
        unsafe { bindings::fsampler1d_texel_lod(&self.ffi, sampler_coord, level) }
    }
}
