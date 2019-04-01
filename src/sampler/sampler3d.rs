
use crate::ffi::root::gli;
use crate::ffi::root::bindings::FSampler3D as bindings;

use crate::sampler::{Wrap, Filter};
use crate::texture::Texture3D;
use crate::extent::Extent3d;

use std::marker::PhantomData;

/// 3d texture sampler.
pub struct FSampler3D<'a> {

    ffi: gli::fsampler3D,
    phantom_type: PhantomData<&'a ()>,
}

impl<'a, 'b: 'a> FSampler3D<'a> {

    /// Constructor of `FSampler3D`.
    pub fn new(texture: &'b Texture3D, wrap: Wrap, mip: Filter, min: Filter) -> FSampler3D {
        FSampler3D {
            ffi: unsafe { bindings::fsampler3d_new(texture.raw_ffi(), wrap.0, mip.0, min.0) },
            phantom_type: PhantomData,
        }
    }

    /// Set the border color used by sampler. Default is `[0, 0, 0, 1]`.
    pub fn set_border_color(&mut self, color: [u32; 4]) {
        unsafe { bindings::fsampler3d_set_border_color(&mut self.ffi, color); }
    }

    /// Clear the sampler texture with a uniform texel.
    pub fn clear(&mut self, texel: [u32; 4]) {
        unsafe { bindings::fsampler3d_clear(&mut self.ffi, texel); }
    }

    /// Generate all the mipmaps of the sampler texture from the texture base level.
    pub fn generate_mipmaps(&mut self, minification: Filter) {
        unsafe { bindings::fsampler3d_generate_mipmaps1(&mut self.ffi, minification.0); }
    }

    /// Generate the mipmaps of the sampler texture from the texture base level to the texture max level included.
    pub fn generate_mipmaps_detail(&mut self, base_level: usize, max_level: usize, minification: Filter) {
        unsafe { bindings::fsampler3d_generate_mipmaps3(&mut self.ffi, base_level, max_level, minification.0); }
    }

    /// Fetch a texel from the sampler texture.
    pub fn texel_fetch(&self, texel_coord: Extent3d, level: usize) -> [u32; 4] {
        unsafe { bindings::fsampler3d_texel_fetch(&self.ffi, texel_coord.into(), level) }
    }

    /// Write a texel in the sampler texture.
    pub fn texel_write(&mut self, texel_coord: Extent3d, level: usize, texel: [u32; 4]) {
        unsafe { bindings::fsampler3d_texel_write(&mut self.ffi, texel_coord.into(), level, texel); }
    }

    /// Sample the sampler texture at a specific level.
    pub fn texel_lod(&self, sample_coord: [u32; 3], level: usize) -> [u32; 4] {
        unsafe { bindings::fsampler3d_texel_lod(&self.ffi, sample_coord, level) }
    }
}
