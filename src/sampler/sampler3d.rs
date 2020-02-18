
use crate::ffi::root::gli;
use crate::ffi::root::bindings::FSampler3D as bindings;

use crate::sampler::{Wrap, Filter};
use crate::texture::Texture3D;
use crate::extent::{Extent3d, NormalizeCoord3d};

use std::marker::PhantomData;

/// 3d texture sampler.
///
/// It interprets the texture data as float.
#[repr(transparent)]
pub struct FSampler3D<'a> {

    ffi: gli::fsampler3D,
    phantom_type: PhantomData<&'a ()>,
}

impl<'a, 'b: 'a> FSampler3D<'a> {

    /// Constructor of `FSampler3D`.
    #[cfg(not(target_os = "windows"))]
    pub fn new(texture: &'b Texture3D, wrap: Wrap, mip: Filter, min: Filter) -> FSampler3D {
        FSampler3D {
            ffi: unsafe { bindings::fsampler3d_new(texture.raw_ffi(), wrap.0, mip.0, min.0) },
            phantom_type: PhantomData,
        }
    }

    /// Set the border color used by sampler. Default is `[0.0, 0.0, 0.0, 1.0]`.
    pub fn set_border_color(&mut self, color: [f32; 4]) {
        unsafe { bindings::fsampler3d_set_border_color(&mut self.ffi, color.into()); }
    }

    /// Clear the sampler texture with a uniform texel.
    pub fn clear(&mut self, texel: [f32; 4]) {
        unsafe { bindings::fsampler3d_clear(&mut self.ffi, texel.into()); }
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
    pub fn texel_fetch(&self, texel_coord: Extent3d, level: usize) -> [f32; 4] {
        let raw = unsafe { bindings::fsampler3d_texel_fetch(&self.ffi, texel_coord.into(), level) };
        raw.content
    }

    /// Write a texel in the sampler texture.
    pub fn texel_write(&mut self, texel_coord: Extent3d, level: usize, texel: [f32; 4]) {
        unsafe { bindings::fsampler3d_texel_write(&mut self.ffi, texel_coord.into(), level, texel.into()); }
    }

    /// Sample the sampler texture at a specific level.
    pub fn texel_lod(&self, sample_coord: NormalizeCoord3d, level: f32) -> [f32; 4] {
        let coord: [f32; 3] = sample_coord.into();
        let raw = unsafe { bindings::fsampler3d_texel_lod(&self.ffi, &coord, level) };
        raw.content
    }
}

impl Drop for gli::fsampler3D {

    fn drop(&mut self) {
        unsafe {
            bindings::destroy_sampler3d(self);
        }
    }
}
